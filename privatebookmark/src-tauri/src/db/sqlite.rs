use rusqlite::{Connection,Result};


pub fn connect_database(dbname: &str)->Result<Connection>{
    let conn = Connection::open(format!("{}.db",dbname))?;

    Ok(conn)
}

pub fn create_table(conn: &mut Connection) ->Result<()>{
    let tx = conn.transaction()?;
    tx.execute(
        "CREATE TABLE if not exists cat_colors (
            id integer  autoincrement primary key,
            name text,
            url text,
            content text,
            created_at text
        )", ())?;

    Ok(())
}

pub fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", ())?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

    tx.commit()
}


fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", ())?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

    tx.commit()
}