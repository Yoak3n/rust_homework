pub mod invoke;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
    WebviewWindowBuilder
};
#[derive(Default)]
pub struct AppData {
    pub max_postion: (i32, i32),
    pub min_postion: (i32, i32),
    pub hide: bool,
    pub setting: Settings,
}
use crate::setting::Settings;
use tauri_plugin_sql::{Builder, Migration, MigrationKind};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let setting = Settings::new().unwrap();
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE setting (key TEXT PRIMARY KEY,value TEXT);",
            kind: MigrationKind::Up,
        }
    ];
    // let mut setting = setting.unwrap();
    tauri::Builder::default()
        .manage(Mutex::new(AppData::default()))
        .setup(|app| {
            tray_menu_event(&mut *app)?;
            let state = app.state::<Mutex<AppData>>();
            let mut state = state.lock().unwrap();
            state.min_postion = (80, 80);
            state.max_postion = (520, 720);
            state.setting = setting;
            Ok(())
        })
        .plugin(Builder::new()
            .add_migrations("sqlite:ai.db", migrations)
            .build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            invoke::resize_window,
            invoke::invoke_api,
            invoke::modify_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn tray_menu_event(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let setting_i = MenuItem::with_id(app, "setting", "Setting", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&setting_i,&quit_i])?;
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "setting"=>{
                let h = app.app_handle();
                if let Some(window) = h.get_webview_window("setting"){
                    let _ = window.show();
                    let _ = window.set_focus();
                }else{
                    let w = WebviewWindowBuilder::new(app, "setting",tauri::WebviewUrl::App("setting".into())).build().unwrap();
                    let _ = w.set_title("Setting");
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;
    Ok(())
}
