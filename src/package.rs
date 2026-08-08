use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use tar::Archive;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub fn inspect_gpkg(path: &str) -> Result<(PackageMeta, Vec<String>)> {
    let mut file = File::open(path).context("Failed to open package file")?;
    
    let mut header = [0u8; 4];
    let _ = file.read(&mut header);
    let _ = file.seek(SeekFrom::Start(0));

    let boxed_reader: Box<dyn Read + Send> = if header[0] == 0x1f && header[1] == 0x8b {
        Box::new(flate2::read::GzDecoder::new(file))
    } else if header[0] == 0x28 && header[1] == 0xb5 {
        Box::new(zstd::Decoder::new(file).context("Zstd error")?)
    } else {
        let file_clone = file.try_clone()?;
        if let Ok(decoder) = zstd::Decoder::new(file_clone) {
            Box::new(decoder)
        } else {
            Box::new(file)
        }
    };

    let mut archive = Archive::new(boxed_reader);
    let mut meta = PackageMeta::default();
    let mut files = Vec::new();

    if let Ok(entries) = archive.entries() {
        for entry in entries.flatten() {
            if let Ok(p) = entry.path() {
                let path_str = p.to_string_lossy().to_string();
                files.push(path_str.clone());

                if path_str.ends_with("meta.json") || path_str.ends_with("info.json") || path_str.ends_with("package.json") || path_str.ends_with("manifest.json") {
                    let mut mut_entry = entry;
                    let mut content = String::new();
                    if mut_entry.read_to_string(&mut content).is_ok() {
                        if let Ok(parsed) = serde_json::from_str::<PackageMeta>(&content) {
                            meta = parsed;
                        }
                    }
                }
            }
        }
    }

    if meta.name.is_empty() {
        let file_name = Path::new(path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "package".to_string());

        let parts: Vec<&str> = file_name.split('-').collect();
        if parts.len() >= 2 {
            meta.name = parts[0..parts.len()-1].join("-");
            meta.version = parts.last().unwrap_or(&"1.0.0").to_string();
        } else {
            meta.name = file_name;
            meta.version = "1.0.0".to_string();
        }
        meta.description = "Пакет G OS (Без метаданных)".to_string();
    }
    
    Ok((meta, files))
}