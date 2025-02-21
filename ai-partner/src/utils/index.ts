import { WebviewWindow } from "@tauri-apps/api/webviewWindow";


export const createSettingWindow= async() => {
    let oldWindow = await WebviewWindow.getByLabel("setting");
    if (oldWindow) {

        oldWindow.show();
        oldWindow.setAlwaysOnTop(true);
        return;
    }
    const webviewWindow = new WebviewWindow("setting", {
        url: "/setting",
        title: "设置",
        width: 400,
        height: 300,
        center: true,
        resizable: false,
        alwaysOnTop: true,
    });
    webviewWindow.once("tauri://created", () => {
        console.log("设置窗口已创建");
    });
    webviewWindow.once("tauri://error", (e) => {
        console.log(e);
                
    })
}
export const closeSettingWindow= async()=>{
    const oldWindow = await WebviewWindow.getByLabel("setting");
    if (oldWindow) {
        oldWindow.close();
    }
}


import {invoke} from '@tauri-apps/api/core'
import type {APISetting} from '../types/index'
export  const getApiSetting= async():Promise<APISetting>=>{
    const r:Array<string> = await invoke('invoke_api') 
    if(r.length>0){
        return {base_url:r[0],key:r[1],model:r[2]}
    }
    return {base_url:'',key:'',model:''}
}
