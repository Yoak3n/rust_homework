pub mod sqlite;

use rusqlite::Connection;
#[derive(Debug)]
pub struct Database {
    pub conn:Connection,
    pub name:String
}


pub fn new_database(name :&str)->Database{
    let conn = sqlite::connect_database(name).unwrap();
    Database { conn:conn.into(), name: name.to_string() }
}