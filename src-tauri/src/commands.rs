use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tauri::{Manager, State};

use crate::skin::SkinConfig;
use crate::skin_manager::{self, PetSettings, SkinInfo};
use crate::skin_validator::{validate_skin, ValidationResult};
use crate::state_machine::PetStateMachine;

// ── Path safety ──────────────────────────────────────────────────

fn safe_path_join(base: &Path, user_input: &str) -> Result<PathBuf, String> {
    // 禁止路径遍历
    if user_input.contains("..") {
        return Err("Invalid path: traversal attempt".into());
    }
    Ok(base.join(user_input))
}

// ── Skin commands ────────────────────────────────────────────────

#[tauri::command]
pub async fn load_skin(app: tauri::AppHandle, path: String) -> Result<SkinConfig, String> {
    let resource_path = app.path().resource_dir().map_err(|e| e.to_string())?;

    // 尝试加载指定皮肤
    let result = try_load_skin(&resource_path, &path);

    match result {
        Ok(skin) => {
            // 校验
            let skin_dir = safe_path_join(&resource_path, &path)?;
            let validation = validate_skin(&skin_dir, &skin);
            if validation.valid {
                Ok(skin)
            } else {
                // 校验失败，回退默认
                eprintln!("Skin validation failed, falling back to default");
                for err in validation.errors {
                    eprintln!("  - {}: {}", err.kind, err.message);
                }
                try_load_skin(&resource_path, "assets/skins/default")
                    .map_err(|e| format!("Fallback failed: {}", e))
            }
        }
        Err(e) => {
            // 加载失败，回退默认
            eprintln!("Skin load failed: {}, falling back to default", e);
            try_load_skin(&resource_path, "assets/skins/default")
                .map_err(|e| format!("Fallback failed: {}", e))
        }
    }
}

fn try_load_skin(resource_path: &Path, path: &str) -> Result<SkinConfig, String> {
    let skin_path = safe_path_join(resource_path, path)?.join("skin.json");
    let content = std::fs::read_to_string(&skin_path)
        .map_err(|e| format!("Failed to read skin.json at {}: {}", skin_path.display(), e))?;
    let skin: SkinConfig =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse skin.json: {}", e))?;
    Ok(skin)
}

#[tauri::command]
pub fn validate_skin_path(app: tauri::AppHandle, path: String) -> Result<ValidationResult, String> {
    let resource_path = app.path().resource_dir().map_err(|e| e.to_string())?;
    let skin_dir = safe_path_join(&resource_path, &path)?;

    let skin_json_path = skin_dir.join("skin.json");
    let content = std::fs::read_to_string(skin_json_path)
        .map_err(|e| format!("Failed to read skin.json: {}", e))?;
    let skin: SkinConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse skin.json: {}", e))?;

    Ok(validate_skin(&skin_dir, &skin))
}

#[tauri::command]
pub async fn get_frame_base64(
    app: tauri::AppHandle,
    skin_path: String,
    frame_name: String,
) -> Result<String, String> {
    let resource_path = app.path().resource_dir().map_err(|e| e.to_string())?;
    let safe_skin = safe_path_join(&resource_path, &skin_path)?;
    let frame_path = safe_path_join(&safe_skin, &frame_name)?;

    let bytes = std::fs::read(&frame_path)
        .map_err(|e| format!("Failed to read frame at {}: {}", frame_path.display(), e))?;

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    ))
}

// ── State-machine commands ───────────────────────────────────────

#[tauri::command]
pub fn switch_state(
    state_machine: State<'_, Mutex<PetStateMachine>>,
    new_state: String,
) -> Result<(), String> {
    let mut sm = state_machine.lock().map_err(|e| e.to_string())?;
    sm.switch_state(new_state);
    Ok(())
}

#[tauri::command]
pub fn get_current_state(
    state_machine: State<'_, Mutex<PetStateMachine>>,
) -> Result<String, String> {
    let sm = state_machine.lock().map_err(|e| e.to_string())?;
    Ok(sm.current_state().to_string())
}

#[tauri::command]
pub fn advance_frame(
    state_machine: State<'_, Mutex<PetStateMachine>>,
    total_frames: usize,
) -> Result<(), String> {
    let mut sm = state_machine.lock().map_err(|e| e.to_string())?;
    sm.advance_frame(total_frames);
    Ok(())
}

// ── Skin manager commands ───────────────────────────────────────

#[tauri::command]
pub fn list_skins(app: tauri::AppHandle) -> Result<Vec<SkinInfo>, String> {
    let resource_path = app.path().resource_dir().map_err(|e| e.to_string())?;
    let skins_dir = resource_path.join("assets/skins");
    Ok(skin_manager::list_skins(&skins_dir))
}

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Result<PetSettings, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(skin_manager::load_settings(&app_data_dir))
}

#[tauri::command]
pub fn save_settings_cmd(app: tauri::AppHandle, settings: PetSettings) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    skin_manager::save_settings(&app_data_dir, &settings)
}
