#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;

use app::{cmd, menu, setup, updater, window};
use tauri;

#[tokio::main]
async fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![cmd::run_check_update])
        .setup(setup::init)
        .plugin(tauri_plugin_positioner::init())
        .system_tray(menu::tray_menu())
        .on_window_event(window::on_window_event)
        .on_system_tray_event(menu::tray_handler)
        .run(context)
        .expect("error while running ChatGPT application");
}
