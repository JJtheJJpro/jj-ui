use std::sync::{Arc, Mutex};

use tauri::{Manager, WindowEvent};

struct DeepSettings {
    exit: Arc<Mutex<bool>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DeepSettings {
            exit: Arc::new(Mutex::new(false)),
        })
        .setup(|app| {
            let app_handle = app.handle();

            let window = app.get_webview_window("main").unwrap();
            //window.set_fullscreen(true).unwrap();

            let app_handle_window_event = app_handle.clone();
            window.on_window_event(move |event| {
                let settings = app_handle_window_event.state::<DeepSettings>();
                match event {
                    WindowEvent::CloseRequested { api, .. } => if !*settings.exit.lock().unwrap() {
                        api.prevent_close();
                    },
                    _ => {}
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
