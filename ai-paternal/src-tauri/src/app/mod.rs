pub mod invoke;
use std::sync::Mutex;
use tauri::{Manager,tray::TrayIconBuilder};       
#[derive(Default)]
pub struct AppData {
    pub max_postion: (i32, i32),
    pub min_postion: (i32, i32),
    pub hide: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppData::default()))
        .setup(|app| {
            let state = app.state::<Mutex<AppData>>();
            let mut state = state.lock().unwrap();
            state.min_postion = (80, 80);
            state.max_postion = (520, 720);
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new().build(),
        )
        .invoke_handler(tauri::generate_handler![invoke::resize_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}