use tauri::{
    utils::config::WindowEffectsConfig, window::Effect, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder
};

/// this will create a new window every time it is called using the length of the webview_windows()
/// HashMap to create a unique label for each window.
#[tauri::command]
fn new_window(app: AppHandle) -> tauri::Result<()> {
    let len = app.webview_windows().len();

    WebviewWindowBuilder::new(
        &app,
        format!("window-{}", len),
        WebviewUrl::App("index.html".into()),
    )
    .build()?;

    Ok(())
}

/// attempts to get the window with label focus, if it does it will set the focus to that window,
/// otherwise it will create a brand new window.
#[tauri::command]
fn new_window_or_focus(app: AppHandle) -> tauri::Result<()> {
    match app.webview_windows().get("focus") {
        None => {
            WebviewWindowBuilder::new(&app, "focus", WebviewUrl::App("index.html".into()))
                .build()?;
        }
        Some(window) => {
            window.set_focus()?;
        }
    }

    Ok(())
}

#[tauri::command]
fn options(app: AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(&app, "options", WebviewUrl::App("effects.html".into()))
        .title("Custom title")
        .resizable(false)
        .theme(Some(tauri::Theme::Dark)) // effects other windows also.
        .decorations(false)
        .closable(true)
        .transparent(true)
        .effects(WindowEffectsConfig {
            effects: vec![tauri::utils::WindowEffect::HudWindow],
            state: None,
            radius: Some(14.0),
            color: None,
        })
        .build()?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            new_window,
            new_window_or_focus,
            options
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
