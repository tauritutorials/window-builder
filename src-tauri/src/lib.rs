use std::time::Duration;

use tauri::{
    tray::TrayIconEvent,
    utils::{config::WindowEffectsConfig, WindowEffect},
    window::Effect,
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder,
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

/// creates a window with a transparent effect, effects.html has a transparent <body>
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
            effects: vec![
                // for macos
                WindowEffect::HudWindow,
                // for windows
                WindowEffect::Acrylic,
            ],
            state: None,
            radius: Some(24.0),
            color: None,
        })
        .build()?;

    Ok(())
}

/// this will create a persistant floating window at the top left corner of the screen. there won't
/// be any way to close it because decorations() is set to false.
#[tauri::command]
fn floating(app: AppHandle) -> tauri::Result<()> {
    match app.webview_windows().get("floating") {
        None => {
            WebviewWindowBuilder::new(&app, "floating", WebviewUrl::App("index.html".into()))
                .always_on_top(true)
                .decorations(false)
                .inner_size(400.0, 400.0)
                .position(0.0, 0.0)
                .build()?;
        }
        Some(window) => {
            window.close()?;
        }
    }
    Ok(())
}

// current bug with positioner permissions
// https://github.com/tauri-apps/plugins-workspace/issues/1891
// add this to capabilities
//   "permissions": [
//       "positioner:allow-move-window",
//       "positioner:allow-set-tray-icon-state"
//   ]

/// This will build a window with position.html and below are examples of everywhere you can move
/// the window to with the positioner plugin. at the bottom we position the window to the right of
/// the tray icon.
fn position(app: &AppHandle) -> tauri::Result<()> {
    let window =
        WebviewWindowBuilder::new(app, "position", WebviewUrl::App("position.html".into()))
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .build()?;

    window.move_window(Position::TrayCenter)?;

    window.clone().on_window_event(move |evt| match evt {
        tauri::WindowEvent::Focused(is_focused) if !is_focused => {
            window.close().ok();
        }
        _ => {}
    });
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
            // here we build the trau icon and give it an id.
            tauri::tray::TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .menu_on_left_click(false)
                .on_tray_icon_event(|tray_handle, event| {
                    // we pass the tray event to the positioner plugin so it can register the
                    // location of the tray icon.
                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);

                    match event {
                        TrayIconEvent::Click { .. } => {
                            // then we call position() to build the window.
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
