use tauri::{updater::UpdateResponse, AppHandle, Manager, Result, Wry};

pub fn run_check_update(app: AppHandle<Wry>, silent: bool, has_msg: Option<bool>) {
    tauri::async_runtime::spawn(async move {
        let result = app.updater().check().await;
        let update_resp = result.unwrap();
        if update_resp.is_update_available() {
            if silent {
                tauri::async_runtime::spawn(async move {
                    silent_install(app, update_resp).await.unwrap();
                });
            } else {
                tauri::async_runtime::spawn(async move {
                    prompt_for_install(app, update_resp).await.unwrap();
                });
            }
        } else if let Some(v) = has_msg {
            if v {
                tauri::api::dialog::message(
                    app.app_handle().get_window("core").as_ref(),
                    "ChatGPT",
                    "ChatGPT Desktop is updated!",
                );
            }
        }
    });
}

pub async fn prompt_for_install(app: AppHandle<Wry>, update: UpdateResponse<Wry>) -> Result<()> {
    let windows = app.windows();
    let parent_window = windows.values().next();
    let package_info = app.package_info().clone();

    let body = update.body().unwrap();

    let should_install = tauri::api::dialog::blocking::ask(
        parent_window,
        format!(r#"A new version of {} is available! "#, package_info.name),
        format!(
            r#"{} {} is now available -- you have {}.

Would you like to install it now?

Release Notes:
{}"#,
            package_info.name,
            update.latest_version(),
            package_info.version,
            body
        ),
    );

    if should_install {
        update.download_and_install().await?;

        // Ask user if we need to restart the application
        let should_exit = tauri::api::dialog::blocking::ask(
            parent_window,
            "Ready to Restart",
            "The installation was successful, do you want to restart the application now?",
        );
        if should_exit {
            app.restart();
        }
    }

    Ok(())
}

pub async fn silent_install(app: AppHandle<Wry>, update: UpdateResponse<Wry>) -> Result<()> {
    let windows = app.windows();
    let parent_window = windows.values().next();

    update.download_and_install().await?;

    // Ask user if we need to restart the application
    let should_exit = tauri::api::dialog::blocking::ask(
        parent_window,
        "Ready to Restart",
        "The silent installation was successful, do you want to restart the application now?",
    );
    if should_exit {
        app.restart();
    }

    Ok(())
}
