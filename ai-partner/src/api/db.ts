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
export async function querySetting() :Promise<AppSetting>{
    const DB = await connectDB();
    const result: Array<SettingRecord> = await DB.select("SELECT * FROM setting")
    let app: AppSetting = { base_url: "", key: "", model: "" ,smoothing:false}
    if (result.length > 0) {
        result.forEach((item) => {
            if (item.key == 'base_url') {
                item.value = (item.value as string).replace(/\/$/, "")
                app.base_url = item.value
            }
            if (item.key == 'key') {
                app.key = (item.value as string)
            }
            if (item.key == 'model') {
                app.model = (item.value as string)
            }
            if (item.key == 'smoothing') {
                if (item.value == 'true') {
                    app.smoothing = true
                }else{
                    app.smoothing = false
                }
            }
        })
    }
    console.log("querySetting:",app);
    return app
}
export async function querySingleSetting(key: string) {
    const DB = await connectDB();
    const result: Array<SettingRecord> = await DB.select("SELECT * FROM setting WHERE key = $1", [key])
    if (result.length <= 0) return null
    console.log(result);
    
    
}
import type { AppSetting } from '../types';
// 插入数据
export async function insertInitailSetting() {
    const DB = await connectDB();
    const app: AppSetting = {
        "base_url": "",
        "key": "",
        "model": "",
        "smoothing":false
    }
    for (const key in app) {
        if (app.hasOwnProperty(key)) {
            await DB.execute("INSERT INTO setting (key, value) VALUES ($1, $2)", [key, app[key as keyof AppSetting].toString()])
        }
    }
    return
}

// 更新数据
export async function updateAllSetting(app: AppSetting) {
    const DB = await connectDB();
    await DB.execute(`UPDATE setting SET value = 
        CASE key 
        WHEN $1 THEN $2
        WHEN $3 THEN $4
        WHEN $5 THEN $6
        WHEN $7 THEN $8
        ELSE ''
        END
        WHERE key IN($1, $3, $5, $7)`, [
        "base_url", app.base_url,
        "key", app.key, 
        "model", app.model, 
        "smoothing", app.smoothing.toString()])
}
export async function updateSingleSetting(key: string, value: string) {
    const DB = await connectDB();
    await DB.execute("UPDATE setting SET value = $1 WHERE key = $2", [value, key])
}

// // 删除数据
// export async function deleteDb(sql: string, params: any[] = []) {
// return await invoke('delete_db', { sql, params });
// }