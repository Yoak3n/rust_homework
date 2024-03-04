use crate::model::Bookmark;

#[tauri::command]
pub async fn create_bookmark(bookmark:Bookmark) -> Result<(), String> {
  println!("create_bookmark{:?}",bookmark);
  Ok(())
}
