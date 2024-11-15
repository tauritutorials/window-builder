use std::time::Duration;

use tauri::{
    tray::TrayIconEvent, utils::{config::WindowEffectsConfig, WindowEffect}, window::Effect, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder
};
use tauri_plugin_positioner::{Position, WindowExt};

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
fn effects(app: AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(&app, "effects", WebviewUrl::App("effects.html".into()))
        .title("transparent effects")
        .resizable(false)
        .theme(Some(tauri::Theme::Dark)) // changes theme on other windows
        .closable(false)
        .transparent(true)
        .inner_size(400.0, 800.0)
        .effects(WindowEffectsConfig {
            effects: vec![WindowEffect::HudWindow],
            state: None,
            radius: Some(24.0),
            color: None,
        })
        .build()?;

    Ok(())
}

#[tauri::command]
fn floating(app: AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(&app, "floating", WebviewUrl::App("index.html".into()))
        .always_on_top(true)
        .decorations(false)
        .inner_size(400.0, 400.0)
        .position(0.0, 0.0)
        .build()?;

    Ok(())
}

// current bug with positioner permissions
// https://github.com/tauri-apps/plugins-workspace/issues/1891
// add this to capabilities
//   "permissions": [
//       "positioner:allow-move-window",
//       "positioner:allow-set-tray-icon-state"
//   ]

fn position(app: &AppHandle) -> tauri::Result<()> {
    let window =
        WebviewWindowBuilder::new(app, "position", WebviewUrl::App("position.html".into()))
            .build()?;

    // all the good positions
    // ---------------------------------------------
    // window.move_window(Position::TopLeft)?;
    // window.move_window(Position::TopCenter)?;
    // window.move_window(Position::TopRight)?;
    //
    // window.move_window(Position::BottomLeft)?;
    // window.move_window(Position::BottomCenter)?;
    // window.move_window(Position::BottomRight)?;
    //
    // window.move_window(Position::LeftCenter)?;
    // window.move_window(Position::Center)?;
    // window.move_window(Position::RightCenter)?;
    //
    // requires tray position to be set
    // window.move_window(Position::TrayBottomCenter)?;
    // window.move_window(Position::TrayBottomLeft)?;
    // window.move_window(Position::TrayBottomRight)?;
    //
    // window.move_window(Position::TrayLeft)?;
    // window.move_window(Position::TrayCenter)?;
    window.move_window(Position::TrayRight)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            new_window,
            new_window_or_focus,
            effects,
            floating,
        ])
        .setup(|app| {
            tauri::tray::TrayIconBuilder::with_id("main")
                .menu_on_left_click(false)
                .on_tray_icon_event(|tray_handle, event| {
                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);

                    match event {
                        TrayIconEvent::Click { .. } => {
                            position(tray_handle.app_handle()).ok();
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
