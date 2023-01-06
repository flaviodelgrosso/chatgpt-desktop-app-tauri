use crate::window::toggle_window;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

pub fn tray_menu() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("github".to_string(), "View on GitHub"))
            .add_item(
                CustomMenuItem::new("dev_tools".to_string(), "Toggle Developer Tools")
                    .accelerator("CmdOrCtrl+Shift+I"),
            )
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit".to_string(), "Quit ChatGPT")),
    )
}

pub fn tray_handler(handle: &AppHandle, event: SystemTrayEvent) {
    on_tray_event(handle, &event);
    let win = handle.get_window("main").unwrap();

    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            win.move_window(Position::TrayCenter).unwrap();
            toggle_window(&win);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "github" => {
                tauri::api::shell::open(&handle.shell_scope(), env!("CARGO_PKG_REPOSITORY"), None)
                    .unwrap();
            }
            "dev_tools" => {
                win.open_devtools();
                win.close_devtools();
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
