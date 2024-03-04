use chrono::NaiveDateTime;

#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct Bookmark {
    pub name :String,
    pub url :String,
    pub content:String,
    pub create_at:NaiveDateTime,
}

pub fn new_bookmark(name:&str,url:&str,content:&str)->Bookmark{
    Bookmark{
        name:name.to_string(),
        url:url.to_string(),
        content:content.to_string(),
        create_at:NaiveDateTime::from_timestamp_opt(chrono::Local::now().timestamp(),0).unwrap(),
    }
}

