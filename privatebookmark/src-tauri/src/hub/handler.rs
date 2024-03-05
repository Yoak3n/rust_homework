use crate::{db::Database, model::Bookmark,db::sqlite};
use std::sync::Mutex;

#[tauri::command]
pub fn create_bookmark(bookmark:Bookmark,state:tauri::State<Mutex<Database>>) -> String {
  let mut l = state.lock().expect("lock failed");
  let result = sqlite::create_bookmark(&mut l.conn, bookmark);
  match result {
    Ok(_) => {
     "Successfully".to_string()
    }
    Err(e) => {
      format!("Failed: {}", e).to_string()
    }
  }
      

}
