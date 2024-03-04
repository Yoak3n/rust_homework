use rusqlite::{Connection,Result};
use crate::model::Bookmark;

pub fn connect_database(dbname: &str)->Result<Connection>{
    let conn = Connection::open(format!("{}.db",dbname))?;

    Ok(conn)
}

pub fn create_table(conn: &mut Connection) ->Result<()>{ 
    conn.execute(
        "create table if not exists bookmark (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            url TEXT,
            content TEXT,
            create_at DATETIME

        )", ())?;
    Ok(())
}

pub fn create_bookmark(conn: &mut Connection,bookmark:Bookmark) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("insert into bookmark (name,url,content,create_at) values (?1,?2,?3,?4)", &[&bookmark.name,&bookmark.url,&bookmark.content,&bookmark.create_at.to_string()])?;
    tx.commit()
}

