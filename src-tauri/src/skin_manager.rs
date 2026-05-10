use std::fs;
use std::path::{Path, PathBuf};

use crate::skin::SkinConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkinInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub valid: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PetSettings {
    pub current_skin_id: String,
    pub window_x: i32,
    pub window_y: i32,
}

impl Default for PetSettings {
    fn default() -> Self {
        Self {
            current_skin_id: "default".into(),
            window_x: 100,
            window_y: 100,
        }
    }
}

pub fn list_skins(skins_base_dir: &Path) -> Vec<SkinInfo> {
    let mut skins = Vec::new();

    if let Ok(entries) = fs::read_dir(skins_base_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let skin_json = entry.path().join("skin.json");
                    if skin_json.exists() {
                        let id = entry.file_name().to_string_lossy().to_string();

                        // 只读取一次，同时提取 name 和 valid
                        let (name, valid) = match fs::read_to_string(&skin_json) {
                            Ok(content) => {
                                match serde_json::from_str::<SkinConfig>(&content) {
                                    Ok(skin) => (skin.name, true),
                                    Err(_) => (id.clone(), false),
                                }
                            }
                            Err(_) => (id.clone(), false),
                        };

                        skins.push(SkinInfo {
                            path: format!("assets/skins/{}", id),
                            id,
                            name,
                            valid,
                        });
                    }
                }
            }
        }
    }

    skins
}

pub fn settings_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("settings.json")
}

pub fn load_settings(app_data_dir: &Path) -> PetSettings {
    let path = settings_path(app_data_dir);
    fs::read_to_string(&path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

pub fn save_settings(app_data_dir: &Path, settings: &PetSettings) -> Result<(), String> {
    let path = settings_path(app_data_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let s = PetSettings::default();
        assert_eq!(s.current_skin_id, "default");
        assert_eq!(s.window_x, 100);
        assert_eq!(s.window_y, 100);
    }

    #[test]
    fn test_settings_roundtrip() {
        let s = PetSettings {
            current_skin_id: "custom".into(),
            window_x: 200,
            window_y: 300,
        };
        let json = serde_json::to_string(&s).unwrap();
        let parsed: PetSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.current_skin_id, "custom");
        assert_eq!(parsed.window_x, 200);
        assert_eq!(parsed.window_y, 300);
    }

    #[test]
    fn test_settings_path() {
        let dir = std::path::Path::new("/tmp/app-data");
        let path = settings_path(dir);
        assert_eq!(path, dir.join("settings.json"));
    }

    #[test]
    fn test_list_skins_empty_dir() {
        let tmp = std::env::temp_dir().join("test_skins_empty_12345");
        std::fs::create_dir_all(&tmp).unwrap();
        let skins = list_skins(&tmp);
        assert!(skins.is_empty());
        std::fs::remove_dir_all(&tmp).unwrap();
    }
}