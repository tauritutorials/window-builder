use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn new_window(app: AppHandle) {
    WebviewWindowBuilder::new(&app, "window-1", WebviewUrl::App("index.html".into()))
        .build()
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![new_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
