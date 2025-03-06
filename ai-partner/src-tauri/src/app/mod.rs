mod invoke;
mod model;
mod state;
mod interaction;
use std::sync::{Mutex,Arc};
use tauri_plugin_window_state;
use super::store::setting::Configuration;
use state::AppState;
// use tauri::{Emitter, Listener, Manager};
// use tokio::time::{sleep, Duration};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let config = Configuration::init_config().unwrap();
    let app_state = AppState{
        config:Arc::new(Mutex::new(config))
    };
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            invoke::greet,
            invoke::create_dialog,
            invoke::get_app_install_path,
            invoke::completions_stream,
            invoke::pause_stream,
            invoke::set_config,
            invoke::get_config,
            invoke::on_drag_end_invoke
            ]
        )
        .setup(|app| {
            // let debounce_timer = Arc::new(Mutex::new(None::<tokio::task::JoinHandle<()>>));
            #[cfg(desktop)]
            {  
                // let window = app.get_webview_window("dialog").unwrap();

                // window.listen("tauri://move", move|_|{
                //     let debounce_timer = debounce_timer.clone();

                //     let mut timer = debounce_timer.lock().unwrap();
                //     if let Some(handle) = timer.take() {
                //         handle.abort();
                //     }
                //     *timer = Some(tokio::spawn(async move{
                //         sleep(Duration::from_millis(200)).await;
                //         let _ = app.emit("tauri://move-end", ());
                //     }))
                    
                // });
                interaction::register_shortcuts(app)?;
                interaction::create_systray(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// async fn handle_move_end(){

// }