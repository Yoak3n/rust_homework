mod invoke;
mod model;
mod state;
mod interaction;
use std::sync::{Mutex,Arc};
use tauri_plugin_window_state;
use super::store::setting::Configuration;
use state::AppState;



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
            invoke::get_config
            ]
        )
        .setup(|app| {

            #[cfg(desktop)]
            {   
                interaction::register_shortcuts(app)?;
                interaction::create_systray(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

