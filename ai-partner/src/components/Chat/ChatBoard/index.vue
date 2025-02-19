<script lang="ts" setup>
import { nextTick, onMounted, ref } from 'vue'
import ChatMessage from '../ChatMessage/index.vue'
import type { MessageItem } from '../../../types/index'
import { toRefs } from 'vue';
let chatBody = ref<HTMLInputElement | null>(null)
import emitter from '../../../bus';
onMounted(() => {
  console.log('inputRef:', chatBody.value, typeof chatBody.value); // 打印出绑定的 DOM 元素
  emitter.on('scrollToBottom', () => {
    nextTick(() => chatBody.value?.scrollTo({ top: chatBody.value.scrollHeight - 500, behavior: 'smooth' }))

  })
});
const props = defineProps(
  {
    messages: {
      type: Array<MessageItem>,
      default: () => []
    }
  }
)

let { messages } = toRefs(props)




</script>
<template>
  <div class="chat-board" ref="chatBody">
    <div class="chat-body">
      <ChatMessage v-for="message in messages" :message="message" />
    </div>
  </div>
</template>


<style lang="less" scoped>
.chat-board {
  width: 100%;
  height: calc(100vh - 100px);
  margin-bottom: 50px;
  position: relative;

  scrollbar-width: thin;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #ddd3f9 transparent;

  .chat-body {
    display: flex;
    flex-direction: column;
    padding: 0 50px;
    gap: 20px;
  }
}
</style>