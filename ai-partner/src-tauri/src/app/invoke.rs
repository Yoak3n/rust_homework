#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{
    AppHandle,
    Error, 
    Manager, 
    WebviewUrl, WebviewWindowBuilder,
};
#[tauri::command]
pub async fn create_dialog(app_handle: AppHandle) -> Result<(), Error> {
    match app_handle.get_webview_window("dialog")  {
        Some(w) => {
            let v = w.is_visible()?;
            if v {
                println!("hide");
                w.hide()?;
            }else{
                println!("show");
                w.show()?;
            }
        },
        None => {
            let window = WebviewWindowBuilder::new(
                &app_handle ,"dialog", 
                WebviewUrl::App("/dialog".into()))
                .transparent(true)
                .center()
                .build()?;

            window.set_resizable(false)?;
            window.set_always_on_top(true)?;
            window.set_decorations(false)?;

        }
    } 
    Ok(())
}
