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

type DebounceFunction<T extends any[]> = (...args: T) => void;

export function debounce<T extends any[]>(fn: DebounceFunction<T>, delay: number): DebounceFunction<T> {
  let timer: ReturnType<typeof setTimeout> | null = null;

  return (...args: T) => {
    if (timer) {
      clearTimeout(timer);
    }
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  };
}

export function throttle<T extends (...args: any[]) => void>(
    func: T,
    wait: number
  ): (...args: Parameters<T>) => void {
    let lastExecTime = 0; // 上次执行的时间戳
    let timeoutId: ReturnType<typeof setTimeout> | null = null;
  
    return (...args: Parameters<T>) => {
      const now = Date.now();
      const timeSinceLastExec = now - lastExecTime;
  
      // 如果距离上次执行的时间超过 wait，立即执行
      if (timeSinceLastExec >= wait) {
        lastExecTime = now;
        func(...args);
      } else if (!timeoutId) {
        // 否则，设置一个定时器，在剩余时间后执行
        timeoutId = setTimeout(() => {
          lastExecTime = Date.now();
          func(...args);
          timeoutId = null;
        }, wait - timeSinceLastExec);
      }
    };
  }
