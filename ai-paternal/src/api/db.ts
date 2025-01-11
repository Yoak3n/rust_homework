import { invoke } from '@tauri-apps/api/core';
import Database from '@tauri-apps/plugin-sql';

// 初始化数据库
export async function initDb() {
    window.$DB = await Database.load('sqlite:ai.db');
}

// 查询数据
export async function queryDb(sql: string, params: any[] = []) {
return await invoke('query_db', { sql, params });
}

// 插入数据
export async function insertDb(sql: string, params: any[] = []) {
return await invoke('insert_db', { sql, params });
}

// 更新数据
export async function updateDb(sql: string, params: any[] = []) {
return await invoke('update_db', { sql, params });
}

// 删除数据
export async function deleteDb(sql: string, params: any[] = []) {
return await invoke('delete_db', { sql, params });
}