use tauri::{App, GlobalShortcutManager, Manager};

use crate::window::{setup_window, toggle_window};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let handle = app.app_handle();

    setup_window(&handle);

    {
        let handle = app.app_handle();
        let mut shortcut = app.global_shortcut_manager();
        let accelerator = "CmdOrCtrl+Shift+G";
        let core_shortcut = shortcut.is_registered(accelerator);

        if core_shortcut.is_ok() {
            shortcut
                .register(accelerator, move || {
                    if let Some(w) = handle.get_window("main") {
                        toggle_window(&w);
                    }
                })
                .unwrap();
        };
    }

    Ok(())
}
