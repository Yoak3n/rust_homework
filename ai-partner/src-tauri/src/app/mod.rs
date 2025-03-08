mod invoke;
mod state;
mod interaction;
use std::sync::{Mutex,Arc};
use tauri_plugin_window_state;
use super::store::setting::Configuration;
use super::store::db::Database;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let config = Configuration::init_config().unwrap();
    let db = Database::new(".".into()).expect("Failed to initialize database");
    let app_state = AppState{
        config:Arc::new(Mutex::new(config)),
        db: Arc::new(db),
    };
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            invoke::operation::create_dialog,
            invoke::operation::get_app_install_path,
            invoke::chat::completions_stream,
            invoke::chat::pause_stream,
            invoke::operation::set_config,
            invoke::operation::get_config,
            invoke::chat::create_conversation,
            invoke::chat::get_conversations,
            invoke::chat::save_message,
            invoke::chat::get_conversation_messages,
            invoke::chat::delete_conversation
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

// async fn handle_move_end(){

// }