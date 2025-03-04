import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type{ AppSetting } from "../../types";
export const useApiStore = defineStore("api", {
  state: () => ({
    api: {
        url: "",
        key: "",
        model:""
    },
    smooth: false
  }),
  actions: {
        async getApifromConfig (){
          const setting:AppSetting = await invoke("get_config")
          this.api.url = setting.api.url
          this.api.key = setting.api.key
          this.api.model = setting.api.model
          this.smooth = setting.smooth
      }
  }
})