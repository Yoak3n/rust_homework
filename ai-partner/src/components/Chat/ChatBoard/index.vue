<script lang="ts" setup>
import { nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import ChatMessage from '../ChatMessage/index.vue'
import type { MessageItem } from '../../../types/index'
import { toRefs } from 'vue';
let chatBody = ref<HTMLInputElement | null>(null)
import { querySetting } from '../../../api/db';
import emitter from '../../../bus';
onMounted(() => {
  querySetting().then((res) => {
    if (res) {
      model_name.value = res.model
    }
  })
  emitter.on('scrollToBottom', () => {
    nextTick(() => chatBody.value?.scrollTo({ top: chatBody.value.scrollHeight - 500, behavior: 'smooth' }))
  })
});
onBeforeUnmount(() => {
  emitter.off('scrollToBottom')
})

let model_name = ref('')
const props = defineProps(
  {
    messages: {
      type: Array<MessageItem>
    }
  }
)

let { messages } = toRefs(props)




</script>
<template>
  <div class="chat-board" ref="chatBody">
    <h1>{{model_name}}</h1>
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