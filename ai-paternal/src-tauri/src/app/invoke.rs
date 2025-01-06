
use std::{borrow::Borrow, sync::Mutex};
use tauri::{PhysicalSize, Size,State,Position,PhysicalPosition};
use super::{AppData, Settings};

#[tauri::command]
pub async fn resize_window(window: tauri::Window, state: State<'_, Mutex<AppData>>,hide: bool,shortcut:bool) -> Result<(), String> {
    let r: Result<(), tauri::Error>;
    let mut state = state.lock().unwrap();
    if shortcut {
        state.hide = !state.hide;
    }else{
        state.hide = hide;
    }
    println!("{}",state.hide);
    if state.hide {
        let current = window.inner_position().expect("get window position error");
        state.max_postion = (current.x, current.y);
        let _ = window.set_resizable(true);
        r = window.set_size(Size::new(Size::Physical(PhysicalSize {
            width: 50,
            height: 50,
        })));
        let _ = window.set_resizable(false);
        let after = window.inner_position().expect("get window position error");
        state.min_postion = (after.x, after.y);
        let _ = window.set_position(Position::Physical(PhysicalPosition::new(state.max_postion.0 +(520-50)-20, state.max_postion.1-5)));
    } else {
        let current = window.inner_position().expect("get window position error");
        state.min_postion = (current.x, current.y);
        let _ = window.set_resizable(true);
        r = window.set_size(Size::new(Size::Physical(PhysicalSize {
            width: 520,
            height: 720,
        })));
        let _ = window.set_resizable(false);
        if state.min_postion.0 == 80 {
            let _ = window.set_position(Position::Physical(PhysicalPosition::new(state.max_postion.0, state.max_postion.1)));
        }else{
            let _ = window.set_position(Position::Physical(PhysicalPosition::new(state.min_postion.0, state.min_postion.1)));
        }

        let after = window.inner_position().expect("get window position error");
        state.max_postion = (after.x, after.y);
        let _ = window.set_position(Position::Physical(PhysicalPosition::new(state.min_postion.0 - (520-50)+20, state.min_postion.1-5)));
    };
    match r {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}


// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn invoke_api(state: tauri::State<'_, Mutex<AppData>>) -> Result<(String,String,String), String> {
    let state = state.lock().expect("get state error");
    let se:&Settings = state.setting.borrow();
  Ok((se.api.base_url.clone(), se.api.key.clone(),se.api.model.clone()))
}

#[tauri::command]
pub fn modify_api(state: tauri::State<'_, Mutex<AppData>>,base_url: String,key: String,model:String) -> Result<(), String> {
    let mut state = state.lock().expect("get state error");
    state.setting.update_api(crate::setting::API { base_url, key,model});
    Ok(())
}
