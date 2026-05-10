mod commands;
mod interaction;
mod skin;
mod skin_manager;
mod skin_validator;
mod state_machine;
mod tray;

use std::sync::Mutex;

use skin_manager::PetSettings;
use state_machine::PetStateMachine;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(PetStateMachine::new()))
        .manage(Mutex::new(PetSettings::default()))
        .invoke_handler(tauri::generate_handler![
            commands::load_skin,
            commands::validate_skin_path,
            commands::get_frame_base64,
            commands::switch_state,
            commands::get_current_state,
            commands::advance_frame,
            crate::interaction::set_ignore_cursor_events,
            crate::interaction::start_dragging,
            crate::interaction::stop_dragging,
            commands::list_skins,
            commands::get_settings,
            commands::save_settings_cmd,
        ])
        .setup(|app| {
            // 设置托盘
            crate::tray::setup_tray(app.handle())?;
            
            // 刷新皮肤菜单
            crate::tray::refresh_skin_menu(app.handle())?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}