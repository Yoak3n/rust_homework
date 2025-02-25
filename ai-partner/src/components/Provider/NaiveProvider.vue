<script setup lang="ts">
import { defineComponent, h, onMounted, onUnmounted } from 'vue'
import {
  NDialogProvider,
  NLoadingBarProvider,
  NMessageProvider,
  NNotificationProvider,
  useDialog,
  useLoadingBar,
  useMessage,
  useNotification,
} from 'naive-ui'
import { listen,UnlistenFn } from '@tauri-apps/api/event';
import {switchDialogWindow} from '../../utils/index'
function registerNaiveTools() {
  window.$loadingBar = useLoadingBar()
  window.$dialog = useDialog()
  window.$message = useMessage()
  window.$notification = useNotification()
}
let unlisten:UnlistenFn;
const NaiveProviderContent = defineComponent({
  name: 'NaiveProviderContent',
  setup() {
    registerNaiveTools()
    const disableRightClick = (e: MouseEvent) => {
      e.preventDefault();
    };

    // 在组件挂载时添加事件监听器
    onMounted(async() => {
      document.addEventListener('contextmenu', disableRightClick);
      unlisten = await listen<string>('dialog',(event)=>{
        console.log('payload',event.payload);
        switchDialogWindow()
      },{})
    });

    // 在组件卸载时移除事件监听器
    onUnmounted(() => {
      document.removeEventListener('contextmenu', disableRightClick);
      unlisten();
    });
    },
  render() {
    return h('div')
  },
})
</script>

<template>
  <NLoadingBarProvider>
    <NDialogProvider>
      <NNotificationProvider>
        <NMessageProvider placement="bottom">
          <slot />
          <NaiveProviderContent />
        </NMessageProvider>
      </NNotificationProvider>
    </NDialogProvider>
  </NLoadingBarProvider>
</template>