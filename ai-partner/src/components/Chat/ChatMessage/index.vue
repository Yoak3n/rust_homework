<template>
    <div :class="`message ${message.role === 'assistant' ? 'bot' : message.role === 'system-error' ? 'error' : 'user'}-message`">  
        <Bot v-if="message.role==='assistant' || message.role === 'system-error'" />
        <p class="message-text">
            <MarkdownRender :source="displayText"/>
        </p>
    </div>
</template>

<script lang="ts" setup>
import { ref, toRefs,onMounted,watch, onBeforeUnmount } from 'vue';
import type{ PropType} from 'vue'
import MarkdownRender from '../../Markdown/index.vue'
import type { MessageItem } from '../../../types/index'
import Bot from '../../Icon/Bot.vue'
import emitter from '../../../bus';
onMounted(()=>{
    savedCallback.value = ()=>{
        // 检测定时器是否在运行，防止内存泄漏
        console.log('running')
        if (index <= message.value.content!.length){
                displayText.value += message.value.content!.charAt(index) 
                index ++
            }else{
                enableRunning.value = false
            }
    }
    if (message.value.role === 'assistant'){
        if (!enableRunning.value){
            enableRunning.value = true
        }
    }else{
        displayText.value = message.value.content!
    }
    emitter.on('updateHistory',(res:any)=>{
        if (res.role === 'assistant'){
            if (!enableRunning.value){
                enableRunning.value = true
            }
        }
    })
})
onBeforeUnmount(()=>{
    clearInterval(timerRef.value!)
    emitter.off('updateHistory')
})
let index = 0;
let displayText = ref<string>('')
let timerRef = ref<number|null>(null)
let savedCallback = ref<Function>(()=>{})
let enableRunning = ref<boolean>(false)
const props = defineProps({
    message: {
        type: Object as PropType<MessageItem>,
        required: true
}})

let { message } = toRefs(props)
watch(enableRunning,(n)=>{
    if (message.value.role !== 'assistant' ){
        displayText.value = message.value.content!
    }else{
        if (n){
            timerRef.value = setInterval(savedCallback.value,50)
        }else{
            clearInterval(timerRef.value!)
        }
    }
    return ()=>clearInterval(timerRef.value!)
})


</script>

<style scoped lang="less">
.message{
    display: flex;
    align-items: center;
    gap: 11px;
}
.bot-message svg{
    height: 35px;
    width: 35px;
    padding: 6px;
    flex-shrink: 0;
    fill: #fff;
    border-radius: 50%;

    align-self: flex-start;
}

.message .message-text{
    padding: 12px 16px;
    max-width: 75%;
    word-wrap: break-word;

    font-size: 0.9rem;
}
.bot-message .message-text {
    background-color: #f6f2ff;
    border-radius: 13px 13px 13px 3px;
    white-space: normal;
}
.error-message .message-text {
    color: #f00;
    background-color: #f6f2ff;
    border-radius: 13px 13px 13px 3px;
}
.user-message{
    flex-direction: column;
    align-items: flex-end;
}
.user-message .message-text {
    color: #fff;
    background-color: #6d4fc2;
    border-radius: 13px 13px 3px 13px;
    div{
        white-space: normal;
    }
}
</style>