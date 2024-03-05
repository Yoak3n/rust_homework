// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hellow,{}! You've been greeted from Rust!", name)
}
use bookmark::{db,hub::handler};
use tauri_plugin_sql::Builder;
use std::sync::Mutex;
fn main(){
    let pool = db::new_database("bookmark");  
    let state = Mutex::new(pool);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,handler::create_bookmark])
        .manage(state)
        .plugin(
            Builder::default()
            .build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
