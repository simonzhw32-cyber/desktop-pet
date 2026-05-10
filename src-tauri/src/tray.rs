use std::sync::Mutex;

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, Submenu},
    tray::{TrayIcon, TrayIconBuilder},
    Emitter, Manager,
};

use crate::skin_manager::SkinInfo;

pub fn setup_tray(app: &tauri::AppHandle) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let show_hide = MenuItem::with_id(app, "show_hide", "Show/Hide", true, None::<&str>)?;
    let skin_submenu = Submenu::with_id(app, "skin_submenu", "Switch Skin", true)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_hide, &skin_submenu, &quit])?;

    // Decode icon PNG and create Image
    let icon_bytes = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory(icon_bytes)?;
    let rgba = img.to_rgba8();
    let icon = Image::new_owned(rgba.to_vec(), rgba.width(), rgba.height());

    let app_clone = app.clone();
    let tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "show_hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(true) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                        }
                    }
                }
                "quit" => {
                    let app_clone = app.clone();
                    tauri::async_runtime::spawn(async move {
                        use tauri_plugin_dialog::DialogExt;
                        let confirmed = app_clone
                            .dialog()
                            .message("Are you sure you want to quit Desktop Pet?")
                            .title("Quit Confirmation")
                            .kind(tauri_plugin_dialog::MessageDialogKind::Warning)
                            .ok_button_label("Quit")
                            .cancel_button_label("Cancel")
                            .blocking_show();
                        if confirmed {
                            app_clone.exit(0);
                        }
                    });
                }
                id if id.starts_with("skin_") => {
                    let skin_id = id.replace("skin_", "");
                    let app_clone = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = app_clone.emit("switch_skin", skin_id);
                    });
                }
                _ => {}
            }
        })
        .build(app)?;

    // Keep tray alive
    Box::leak(Box::new(app_clone));

    Ok(tray)
}

/// 更新皮肤子菜单内容（先清空再填充）
pub fn update_skin_menu(
    app: &tauri::AppHandle,
    skins: &[SkinInfo],
    current_skin_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let menu = app.menu().ok_or("menu not found")?;
    let skin_submenu: Submenu<tauri::Wry> = menu
        .get("skin_submenu")
        .and_then(|kind| kind.as_submenu().cloned())
        .ok_or("skin_submenu not found")?;

    // 先移除现有菜单项
    let existing_count = skin_submenu.items().len();
    for i in (0..existing_count).rev() {
        skin_submenu.remove(i)?;
    }

    // 添加皮肤选项
    for skin in skins {
        let id = format!("skin_{}", skin.id);
        let label = if skin.id == current_skin_id {
            format!("✓ {}", skin.name)
        } else {
            skin.name.clone()
        };
        let item = MenuItem::with_id(app, &id, &label, true, None::<&str>)?;
        skin_submenu.append(&item)?;
    }

    Ok(())
}

/// 获取皮肤列表并更新菜单
pub fn refresh_skin_menu(
    app: &tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::skin_manager::list_skins;

    let skins = list_skins(std::path::Path::new("assets/skins"));

    let current_skin_id = app
        .state::<Mutex<crate::skin_manager::PetSettings>>()
        .lock()
        .map(|s| s.current_skin_id.clone())
        .unwrap_or_else(|_| "default".to_string());

    update_skin_menu(app, &skins, &current_skin_id)?;
    Ok(())
}
