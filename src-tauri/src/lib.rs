use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, UNIX_EPOCH},
};
use serde::Serialize;
use tauri::{Emitter, Manager, Runtime, State, WindowEvent};

#[derive(Serialize, Clone)]
struct TimeUpdate {
    hrs: i8,
    mins: i8,
    secs: i8,
}


pub const TZ_Y: i8 = -12;
pub const TZ_X: i8 = -11;
/// Principal cities: Honolulu
pub const TZ_W: i8 = -10;
pub const TZ_V: i8 = -9;
pub const TZ_U: i8 = -8;
/// Principal cities: Denver, Calgary, Ciudad Juárez
pub const TZ_T: i8 = -7;
pub const TZ_S: i8 = -6;
pub const TZ_R: i8 = -5;
pub const TZ_Q: i8 = -4;
pub const TZ_P: i8 = -3;
pub const TZ_O: i8 = -2;
pub const TZ_N: i8 = -1;
pub const TZ_Z: i8 = 0;
pub const TZ_A: i8 = 1;
pub const TZ_B: i8 = 2;
pub const TZ_C: i8 = 3;
pub const TZ_D: i8 = 4;
pub const TZ_E: i8 = 5;
pub const TZ_F: i8 = 6;
pub const TZ_G: i8 = 7;
pub const TZ_H: i8 = 8;
pub const TZ_I: i8 = 9;
pub const TZ_K: i8 = 10;
pub const TZ_L: i8 = 11;
pub const TZ_M: i8 = 12;

#[tauri::command]
async fn frontend_loaded<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    settings: State<'_, DeepSettings>,
) -> Result<(), String> {
    let exit_copy = settings.exit.clone();
    let thread_copy = settings.time_thread.clone();

    *thread_copy.lock().unwrap() = Some(thread::spawn(move || {
        while !exit_copy.load(Ordering::Relaxed) {
            let since_the_epoch = UNIX_EPOCH.elapsed().unwrap();
            thread::sleep(Duration::from_millis(
                1000 - since_the_epoch.subsec_millis() as u64,
            ));
            let epoch_secs = since_the_epoch.as_secs();
            let (hrs, mins, secs) = ((((epoch_secs / 60) / 60) % 24) as i8, ((epoch_secs / 60) % 60) as i8, (epoch_secs % 60) as i8);
            match app.emit("main-clock-time-update", TimeUpdate { hrs: hrs + TZ_T, mins, secs }) {
                Ok(()) => {

                    //println!("minute tick test");
                }
                Err(_e) => (),
            }
        }
    }));

    println!("Front-end is up and running");
    Ok(())
}

struct DeepSettings {
    exit: Arc<AtomicBool>,
    time_thread: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage({
            let exit = Arc::new(AtomicBool::new(false));
            DeepSettings {
                exit: exit.clone(),
                time_thread: Arc::new(Mutex::new(None)),
            }
        })
        .setup(|app| {
            let app_handle = app.handle();

            let window = app.get_webview_window("main").unwrap();
            //window.set_fullscreen(true).unwrap();

            let app_handle_window_event = app_handle.clone();
            window.on_window_event(move |event| {
                let settings = app_handle_window_event.state::<DeepSettings>();
                match event {
                    WindowEvent::CloseRequested { api, .. } => {
                        if !settings.exit.load(Ordering::Relaxed) {
                            api.prevent_close();
                        }
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![frontend_loaded])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
