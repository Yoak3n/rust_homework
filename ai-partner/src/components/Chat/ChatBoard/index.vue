<script lang="ts" setup>
import { nextTick,  onMounted, ref,onUnmounted,PropType} from 'vue'
import ChatMessage from '../ChatMessage/index.vue'
import type { MessageItem } from '../../../types/index'
import emitter from '../../../bus';
let chatBody = ref<HTMLInputElement | null>(null)

onMounted(() => {
  emitter.on('scrollToBottom', ()=>{
    scrollToBottom()
})
});
onUnmounted(() => {
  emitter.off('scrollToBottom')
})



const scrollToBottom = () => {
  nextTick(() => {
    if (chatBody.value) {
      chatBody.value.scrollTo({ top: chatBody.value.scrollHeight, behavior: 'smooth' });
    }
  });
};
let props = defineProps(
  {
    messages: {
      type: Array as PropType<MessageItem[]>,
      required: true
    },
    smoothing:{
      type:Boolean,
      default:false
    },
    model:{
      type:String,
      default:''
    }
  }
)

// watch(
//   ()=>props.messages,
//   (newValue) => {
//     console.log("watch in ChatBoard",newValue);
//   },{
//     deep:true
//   }
// )
</script>
<template>
  <div class="chat-board" ref="chatBody">
    <h1>{{model}}</h1>
    <div class="chat-body">
      <ChatMessage v-for="m in props.messages" :message="m" :smoothing="smoothing" :key="m.timestamp"/>
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
  @media screen and (max-width: 768px) {
    .chat-body{
      padding:0 ;
    }
  }
}
</style>