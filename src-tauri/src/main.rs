#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;

use app::{menu, setup, window};
use tauri;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .setup(setup::init)
        .plugin(tauri_plugin_positioner::init())
        .system_tray(menu::tray_menu())
        .on_window_event(window::on_window_event)
        .on_system_tray_event(menu::tray_handler)
        .run(context)
        .expect("error while running ChatGPT application");
}
