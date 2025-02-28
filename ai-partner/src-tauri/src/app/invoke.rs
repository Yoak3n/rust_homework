#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{
    AppHandle, Error, Manager, WebviewUrl, WebviewWindowBuilder,PhysicalSize,
};
use super::model::*;
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
use serde_json::json;

#[tauri::command]
pub async fn completions_stream(app_handle: tauri::AppHandle,id:usize) -> Result<MessageItem, String> {
    let messages:Vec<MessageItem> = [
        MessageItem{
            role: "system".to_string(),
            content: "你是一个智能助手，请根据用户的问题，提供简洁、准确的回答".to_string(),
            reasoning_content: "".to_string(),
        },
        MessageItem{
            role: "user".to_string(),
            content: "你好，你是谁？".to_string(),
            reasoning_content: "".to_string(),
        }
    ].to_vec();

    // tokio::spawn(async move {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", ""))
                .expect("Invalid authorization header")
        );
        let payload = json!({
            "stream": true,
            "messages": messages,
            "model":"deepseek-r1"
        });
        let response = match client
            .post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
            .headers(headers)
            .json(&payload)
            .send()
            .await {
                Ok(res) => res,
                Err(e) => {
                    println!("请求失败: {}", e);
                    let _ = app_handle.emit("stream-error", e.to_string());
                    return Err(e.to_string());
                }
            };
        if response.status() != 200 {
            println!("请求失败: {}", response.status());
            let _ = app_handle.emit("stream-error", response.status().to_string());
            return Err(response.status().to_string());
        }
        let mut full = MessageItem::default();
        let mut index:usize = 0;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    println!("收到数据: {}", String::from_utf8_lossy(&bytes));
                    if let Some(ret) = handle_stream_data(&bytes){
                        index += 1;
                        for item in ret{
                            full.append(&item);
                            let payload = StreamEmitter::new(item, index, id);
                            if let Err(e) = app_handle.emit("stream-data", payload) {
                                eprintln!("事件发送失败: {}", e);
                            }
                        }
                    }else{
                        println!("收到结束信号");
                        if let Err(e) = app_handle.emit("stream-end", id) {
                            eprintln!("事件发送失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("流错误: {}", e);
                    let _ = app_handle.emit_to("main","stream-error", e.to_string());
                }
            }
        }
    // });

    Ok(full)
}
use std::io::{BufReader,BufRead};
fn handle_stream_data(data: &[u8])->Option<Vec<MessageType>> {
    let mut ret:Vec<MessageType> = vec![];
    let reader = BufReader::new(data);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("data: ") {
            let content = line.trim_start_matches("data: ").trim();
            if content == "[DONE]" {
                return None;
            }
            // 解析JSON
            match serde_json::from_str::<StreamData>(content) {
                Ok(json) => {
                    if let Some(c) = &json.choices[0].delta.content{
                        if !c.is_empty(){ret.push(MessageType::Content(c.clone()));}
                    }else if let Some(r) = &json.choices[0].delta.reasoning_content{
                        if !r.is_empty(){ret.push(MessageType::ReasoningContent(r.clone()));}
                    }
                }
                Err(e) => {
                    eprintln!("JSON解析失败: {} | 原始内容: {}", e, content);
                    continue;
                }
            }
        };
    }
    Some(ret)
}
