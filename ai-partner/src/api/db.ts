import Database from '@tauri-apps/plugin-sql';
// 初始化数据库

async function connectDB() {
    return await Database.load('sqlite:ai.db');
}
initDB();
async function initDB() {
    console.log("initDB");
    
    const DB = await connectDB();
    DB.execute(`CREATE TABLE IF NOT EXISTS setting (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    )`);
    const check: Array<SettingRecord> = await DB.select("SELECT key FROM setting")
    if (check.length == 0 || check.find((item) => item.key == 'base_url') == undefined) {
        await insertInitailSetting()
    }
}

export interface SettingRecord {
    key: string,
    value: string
}

// 查询数据
export async function querySetting() {
    console.log("querySetting");
    
    const DB = await connectDB();
    const result: Array<SettingRecord> = await DB.select("SELECT * FROM setting")
    let api: APISetting = { base_url: "", key: "", model: "" }
    if (result.length > 0) {
        result.forEach((item) => {
            if (item.key == 'base_url') {
                item.value = item.value.replace(/\/$/, "")
                api.base_url = item.value
            }
            if (item.key == 'key') {
                api.key = item.value
            }
            if (item.key == 'model') {
                api.model = item.value
            }
        })
    }
    return api
}
export async function querySingleSetting(key: string) {
    const DB = await connectDB();
    const result: Array<SettingRecord> = await DB.select("SELECT * FROM setting WHERE key = $1", [key])
    return result[0]
}
import type { APISetting } from '../types';
// 插入数据
export async function insertInitailSetting() {
    const DB = await connectDB();
    const api: APISetting = {
        "base_url": "",
        "key": "",
        "model": "",
    }
    for (const key in api) {
        if (api.hasOwnProperty(key)) {
            await DB.execute("INSERT INTO setting (key, value) VALUES ($1, $2)", [key, api[key as keyof APISetting]])
        }
    }
    return
}

// 更新数据
export async function updateAllSetting(api: APISetting) {
    const DB = await connectDB();
    await DB.execute(`UPDATE setting SET value = 
        CASE key 
        WHEN $1 THEN $2
        WHEN $3 THEN $4
        WHEN $5 THEN $6
        ELSE ''
        END
        WHERE key IN($1, $3, $5)`, ["base_url", api.base_url, "key", api.key, "model", api.model])

}
export async function updateSingleSetting(key: string, value: string) {
    const DB = await connectDB();
    await DB.execute("UPDATE setting SET value = $1 WHERE key = $2", [value, key])
}

// // 删除数据
// export async function deleteDb(sql: string, params: any[] = []) {
// return await invoke('delete_db', { sql, params });
// }