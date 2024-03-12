import Database from "tauri-plugin-sql-api";
import type { Bookmark, Category } from "./type";
// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.

const SQLITE_PATH = "sqlite:bookmark.db";
async function initiate_table() {
    const db: Database = await Database.load(SQLITE_PATH);
    await db.execute(`CREATE TABLE IF NOT EXISTS category (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        tab TEXT NOT NULL,
        created_at DATETIME DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now', 'localtime'))
    )`)
    await db.execute(
        `CREATE TABLE IF NOT EXISTS bookmark (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            content TEXT,
            created_at DATETIME DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now', 'localtime')),
            category_id INTEGER,
            CONSTRAINT fk_category
            FOREIGN KEY (category_id)
            REFERENCES category(id)
        )`
    )
}

initiate_table()

export async function create_bookmark(record: Bookmark) {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.execute(
        `INSERT INTO bookmark (name,url,content) VALUES ('${record.name}','${record.url}','${record.content}')
        `)
    return result.lastInsertId
}
export async function update_bookmark(record: Bookmark) {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.execute(
        `UPDATE bookmark SET name='${record.name}',url='${record.url}',content='${record.content}' WHERE id=${record.id}`
    )
    console.log(result);
}
export async function read_bookmarks(): Promise<Bookmark[]> {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.select<Bookmark[]>("SELECT * FROM bookmark")
    return result
}

export async function delete_bookmark(id: number) {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.execute(
        `DELETE FROM bookmark WHERE id=${id}`
    )
    console.log(result);
}

export async function create_category(record :Category){
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.execute(
        `INSERT INTO category (name,tab) VALUES ('${record.name}','${record.tab}')`
    )
    console.log(result);
}

export async function update_category(record:Category) {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.execute(
        `UPDATE category SET name='${record.name}',tab='${record.tab}' WHERE id=${record.id}`
    )
    console.log(result);
}

export async function read_categories(): Promise<Category[]> {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.select<Category[]>("SELECT * FROM category")
    return result
}

export async function delete_category(id: number) {
    const db: Database = await Database.load(SQLITE_PATH);
    const result = await db.execute(
        `DELETE FROM category WHERE id=${id}`
    )
    console.log(result);
}