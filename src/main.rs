mod package;
mod runner;

slint::include_modules!();

use rfd::FileDialog;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;
use std::thread;

fn main() -> anyhow::Result<()> {
    let app = AppWindow::new()?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<String>>();
    
    let app_weak_for_tokio = app.as_weak();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            while let Some(args) = rx.recv().await {
                let args_refs: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
                runner::run_gvalli_command(args_refs, app_weak_for_tokio.clone()).await;
            }
        });
    });

    // Обработчик загрузки установленных пакетов
    let app_weak = app.as_weak();
    app.global::<AppLogic>().on_load_installed_packages(move || {
        let app_weak_clone = app_weak.clone();
        thread::spawn(move || {
            let output = std::process::Command::new("gvalli").arg("list").output();
            let mut packages = Vec::new();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.contains("GLib-CRITICAL") { continue; }
                    
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if !parts.is_empty() {
                        packages.push(crate::InstalledPackage {
                            name: SharedString::from(parts[0]),
                            version: SharedString::from(if parts.len() > 1 { parts[1] } else { "1.0.0" }),
                            description: SharedString::from("Установлено в G OS"),
                        });
                    }
                }
            }

            if packages.is_empty() {
                packages.push(crate::InstalledPackage {
                    name: SharedString::from("Список пуст"),
                    version: SharedString::from("-"),
                    description: SharedString::from("Не удалось получить пакеты через 'gvalli list'"),
                });
            }

            let _ = app_weak_clone.upgrade_in_event_loop(move |app| {
                let model = Rc::new(VecModel::from(packages));
                app.global::<AppLogic>().set_installed_packages(ModelRc::from(model));
            });
        });
    });

    app.global::<AppLogic>().invoke_load_installed_packages();

    // Выбор файла
    let app_weak = app.as_weak();
    app.global::<AppLogic>().on_open_file_dialog(move || {
        let app = app_weak.unwrap();
        if let Some(path) = FileDialog::new().add_filter("G OS Package", &["gpkg"]).pick_file() {
            let path_str = path.to_string_lossy().to_string();
            let logic = app.global::<AppLogic>();
            
            logic.set_selected_file(SharedString::from(&path_str));
            logic.set_pkg_name(SharedString::from("Загрузка..."));
            logic.set_pkg_version(SharedString::from(""));
            logic.set_pkg_description(SharedString::from("Анализ архива..."));
            logic.set_archive_files(ModelRc::from(Rc::new(VecModel::from(vec![]))));
            logic.set_terminal_output(SharedString::from(""));

            let app_weak_clone = app_weak.clone();
            thread::spawn(move || {
                let (meta, files) = package::inspect_gpkg(&path_str).unwrap_or_else(|_| {
                    (
                        package::PackageMeta {
                            name: "gpkg-package".to_string(),
                            version: "1.0.0".to_string(),
                            description: "Пакет готов к установке через gvalli".to_string(),
                        },
                        vec!["Бинарный или защищенный формат архива".to_string()],
                    )
                });

                let _ = app_weak_clone.upgrade_in_event_loop(move |app| {
                    let logic = app.global::<AppLogic>();
                    logic.set_pkg_name(SharedString::from(meta.name));
                    logic.set_pkg_version(SharedString::from(meta.version));
                    logic.set_pkg_description(SharedString::from(meta.description));

                    let file_model = Rc::new(VecModel::from(
                        files.into_iter().map(SharedString::from).collect::<Vec<_>>()
                    ));
                    logic.set_archive_files(ModelRc::from(file_model));
                });
            });
        }
    });

    // Установка пакета
    let app_weak = app.as_weak();
    let tx_clone_install = tx.clone();
    app.global::<AppLogic>().on_install_package(move |file_path| {
        let app = app_weak.unwrap();
        app.global::<AppLogic>().set_is_processing(true);
        app.global::<AppLogic>().set_terminal_output(SharedString::from("Запуск установки..."));

        // ИСПРАВЛЕНИЕ: Используем `gpkg install` для локальных файлов. 
        // (Если в консоли ты пишешь иначе, просто поменяй эти строки)
        let args = vec!["gpkg".to_string(), "install".to_string(), file_path.to_string()];
        let _ = tx_clone_install.send(args);
    });

    // Удаление пакета
    let app_weak = app.as_weak();
    let tx_clone_remove = tx.clone();
    app.global::<AppLogic>().on_remove_package(move |pkg_name| {
        let app = app_weak.unwrap();
        app.global::<AppLogic>().set_is_processing(true);
        app.global::<AppLogic>().set_terminal_output(SharedString::from("Запуск удаления..."));

        let args = vec!["remove".to_string(), pkg_name.to_string()];
        let _ = tx_clone_remove.send(args);
    });

    app.run()?;
    Ok(())
}