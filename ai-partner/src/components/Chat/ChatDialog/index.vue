<script lang="ts" setup>
import { nextTick, onMounted, onUnmounted, ref } from 'vue'
import { NIcon} from 'naive-ui'
import {InfiniteSharp} from '@vicons/ionicons5'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { UnlistenFn,listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

import {debounce, throttle} from '../../../utils'
import { MessageItem,Payload } from '../../../types'
import emitter from '../../../bus'
import MarkdownRender from '../../../components/Markdown/index.vue'

let message = ref('')
let dragging = ref(false)
const submittMessage = () => {
  makeMessage(message.value)
  recircle()
}
let timeId:number;
const defaultMessage = [{role: 'assistant', content: '', reasoning_content: ''}]
let messageUpdate = ref<MessageItem>(defaultMessage[0])
const generating = ref(false)
const makeMessage = async (question:string) => {
  const messageItems:Array<MessageItem> = [{role: 'user', content: question,reasoning_content:''},]
  message.value= ''
  timeId = Date.now()
  let index = 0;
  const unlistenFnData = await listen('stream-data', (event) => {
            const payload = event.payload as Payload
            if (payload.id != timeId) return
            index++
            generating.value = true
            if (payload.message_type == 'reasoning_content') {
                if (payload.index != index) return
                messageUpdate.value!.reasoning_content += payload.data
            } else if (payload.message_type == 'content') {

                if (payload.index != index) return
                messageUpdate.value!.content += payload.data
            } else {
                unlistenFnData()
                generating.value = false
            }
            nextTick(() => {
                throttleEmitScrollToBottom()
            })
        })
  invoke('completions_stream', { messages: messageItems,id: timeId}).catch((e) => {
    window.$message.error(e.toString(),{ duration: 3000 })
  })
}
const switchState = debounce(() => {dragging.value = false},300)
let unlisten:UnlistenFn;
onMounted(async() => {
  emitter.emit('message-cleared')
  unlisten = await getCurrentWindow().listen('tauri://move', () => {
    switchState()
  })
})
onUnmounted(() => {
  unlisten()
})
const recircle = ()=>{
  messageUpdate.value= {...defaultMessage[0],content:'',reasoning_content:''}
  emitter.emit('message-cleared')
}
const throttleEmitScrollToBottom = throttle(() => {emitter.emit('scrollToBottom')},500)
</script>


<template>
  <div class="dialog">
    <div class="dialog-bg"></div>
    <div class="dialog-header"  data-tauri-drag-region :class="dragging?'dragging':''" @mousedown="dragging=true">
    </div>
    
   
    <div class="dialog-content">
      <textarea class="dialog-textarea" 
      :placeholder="generating?'生成中...':'输入你的问题'"
      rows="4" 
      cols="50" 
      v-model="message" 
      @keydown="(e)=>{
      if(e.key === 'Enter' && !e.shiftKey){e.preventDefault();submittMessage()}
      }" >
      </textarea>
      <button class="btn" @click="recircle">
        <n-icon>
          <InfiniteSharp />
        </n-icon>
      </button>
  </div>
  <div class="message-container" v-if="messageUpdate.content != '' || messageUpdate.reasoning_content != ''"  >
    <div class="output">
      <div class="message-item" v-if="messageUpdate.reasoning_content">
        <div class="reasoning">{{ messageUpdate.reasoning_content }}</div>
      </div>
      <div class="message-item" v-if="messageUpdate.content">
        <div class="content">
          <MarkdownRender :source="messageUpdate.content" />
        </div>
      </div>
    </div>
  </div>
  </div>

</template>


<style lang="less">
body{
  background-color: transparent;
}
.message-container {
  flex: 1;
  overflow-y: auto;
  height: 100%;
  margin: 15px 0;
  padding: 0 15px;
  max-height: 100vh;
}
.message-container::-webkit-scrollbar {
  width: 6px;
}
message-container::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}
message-container::-webkit-scrollbar-track {
  background-color: transparent;
}

.output {
  line-height: 1.6;
  font-size: 15px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1); 
}

.message-item {
  margin-bottom: 20px;
  animation: fadeIn 0.3s ease-in-out;
}

.reasoning {
  padding: 12px 16px;
  color: rgba(255, 255, 255, 0.75);  // 降低文字透明度
  background: rgba(44, 62, 80, 0.45);  // 降低背景不透明度
  border-radius: 12px;
  margin-bottom: 10px;
  box-shadow: 0 1px 8px rgba(0, 0, 0, 0.05);  // 减弱阴影
  border: 1px solid rgba(255, 255, 255, 0.05);  // 减弱边框
  font-size: 14px;  // 稍微缩小字号
}

.content {
  padding: 16px 20px;  // 增加内边距
  background: rgba(77, 107, 254, 0.4);  // 稍微提高不透明度
  border-radius: 16px;  // 增大圆角
  color: #fff;
  font-size: 15.5px;  // 略微增加字号
  box-shadow: 0 4px 16px rgba(77, 107, 254, 0.15);  // 添加阴影效果
  border: 1px solid rgba(77, 107, 254, 0.2);  // 添加微妙的边框
  line-height: 1.7;  // 增加行高
  letter-spacing: 0.3px;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
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
.dialog {
  display: flex;
  height: 100%;
  flex-direction: column;
  box-sizing: border-box;
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
  &::placeholder {
    color: rgba(255, 255, 255, 0.8);
    font-style: italic;
  }
}
</style>