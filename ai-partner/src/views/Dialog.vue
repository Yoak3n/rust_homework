<script lang="ts" setup>
import { ref,onMounted, onUnmounted } from 'vue'
import DialogItem from '../components/Chat/ChatDialog/index.vue'
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window'
import type {Window }from '@tauri-apps/api/window'
import emitter from '../bus'

const dialogWrapper = ref<HTMLElement | null>(null)
let appWindow :Window;

onMounted(async() => {
  // ... 原有的 onMounted 代码 ...
  appWindow = getCurrentWindow()
  if (dialogWrapper.value) {
    resizeObserver.observe(dialogWrapper.value)
  }
  emitter.on('message-cleared', () => {
    appWindow.setSize(new PhysicalSize(800, 100))
  })
})
const resizeObserver = new ResizeObserver(async (entries) => {
  for (const entry of entries) {
    const dialogContent = entry.target.querySelector('.dialog-content')
    const messageContainer = entry.target.querySelector('.message-container')
    if (!dialogContent || !messageContainer) continue
    
    const contentHeight = dialogContent.getBoundingClientRect().height
    const messageHeight = messageContainer.scrollHeight
    const headerHeight = 28  // 头部高度
    const totalHeight = messageHeight + contentHeight + headerHeight + 10
    await appWindow.setSize(new PhysicalSize(800, Math.min(Math.max(totalHeight, 100), 800)))
    //                                                              ^^^
  }
})
onUnmounted(async() => {
  resizeObserver.disconnect()
  await appWindow.setSize(new PhysicalSize(800, Math.min(100, 800)))
  emitter.off('message-cleared')
})
</script>
<template>
    <div class="dialog-wrapper" ref="dialogWrapper" :key="'dialog-wrapper'">
        <DialogItem />
    </div>
</template>
<style>
body{
    height: 100%;
}
.body{
    height: 100%;
}
.dialog-wrapper {
    height: 100%;
    display: flex;
    flex-direction: column;
}
</style>