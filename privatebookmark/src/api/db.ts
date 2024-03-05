import Database from "tauri-plugin-sql-api";
import type { Bookmark } from "./type";
// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
const db = await Database.load("sqlite:bookmark.db");

async function initiate_table() {
    let result = await db.execute(
        `CREATE TABLE IF NOT EXISTS bookmark (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            content TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            category_id INTEGER,
            CONSTRAINT fk_category
            FOREIGN KEY (category_id)
            REFERENCES category(id)`
    )
    console.log(result);
    await db.execute(`
    CREATE TABLE IF NOT EXISTS category (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        tab TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )`)
    await db.execute(`
    INSERT INTO category (name,tab) VALUES ('default','default')
    `)
}

initiate_table()

export async function create_bookmark(record: Bookmark) {
    console.log(record);
    const result = await db.execute(
        `INSERT INTO bookmark (name,url,content) VALUES ('${record.name}','${record.url}','${record.content}')
        `)
    return result.lastInsertId
}
export async function update_bookmark(record: Bookmark) {
    const result = await db.execute(
        `UPDATE bookmark SET name='${record.name}',url='${record.url}',content='${record.content}' WHERE id=${record.id}`
    )
    console.log(result);
}
export async function read_bookmarks(): Promise<Bookmark[]> {
    const result = await db.select<Bookmark[]>("SELECT * FROM bookmark")
    return result
}