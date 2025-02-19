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

const setMessage = (mi:MessageItem) => {
  messages.value.push(mi)
}

const inputHeigth = 50;
const inputHeigthString = computed(() => `${inputHeigth}px`)
const submitUserMessage = () => {
  if(input.value.trim() == '') return
  const m:MessageItem = {role:'user', content:input.value, text:input.value}
  input.value = ''
  setMessage(m)
}
</script>
<template>
    <ChatForm :messages="messages"/>
    <form class="chat-input" @submit="(e) =>{
      e.preventDefault()
      submitUserMessage()
    }">
      <input type="text" placeholder="Type a message..." v-model="input"/>
      <button type="submit">
        <n-icon :size="24">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-send"><line x1="22" y1="2" x2="11" y2="13"></line><polygon points="22 2 15 22 11 13 2 9 22 2"></polygon></svg>
        </n-icon>
      </button>
    </form>
</template>

<style scoped>
.chat-input{
  position: absolute;
  bottom: 0;
  width: 100%;
  height: v-bind(inputHeigthString);
  display: flex;
  input{
    height: v-bind(inputHeigthString);
    width: 90%;
    border-radius: 5px;
  }
  button{
    height: v-bind(inputHeigthString);
    width: 10%;
    border-radius: 5px;
    background-color: #6d4fc2;
    cursor: pointer;
    transition: background-color 0.1s ease-in-out;
    color: #f6f2ff;
    &:hover{
      background-color: #f6f2ff;
      border: none;
      outline: none;
      color: #6d4fc2;
    }
    &:active{
      border: none;
    }
  }
}
</style>