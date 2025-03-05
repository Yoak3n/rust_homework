#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{
    AppHandle, Emitter,
    Error, Manager, 
    PhysicalSize, State, 
    WebviewUrl, WebviewWindowBuilder
};
use super::{model::*, state::AppState};
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

use futures::StreamExt;
use reqwest::{Client,header::{HeaderMap,HeaderValue,AUTHORIZATION}};
use serde_json::json;

#[tauri::command]
pub async fn completions_stream(app_handle: tauri::AppHandle, state: State<'_,AppState>,id: usize,messages:Vec<MessageItem>) -> Result<MessageItem, String> {
    let api = state.config.try_lock().expect("get config of state error").api.clone();
    println!("api: {:#?}", api);
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", &api.key))
            .expect("Invalid authorization header")
    );
    let payload = json!({
        "stream": true,
        "messages": messages,
        "model":&api.model,
    });
    let response = match client
        .post(&api.url)
        .headers(headers)
        .json(&payload)
        .send()
        .await {
            Ok(res) => res,
            Err(e) => {
                println!("请求失败: {}", e);
                let payload = StreamError::new(e.to_string(), id);
                let _ = app_handle.emit_to("main","stream-error",&payload);
                return Err(e.to_string());
            }
        };
    if response.status() != 200 {
        println!("请求失败: {}", response.status());
        let payload = StreamError::new(response.status().to_string(), id);
        let _ = app_handle.emit_to("main","stream-error",&payload);
        return Err(response.status().to_string());
    }
    let mut full = MessageItem::default();
    let mut index:usize = 0;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        index += 1;
        match chunk {
            Ok(bytes) => {
                if let Some(ret) = handle_stream_data(&bytes){
                    for item in ret{
                        full.append(&item);
                        let payload = StreamEmitter::new(item, index, id);
                        if let Err(e) = app_handle.emit("stream-data", payload) {
                            eprintln!("事件发送失败: {}", e);
                        }
                    }
                }else{
                    let item = MessageType::DONE;
                    let payload = StreamEmitter::new(item, index, id);
                    if let Err(e) = app_handle.emit("stream-data", payload) {
                        eprintln!("事件发送失败: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("流错误: {}", e);
                let payload = StreamError::new(e.to_string(), id);
                let _ = app_handle.emit_to("main","stream-error", &payload);
            }
        }
    }
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


// use crate::store::config::{Configuration,update_config};
use crate::store::setting::Configuration;
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

#[tauri::command]
pub async fn get_config(state : State<'_,AppState>) -> Result<Configuration, String> {
    let config =  state.config.try_lock().expect("get config lock error");
    Ok(config.clone())
}

#[tauri::command]
pub async fn pause_stream(app: tauri::AppHandle,id:usize) -> Result<(), String> {
    println!("暂停流: {}", id);
    let item = MessageType::DONE;
    let payload = StreamEmitter::new(item, 0, id);
    if let Err(e) = app.emit("stream-data", payload) {
        eprintln!("事件发送失败: {}", e);
    }
    Ok(())
}