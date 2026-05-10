use tauri::Window;

#[tauri::command]
pub fn set_ignore_cursor_events(window: Window, ignore: bool) -> Result<(), String> {
    window
        .set_ignore_cursor_events(ignore)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_dragging(window: Window) -> Result<(), String> {
    window.set_ignore_cursor_events(false).map_err(|e| e.to_string())?;
    window.start_dragging().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_dragging(window: Window) -> Result<(), String> {
    window
        .set_ignore_cursor_events(true)
        .map_err(|e| e.to_string())
}
