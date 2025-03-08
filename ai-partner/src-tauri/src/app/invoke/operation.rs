use tauri::{
    AppHandle,State,
    Error,
    Manager,
    WebviewWindowBuilder,WebviewUrl,
    PhysicalSize
};
use crate::app::{Configuration,AppState};



#[tauri::command]
pub async fn create_dialog(app_handle: AppHandle) -> Result<(), Error> {
    match app_handle.get_webview_window("dialog")  {
        Some(w) => {
            let v = w.is_visible()?;
            if v {
                w.set_always_on_top(false)?;
                w.hide()?;
            }else{
                w.set_always_on_top(true)?;
                w.show()?;
            }
        },
        None => {
            let window = WebviewWindowBuilder::new(
                &app_handle ,"dialog", 
                WebviewUrl::App("/dialog".into()))
                .transparent(true)
                .center()
                .title("")
                .resizable(false)
                .shadow(false)
                .always_on_top(true)
                .decorations(false)
                .build()?;

            window.set_size(PhysicalSize::new(600, 400))?;

        }
    } 
    Ok(())
}


use std::env::current_exe;
#[tauri::command]
pub fn get_app_install_path() -> Result<String, String> {
    let exe_path = current_exe().map_err(|e| e.to_string())?;
    
    let install_dir = if cfg!(target_os = "macos") {
        // macOS: 可执行文件位于 .app/Contents/MacOS/ 下
        exe_path.parent()  // MacOS 目录
            .and_then(|p| p.parent())  // Contents 目录
            .and_then(|p| p.parent())  // .app 目录
    } else {
        // Windows 和 Linux: 可执行文件在安装目录的子目录或根目录
        exe_path.parent()
    };
    
    install_dir
        .ok_or("无法确定安装目录".to_string())
        .and_then(|p| p.to_str()
            .ok_or("路径转换错误".to_string())
            .map(|s| s.to_string()))
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Configuration, String> {
    let config =  state.config.try_lock().expect("get config lock error");
    Ok(config.clone())
}

#[tauri::command]
pub async fn set_config(state : State<'_,AppState>,new_config: Configuration) -> Result<(), String> {
    // update_config(|c|{
    if let Err(e) = Configuration::update_config(move|config|
        {
            config.api.key = new_config.api.key.clone();
            config.api.url = new_config.api.url.clone();
            config.api.model = new_config.api.model.clone();
            config.smooth = new_config.smooth.clone();
            let mut c = state.config.try_lock().expect("set config lock error");
            *c = config.clone();
        })
    {
        return Err(e.to_string());
    }
    // });
    Ok(())
}