use tauri::{App, GlobalShortcutManager, Manager};

use crate::app::{updater, window};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let handle = app.app_handle();

    tauri::async_runtime::spawn(async move {
        window::setup_window(&handle);
    });

    {
        let handle = app.app_handle();
        let mut shortcut = app.global_shortcut_manager();
        let accelerator = "CmdOrCtrl+Shift+G";
        let core_shortcut = shortcut.is_registered(accelerator);

        if core_shortcut.is_ok() {
            shortcut
                .register(accelerator, move || {
                    if let Some(w) = handle.get_window("main") {
                        window::toggle_window(&w);
                    }
                })
                .unwrap();
        };
    }

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let app = app.handle();
    updater::run_check_update(app, false, None);

    Ok(())
}
