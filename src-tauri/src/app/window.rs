use tauri::{
    utils::config::WindowUrl, window::WindowBuilder, AppHandle, GlobalWindowEvent, Window,
    WindowEvent,
};

pub fn setup_window(handle: &AppHandle) {
    WindowBuilder::new(
        handle,
        "main",
        WindowUrl::App("https://chat.openai.com/chat".into()),
    )
    .title("ChatGPT")
    .fullscreen(false)
    .inner_size(450.0, 550.0)
    .decorations(false)
    .always_on_top(true)
    .resizable(true)
    .visible(false)
    .initialization_script(include_str!("../assets/init.ts"))
    .user_agent("chrome")
    .build()
    .unwrap();
}

pub fn on_window_event(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

pub fn toggle_window(window: &Window) {
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
    }
}
