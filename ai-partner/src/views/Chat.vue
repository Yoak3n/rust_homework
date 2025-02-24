<script lang="ts" setup>
import ChatBoard from '../components/Chat/ChatBoard/index.vue'
import { NIcon } from 'naive-ui';
import {Reload,Send,Pause} from '@vicons/ionicons5'
import type {AppSetting, MessageItem} from '../types/index'
import { computed, onMounted, ref } from 'vue';

const defaultMessages:Array<MessageItem> = [
  {role:'assistant', content:'你好，我是你的助手，有什么可以帮助你的吗？', text:'你好，我是你的助手，有什么可以帮助你的吗？',timestamp:0}
]

let messages = ref<MessageItem[]>(defaultMessages)
let input = ref<string>('')
import emitter from '../bus';
const setMessage = (mi:MessageItem) => {
  messages.value.push(mi)
  emitter.emit('scrollToBottom')
}

const inputHeigth = 50;
const inputHeigthString = computed(() => `${inputHeigth}px`)

const submitUserMessage = () => {
  if(input.value.trim() == '') return
  const m:MessageItem = {role:'user', content:input.value, text:input.value}
  setMessage(m)
  input.value = ''
  generateBotResponseStream()
}

import { querySetting } from '../api/db'
import { throttle } from '../utils';
let generating = ref(false)
let appSetting = ref<AppSetting>()
onMounted(async () => {
  appSetting.value = await querySetting()
})
const generateBotResponseStream = async () => {
  const { base_url, key, model } = appSetting.value!
  if (!base_url || !key || !model || base_url === "" || key === "" || model === "") {
    updateHistoryStream({ role: "system-error", content: "请先配置API密钥和模型", text: "请先配置API密钥和模型" })
    return
  }
  const ts  = Date.now()
  const requestOptions = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${key}`,
    },
    body: JSON.stringify({
      model,
      messages: messages.value,
      stream: true
    })
  }
  try {
    let api_target = ""
    if (base_url.endsWith('v1')) {
      api_target = `${base_url}/chat/completions`
    } else {
      api_target = base_url
    }
    if (!api_target.startsWith(`http[s]?://`)) {}
    const res = await fetch(api_target, requestOptions)
    if (!res.ok) throw new Error(res.statusText || "Something went wrong")
    const reader = res.body!.getReader();
    const decoder = new TextDecoder('utf-8');
    let newAnswer: MessageItem = { role: "assistant", content: "", text: "" ,timestamp: ts,reasoning_content:""}
    while (true) {
      const { done, value } = await reader.read();
      if (done) {generating.value = false;break;} // 流结束
      generating.value = true
      // 将二进制数据转换为文本
      decoder.decode(value, { stream: true }).split('\n').forEach(chunk => {
        if (chunk === '\n' || chunk === '' || chunk.includes('data: [DONE]')) return
        const jsonAnswer = chunk.replace('data:', '').replace('data: ', '')
        JSON.parse(jsonAnswer, (key, value) => {
          if (key === 'created'){newAnswer.timestamp = value}
          if (key === 'choices' && !value[0]["finish_reason"]) {
            const answer = value[0].delta.content
            newAnswer.content += answer
            newAnswer.text += answer
            updateHistoryStream(newAnswer)
          }
          return value;
        })
      });
    }


  } catch (err: any) {
    const errMessage: MessageItem = { role: "system-error", content: err.message, text: err.message }
    updateHistoryStream(errMessage)
  }
}
const updateHistoryStream = (m: MessageItem) => {
  try{
    const index = messages.value.findIndex((item) =>item.timestamp == m.timestamp)
    if (index != -1){
      messages.value[index] = {...messages.value[index], content:m.content, text:m.text, reasoning_content:m.reasoning_content}
    }else{
      messages.value.push(m)
    }
    throttelEmitScrollToBottom()
    emitter.emit('updateHistory', m)
  }catch (err) {
    console.log(err);
  }

}
const emitScrollToBottom = () => {
  emitter.emit('scrollToBottom')
}
// const debounceEmitScrollToBottom = debounce(emitScrollToBottom, 300)
const resetHistory = () => {
  messages.value.splice(0, messages.value.length,defaultMessages[0])
}
const throttelEmitScrollToBottom = throttle(emitScrollToBottom, 300)


</script>
<template>
  <div class="chat-view">
    <ChatBoard :messages="messages" :smoothing="appSetting?.smoothing" :model="appSetting?.model"/>
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