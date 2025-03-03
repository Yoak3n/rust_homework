<script lang="ts" setup>
import ChatBoard from '../components/Chat/ChatBoard/index.vue'
import { NIcon } from 'naive-ui';
import {Reload,Send,Pause} from '@vicons/ionicons5'
import type {AppSetting, MessageItem} from '../types/index'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { querySetting } from '../api/db'
import { throttle } from '../utils';
import { invoke } from '@tauri-apps/api/core';
import emitter from '../bus';
import {unListenAll} from '../bus'

const defaultMessages:Array<MessageItem> = [
  {
    role:'assistant', 
    content:'你好，我是你的助手，有什么可以帮助你的吗？', 
    timestamp:0
  }
]

let messages = ref<MessageItem[]>(defaultMessages)
let input = ref<string>('')


const setMessage = (mi:MessageItem) => {
  messages.value.push(mi)
  emitter.emit('scrollToBottom')
}

const inputHeigth = 50;
const inputHeigthString = computed(() => `${inputHeigth}px`)

const submitUserMessage = () => {
  if(input.value.trim() == '') return
  const m:MessageItem = {role:'user', content:input.value, timestamp:0}
  setMessage(m)
  input.value = ''
  generateBotResponseStream()
}


let generating = ref(false)
let appSetting = ref<AppSetting>()
onMounted(async () => {
  appSetting.value = await querySetting()
})
onBeforeUnmount(()=>{
  unListenAll()
})
const generateBotResponseStream = async () => {
  const ts = Date.now()
  const newMessage = {role:'assistant', content:'', timestamp:ts}
  setMessage(newMessage)
  let m:MessageItem = await invoke('completions_stream',{id:ts})
  console.log(m);
  updateHistoryStream(m)
}
const updateHistoryStream = (m: MessageItem) => {
  const index = messages.value.findIndex((item) =>item.timestamp == m.timestamp)
  if (index != -1){
    messages.value[index] = {...messages.value[index], content:m.content, reasoning_content:m.reasoning_content}
  }else{
    messages.value.push(m)
  }
  throttelEmitScrollToBottom()
}
const emitScrollToBottom = () => {
  emitter.emit('scrollToBottom')
}

const resetHistory = async() => {
  messages.value.splice(0, messages.value.length,defaultMessages[0])
}
const throttelEmitScrollToBottom = throttle(emitScrollToBottom, 300)


</script>
<template>
  <div class="chat-view">
    <ChatBoard :messages="messages" :smoothing="appSetting?.smooth" :model="appSetting?.api.model"/>
    <form class="chat-input" @submit="(e) =>{
      e.preventDefault()
      submitUserMessage()
    }">
      <textarea 
      type="textarea" 
      placeholder="Type a message..." 
      v-model="input"  
      required 
      minlength="1" 
      @keydown="(e) => {
        if (e.key === 'Enter' && !e.shiftKey) {
          e.preventDefault()
          submitUserMessage()
        }
      }
      "/>
      <button type="button" class="reset-btn" @click="resetHistory">
        <n-icon size="24">
          <Reload/>
        </n-icon>
      </button>
      <button type="submit" class="submit-btn" v-if="!generating">
        <n-icon size="24">
          <Send/>
        </n-icon>
      </button>
      <button type="button" class="pause-btn" v-else>
        <n-icon size="24">
          <Pause/>
        </n-icon>
      </button>
    </form>
  </div>

</template>

<style scoped>
.chat-view{
  position: relative;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.chat-input{
  position: absolute;
  bottom: 0;
  width: 60%;

  display: flex;
  padding: 5px;
  border-radius: 10px;
  background-color: #eee;
  box-sizing: border-box;
  textarea{
    height: 50px;
    width: 90%;
    border-radius: 5px;
    background-color: #eee;
    padding: 0 17px;
    line-height: 50px;
    font-size: 1rem;
    overflow: auto; /* 允许滚动 */
    resize: none; /* 禁用调整大小手柄 */
    scrollbar-width: none; /* 隐藏滚动条（Firefox） */
    -ms-overflow-style: none; /* 隐藏滚动条（IE/Edge） */
    position: relative; 
  }
  button{
    height: v-bind(inputHeigthString);
    border-radius: 5px;
    padding:10px 15px;
    color: #fff;
    background-color: #6d4fc2;
    cursor: pointer;
    &.reset-btn{
      &:hover{
        background-color: #593bab;
      }
    }
    &.pause_btn{
      background-color: #fff;
      color: #333;
      &:hover{
        background-color: #593bab;
      }
    }
  }
  &:valid button{
    display: block;
    background-color: #6d4fc2;
    cursor: pointer;
    &:hover{
      background-color: #593bab;
      border: none;
      outline: none;
    }
    &:active{
      border: none;
    }
  }
  &:invalid  button.submit-btn{
    background-color: gray;
    cursor: not-allowed;
  }

  &:focus-within{
    outline: 2px solid #6d4fc2;
  }

  @media screen and (max-width: 768px) {
    button{
      padding: 5px 10px;
    }
  }
}
</style>