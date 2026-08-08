# gpkg-manager

[English](#english) | [Русский](#русский)

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

### 📦 Building & Installation
```bash
# Clone the repository
cd gpkg-manager

# Build in release mode
cargo build --release

# Run the application
./target/release/gpkg-manager
