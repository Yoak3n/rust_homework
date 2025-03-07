<script lang="ts" setup>
import { nextTick, onMounted, onUnmounted, ref } from 'vue'
import { NIcon,NDivider} from 'naive-ui'
import {InfiniteSharp} from '@vicons/ionicons5'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { UnlistenFn,listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

import {debounce, throttle} from '../../../utils'
import { MessageItem,Payload } from '../../../types'
import emitter from '../../../bus'

let message = ref('')
let dragging = ref(false)
const submittMessage = () => {
  makeMessage(message.value)
}
const defaultMessage = [{role: 'assistant', content: '', reasoning_content: ''}]
let messageUpdate = ref<MessageItem>(defaultMessage[0])
const makeMessage = async (question:string) => {
  const messageItems:Array<MessageItem> = [{role: 'user', content: question,reasoning_content:''},]
  message.value= ''
  const ts = Date.now()
  let index = 0;
  const unlistenFnData = await listen('stream-data', (event) => {
            const payload = event.payload as Payload
            if (payload.id != ts) return
            index++
            if (payload.message_type == 'reasoning_content') {
                if (payload.index != index) return
                messageUpdate.value!.reasoning_content += payload.data
            } else if (payload.message_type == 'content') {
                if (payload.index != index) return
                messageUpdate.value!.content += payload.data
            } else {
                unlistenFnData()
            }
            nextTick(() => {
                throttleEmitScrollToBottom()
            })
        })
  invoke('completions_stream', { messages: messageItems,id: ts}).catch((e) => {
    window.$message.error(e.toString(),{ duration: 3000 })
  })
}
const switchState = debounce(() => {dragging.value = false},300)
let unlisten:UnlistenFn;
onMounted(async() => {
  unlisten = await getCurrentWindow().listen('tauri://move', () => {
    switchState()
  })
})
onUnmounted(() => {
  unlisten()
})
const recircle = ()=>{messageUpdate.value= {...defaultMessage[0],content:'',reasoning_content:''}}
const throttleEmitScrollToBottom = throttle(() => {emitter.emit('scrollToBottom')},500)
</script>


<template>
  <div class="dialog">
    <div class="dialog-bg"></div>
    <div class="dialog-header"  data-tauri-drag-region :class="dragging?'dragging':''" @mousedown="dragging=true">
    </div>
    <div class="dialog-content">
        <textarea class="dialog-textarea" rows="4" cols="50" v-model="message" @keydown="(e)=>{
        if(e.key === 'Enter' && !e.shiftKey){e.preventDefault();submittMessage()}
        }" >
        </textarea>
        <button class="btn" @click="recircle">
          <n-icon>
            <InfiniteSharp />
          </n-icon>
        </button>
    </div>
    <div class="output">
      {{ messageUpdate.reasoning_content }}
      <n-divider v-if="messageUpdate.content != ''"></n-divider>
      {{ messageUpdate.content }}
    </div>
  </div>

</template>


<style>
body{
  background-color: transparent;
}

.dialog-bg{
  position: fixed;
  height: 100%;
  width: 100%;
  user-select: none;
  filter: blur(10px);
  z-index: -1;
}

.dialog-header {
  width: 50%;
  margin: 0 auto;
  height: 8px;
  backdrop-filter: blur(10px);
  border-radius: 10px;
  background-color: transparent;
  cursor: pointer;
  transition: all .1s ease-out;
  &:hover,
  &.dragging{
    background-color: rgba(200, 200, 200,.7);
  }
}

.dialog-content {
  padding: 10px;
  backdrop-filter: blur(10px);
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: 20px;
  background-color: rgba(77, 107, 254, .5);
  height: 50px;
  .btn{
    height: 50px;
    border-radius: 5px 10px 10px 5px;
    background: #4d6bfe; /* 主色不透明版本 */
    color: white;
    font-weight: 500;
    font-size: 18px;
    border: none;
    cursor: pointer;
    transition: all .1s ease-out;
    i{
      transform: rotateZ(90deg);
    }
    &:hover{
      background: #3a55cc;
      transform: translateY(-1px);
    }
  }
}

.dialog-textarea {
  width: 100%;
  height: 100%;
  border: none;
  padding: 10px;
  box-sizing: border-box;
  resize: none;
  font-size: large;
  background-color: transparent;
  color: #fff;
}

</style>