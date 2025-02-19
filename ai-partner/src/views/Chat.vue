<script lang="ts" setup>
import ChatForm from '../components/Chat/ChatBoard/index.vue'
import { NIcon } from 'naive-ui';
import type {MessageItem} from '../types/index'
import { computed, ref } from 'vue';

let messages = ref<Array<MessageItem>>([
  {
    role:'assistant',
    content:'Hello, how can I help you today?',
    text: 'Hello, how can I help you today?',
  }
])
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
  input.value = ''
  setMessage(m)
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
  if (messages.value[messages.value.length - 1].role === 'assistant') {
    newHistory = [...messages.value.slice(0, messages.value.length - 1), message]
  }else{
    newHistory = [...messages.value,message]
  }
  messages.value = newHistory
  emitter.emit('scrollToBottom')
}





</script>
<template>
  <div class="chat-view">
    <ChatForm :messages="messages"/>
    <form class="chat-input" @submit="(e) =>{
      e.preventDefault()
      submitUserMessage()
    }">
      <input type="text" placeholder="Type a message..." v-model="input" />
      <button type="submit" v-on:invalid="input.trim() == ''">
        <n-icon :size="24">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-send"><line x1="22" y1="2" x2="11" y2="13"></line><polygon points="22 2 15 22 11 13 2 9 22 2"></polygon></svg>
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
  &:valid ~ button{
    display: block;
  
  }
  &:focus-within{
    outline: 2px solid #6d4fc2;
  }
  button{
    height: v-bind(inputHeigthString);
    width: 10%;
    border-radius: 5px;
    background-color: #6d4fc2;
    cursor: pointer;
    color: #fff;
    &:hover{
      background-color: #593bab;
      border: none;
      outline: none;
    }
    &:active{
      border: none;
    }
  }
}
</style>