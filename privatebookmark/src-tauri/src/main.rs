// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hellow,{}! You've been greeted from Rust!", name)
}
use bookmark::db::sqlite;
fn main(){
    let mut conn = sqlite::connect_database("bookmark").unwrap();
    
    let r = sqlite::create_table(&mut conn);
    match r {
        Ok(_) => println!("create table success"),
        Err(e) => println!("create table failed:{}",e),
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
