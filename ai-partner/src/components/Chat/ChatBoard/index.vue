<script lang="ts" setup>
import ChatMessage from '../ChatMessage/index.vue'
import type {MessageItem} from '../../../types/index'
import { toRefs } from 'vue';



const props = defineProps(
  {
    messages: {
      type: Array<MessageItem>,
      default: () => []
    }
  }
)

let {messages} =  toRefs(props)
</script>
<template>
  <div class="chat-board">
    <div class="chat-body">
      <ChatMessage v-for="message in messages"  :message="message" />
    </div>
  </div>
</template>


<style lang="less" scoped>
.chat-board {
  width: auto;
  height: 100%;
  margin-bottom: 50px;
  position: relative;
  background-color: bisque;
  padding: 0 20px;
  // .chat-body{
  //   width: 100%;
  //   overflow-y: auto;
  // }
}

</style>