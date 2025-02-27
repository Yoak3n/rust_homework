#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use serde::Serialize;
use tauri::{
    AppHandle, Error, Manager, WebviewUrl, WebviewWindowBuilder,PhysicalSize,
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
                .resizable(false)
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

use tauri::Emitter;
use futures::StreamExt;
use reqwest::{Client,header::{HeaderMap,HeaderValue,AUTHORIZATION}};

#[derive(Serialize)]
struct MessageBody {

}


#[tauri::command]
pub async fn compeletion_stream(app_handle: tauri::AppHandle) -> Result<(), String> {
    tokio::spawn(async move {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", "your_auth_token"))
                .expect("Invalid authorization header")
        );
        let response = match client.post("https://你的流式API地址")
            .send()
            .await {
                Ok(res) => res,
                Err(e) => {
                    let _ = app_handle.emit("stream-error", e.to_string());
                    return;
                }
            };

        let mut stream = response.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let data = String::from_utf8_lossy(&bytes).to_string();
                    if let Err(e) = app_handle.emit("stream-data", data) {
                        eprintln!("事件发送失败: {}", e);
                    }
                }
                Err(e) => {
                    let _ = app_handle.emit("stream-error", e.to_string());
                }
            }
        }
    });

    Ok(())
}