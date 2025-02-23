<script lang="ts" setup>
import ChatForm from '../components/Chat/ChatBoard/index.vue'
import { NIcon } from 'naive-ui';
import {Reload,Send} from '@vicons/ionicons5'
import type {MessageItem} from '../types/index'
import { computed, ref } from 'vue';

const defaultMessages:Array<MessageItem> = [
  {role:'assistant', content:'你好，我是你的助手，有什么可以帮助你的吗？', text:'你好，我是你的助手，有什么可以帮助你的吗？'}
]
let messages = ref<Array<MessageItem>>(defaultMessages)
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
  generateBotResponseStream(messages.value)
}

import { querySetting } from '../api/db'
const generateBotResponseStream = async (history: Array<MessageItem>) => {
  const { base_url, key, model } = await querySetting()
  if (!base_url || !key || !model || base_url === "" || key === "" || model === "") {
    updateHistoryStream({ role: "system-error", content: "请先配置API密钥和模型", text: "请先配置API密钥和模型" })
    return
  }
  const requestOptions = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${key}`,
    },
    body: JSON.stringify({
      model,
      messages: history,
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
    let newAnswer: MessageItem = { role: "assistant", content: "", text: "" }
    while (true) {
      const { done, value } = await reader.read();
      if (done) break; // 流结束

      // 将二进制数据转换为文本
      decoder.decode(value, { stream: true }).split('\n').forEach(chunk => {
        if (chunk === '\n' || chunk === '' || chunk.includes('data: [DONE]')) return
        const jsonAnswer = chunk.replace('data:', '').replace('data: ', '')
        JSON.parse(jsonAnswer, (key, value) => {
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
const updateHistoryStream = (message: MessageItem) => {
  let newHistory: Array<MessageItem> = []
  if (messages.value[messages.value.length - 1].role === 'assistant' && message.role === 'assistant') {
    newHistory = [...messages.value.slice(0, messages.value.length - 1), message]
    messages.value = newHistory
  }else{
    messages.value.push(message)
  }

  emitter.emit('scrollToBottom')
  emitter.emit('updateHistory', message)
}

const resetHistory = () => {
  messages.value = [defaultMessages[0]]
}



</script>
<template>
  <div class="chat-view">
    <ChatForm :messages="messages"/>
    <form class="chat-input" @submit="(e) =>{
      e.preventDefault()
      submitUserMessage()
    }">
      <input type="text" placeholder="Type a message..." v-model="input"  required minlength="1"/>
      <button type="button" class="reset-btn" @click="resetHistory">
        <n-icon size="24">
          <Reload/>
        </n-icon>
      </button>
      <button type="submit" class="submit-btn" >
        <n-icon size="24">
          <Send/>
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
  background-color: #F7F8FC;
  box-sizing: border-box;
  input{
    height: 50px;
    width: 90%;
    border-radius: 5px;
    background-color: #F7F8FC;
    padding: 0 17px;
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
  &:invalid  .submit-btn{
    background-color: gray;
    cursor: not-allowed;
  }
  &:focus-within{
    outline: 2px solid #6d4fc2;
  }
}
</style>