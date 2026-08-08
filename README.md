# gpkg-manager

---

<a name="english"></a>
## 🇬🇧 English

`gpkg-manager` is a lightweight, ultra-fast native GUI application designed for managing `.gpkg` software packages on **G OS**. It acts as a modern graphical wrapper around the native `gvalli` CLI package manager.

### 🚀 Key Features
* **Modern & Native UI:** Built with Slint UI, ensuring GPU-accelerated performance and low resource consumption.
* **Non-Blocking Operations:** Uses a dedicated background Tokio runtime and channels to keep the UI completely fluid during heavy archive parsing and root operations (`pkexec`).
* **Smart Archive Inspection:** Automatically detects compression algorithms (Zstd, Gzip, or raw Tar) and extracts package metadata (`manifest.json`, `meta.json`, etc.) instantly without unpacking the payload.
* **Integrated Package Management:** Seamlessly switch between installing local packages and browsing/removing already installed system software.

### 🛠️ Tech Stack
* **Language:** Rust (Edition 2021)
* **GUI Framework:** Slint UI (`slint` crate)
* **Async Runtime:** Tokio
* **Archive & Meta:** `tar`, `zstd`, `flate2`, `serde`, `serde_json`
* **Dialogs:** `rfd` (Rusty File Dialogs)

## 🇬🇧 Русский

`gpkg-manager` — это легковесное, ультрабыстрое нативное графическое приложение для управления программными пакетами `.gpkg` в операционной системе **G OS**. Оно служит современным графическим интерфейсом-оберткой над консольным пакетным менеджером `gvalli`.

## 🚀 Основные возможности
* **Современный нативный интерфейс:** Написан на Slint UI, что обеспечивает аппаратное ускорение графики и минимальное потребление системных ресурсов.
* **Асинхронность без фризов:** Использование фонового рантайма Tokio и каналов предотвращает зависание интерфейса во время распаковки архивов и выполнения привилегированных команд через `pkexec`.
* **Умный анализ архивов:** Автоматическое определение алгоритмов сжатия (Zstd, Gzip или чистый Tar) и мгновенное извлечение метаданных (`manifest.json`, `meta.json` и др.) без полной распаковки пакета.
* **Управление пакетами:** Удобное переключение между вкладкой установки новых локальных пакетов и списком уже установленного софта с возможностью его удаления.

## 🛠️ Технологический стек
* **Язык:** Rust (Edition 2021)
* **GUI Фреймворк:** Slint UI (`slint`)
* **Асинхронность:** Tokio
* **Работа с архивами:** `tar`, `zstd`, `flate2`, `serde`, `serde_json`
* **Диалоги:** `rfd` (Rusty File Dialogs)
