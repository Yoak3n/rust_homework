import { UnlistenFn } from '@tauri-apps/api/event'
let ListenHub:Map<number,UnlistenFn> = new Map()


export function getUnlistenFnAndOff(key:number):UnlistenFn|undefined{
    const unlisten = ListenHub.get(key)
    if (unlisten){
        return unlisten
    }
}
export function registerNewListen(key:number,unlisten:UnlistenFn){
    ListenHub.set(key,unlisten)
}

export function unListenAll(){
    ListenHub.forEach((v,k)=>{
        v()
        ListenHub.delete(k)
    })
}