use tauri::{
    menu::{Menu, MenuItem}, tray::TrayIconBuilder, Manager
};
use tauri::{App, Emitter, Error};
use tauri_plugin_global_shortcut::{Builder, Code, Modifiers, ShortcutState};
pub fn register_shortcuts(app: &App) -> Result<(), Error> {
    let handle = app.handle();
    handle.plugin({
        if let Ok(builder) = Builder::new().with_shortcuts(["alt+n", "alt+1"]) {
            builder
                .with_handler(|app_handle, shortcut, event| {
                    let main_window = app_handle.get_webview_window("main").unwrap();
                    if event.state == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::ALT, Code::KeyN) {
                            let _ = main_window.emit("dialog", "1");
                        }
                        if shortcut.matches(Modifiers::ALT, Code::Digit1) {
                            let _ = main_window.emit("dialog", "1");
                        }
                    }
                })
                .build()
        } else {
            Builder::new().build()
        }
    })
}


pub fn create_systray(app: &mut App) -> Result<(), Error> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "GUI", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&hide_i, &quit_i])?;
    let _ = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "hide" => {
                if let Some(main_window) = app.get_webview_window("main") {
                    if main_window.is_visible().unwrap() {
                        main_window.hide().unwrap();
                    } else {
                        main_window.show().unwrap();
                    }
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;
    Ok(())
}
