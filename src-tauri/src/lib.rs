use tauri::{Manager, WindowEvent};

struct DeepSettings {
    exit: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_fullscreen(true).unwrap();

            window.on_window_event(|event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
