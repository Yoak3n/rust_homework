<script lang="ts" setup>
import ChatBoard from '../components/Chat/ChatBoard/index.vue'
import { NIcon } from 'naive-ui';
import {Reload,Send,Pause} from '@vicons/ionicons5'
import type { MessageItem} from '../types'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import {storeToRefs} from 'pinia'
import { throttle } from '../utils';
import { invoke } from '@tauri-apps/api/core';
import {unListenAll} from '../bus'
import emitter from '../bus';
import { useApiStore ,useAppStore} from '../store';

const defaultMessages:Array<MessageItem> = [
  {
    role:'assistant', 
    content:'你好，我是你的助手，有什么可以帮助你的吗？', 
    timestamp:0
  }
]

let messages = ref<MessageItem[]>(defaultMessages)
let input = ref<string>('')


const setMessage = (mi:MessageItem) => {messages.value.push(mi);throttleEmitScrollToBottom()}
// 顶栏高度
const inputHeigth = 50;
const inputHeigthString = computed(() => `${inputHeigth}px`)

const submitUserMessage = () => {
  if(input.value.trim() == '') return
  const timestamp = Date.now()
  const m:MessageItem = {role:'user', content:input.value, timestamp:timestamp - 1}
  setMessage(m)
  input.value = ''
  generateBotResponseStream()
}

const $AppStore = useAppStore()
const $ApiStore = useApiStore()
let {api,smooth} = storeToRefs($ApiStore)
let {generating} = storeToRefs($AppStore)
let ts = Date.now()
onMounted(async()=>{
  $ApiStore.getApifromConfig()
})
onBeforeUnmount(()=>{
  unListenAll()
})
const generateBotResponseStream = async () => {
  ts = Date.now()
  const currentMessages:any = []
  messages.value.forEach((item) => currentMessages.push({role:item.role, content:item.content,reasoning_content:''}))
  const newMessage = {role:'assistant', content:'', timestamp:ts, reasoning_content:''}
  setMessage(newMessage)
  invoke('completions_stream',{id:ts,messages:currentMessages}).then((res) => {
    let m = res as MessageItem
    m.timestamp = ts
    updateHistoryStream(m)
  }).catch((e:string) => console.log("catch",e))
}

const updateHistoryStream = (m: MessageItem) => {
  const index = messages.value.findIndex((item) =>item.timestamp == m.timestamp)
  if (index != -1){messages.value[index] = {...messages.value[index], content:m.content, reasoning_content:m.reasoning_content, role:m.role}
  }else{messages.value.push(m)}
  throttleEmitScrollToBottom()
}

const resetHistory = async() => messages.value.splice(0, messages.value.length, defaultMessages[0])
const throttleEmitScrollToBottom  = throttle(() => emitter.emit('scrollToBottom'), 1000)
</script>
<template>
  <div class="chat-view">
    <ChatBoard :messages="messages" :smoothing="smooth" :model="api.model"/>
    <form class="chat-input" @submit="(e) =>{
      e.preventDefault()
      submitUserMessage()
    }">
      <textarea 
      placeholder="Type a message..." 
      v-model="input"  
      required 
      minlength="1" 
      @keydown="(e) => {
        if (e.key === 'Enter' && !e.shiftKey && !generating) {
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
      <button type="button" class="pause-btn" v-else @click="(e)=>{
        e.preventDefault()
        invoke('pause_stream',{id:ts}).then(()=>$AppStore.setGenerating(false))
      }">
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
    padding-top: 15px;
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