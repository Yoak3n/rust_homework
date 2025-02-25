import { createApp } from "vue";
import App from "./App.vue";
import "./assets/style/index.css";
const app = createApp(App)

import router from "./router";
app.use(router)
import {UnlistenFn,listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import {Window } from '@tauri-apps/api/window'
let current = Window.getCurrent();
let unlisten:UnlistenFn 
if (current.label == 'main'){
    unlisten = await listen<string>('dialog',(event)=>{
        console.log('payload',event.payload);
        invoke('create_dialog')
      })
}

app.mount("#app");
app.onUnmount(()=>{
    if (unlisten != null || unlisten !=undefined) {
        unlisten()
    }

})
