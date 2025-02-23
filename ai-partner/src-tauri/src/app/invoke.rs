#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
pub async fn create_dialog(app_handle: tauri::AppHandle) -> Result<(), String> {
    let _webview_window = tauri::WebviewWindowBuilder::new(&app_handle, "label", tauri::WebviewUrl::App("/dialog".into()))
    .build()
    .unwrap();
    let _  = _webview_window.center();
    Ok(())
}
