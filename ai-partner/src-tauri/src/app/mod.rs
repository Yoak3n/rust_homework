pub mod invoke;
mod shortcuts;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder
};
use tauri_plugin_window_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![invoke::greet,invoke::create_dialog])
        .setup(|app| {
            #[cfg(desktop)]
            {   
                use tauri::Emitter;
                use tauri_plugin_global_shortcut::{Code, Modifiers,ShortcutState};

                app.handle().plugin(
                    {
                        if let Ok(builder) = tauri_plugin_global_shortcut::Builder::new().with_shortcuts(["alt+n","alt+1"]){
                            builder.with_handler( |app, shortcut,event| {
                                if event.state== ShortcutState::Pressed{
                                    if shortcut.matches(Modifiers::ALT, Code::KeyN) {
                                        let _ = app.emit("dialog", "1");
                                    }
                                    if shortcut.matches(Modifiers::ALT, Code::Digit1) {
                                        let _ = app.emit("dialog", "1");
                                    }
                                }
                            }).build()
                        }else{
                            tauri_plugin_global_shortcut::Builder::new().build()
                        }
                    }
                )?;
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&quit_i])?;
                let _ = TrayIconBuilder::new()
                    .menu(&menu)
                    .show_menu_on_left_click(true)
                    .icon(app.default_window_icon().unwrap().clone())
                    .build(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}