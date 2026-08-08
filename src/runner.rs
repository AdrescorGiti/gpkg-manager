use slint::{ComponentHandle, SharedString, Weak};
use std::process::Stdio;
use tokio::process::Command;
use crate::AppWindow;

pub async fn run_gvalli_command(args: Vec<&str>, window_weak: Weak<AppWindow>) {
    let mut cmd = Command::new("pkexec");
    cmd.arg("gvalli").args(&args);
    
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut is_success = false;

    let msg = match cmd.output().await {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).replace('\0', "");
            let stderr_raw = String::from_utf8_lossy(&out.stderr).replace('\0', "");
            
            // Фильтруем баг самого gvalli/pkexec, удаляя строки с GLib-CRITICAL
            let stderr_clean: Vec<&str> = stderr_raw
                .lines()
                .filter(|line| !line.contains("GLib-CRITICAL"))
                .collect();
            let stderr = stderr_clean.join("\n");

            let exit_code = out.status.code().unwrap_or(-1);
            is_success = out.status.success();

            if is_success {
                format!("Успешно завершено!\nКод выхода: {}\n\n--- STDOUT ---\n{}\n--- STDERR ---\n{}", exit_code, stdout, stderr)
            } else {
                format!("ОШИБКА ВЫПОЛНЕНИЯ!\nКод выхода: {}\n\n--- STDOUT ---\n{}\n--- STDERR ---\n{}", exit_code, stdout, stderr)
            }
        },
        Err(e) => format!("Критическая ошибка запуска процесса pkexec: {}", e),
    };

    let _ = window_weak.upgrade_in_event_loop(move |app| {
        let logic = app.global::<crate::AppLogic>();
        logic.set_terminal_output(SharedString::from(msg));
        logic.set_is_processing(false);
        
        // Если пакет успешно установлен или удален - сразу обновляем список установленных
        if is_success {
            logic.invoke_load_installed_packages();
        }
    });
}