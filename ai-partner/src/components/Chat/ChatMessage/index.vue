<template>
    <div :class="`message ${message.role === 'assistant' ? 'bot' : message.role === 'system-error' ? 'error' : 'user'}-message`">
        {message.role !=='user' && <BotIcon/>}
        <p className="message-text">{{displayText}}</p>
    </div>
</template>

<script lang="ts" setup>
import { ref, toRefs,onMounted,watch } from 'vue';
import type{ PropType} from 'vue'
import type { MessageItem } from '../../../types/index'

onMounted(()=>{
    savedCallback.value = ()=>{
        if (index < message.value.content!.length){
                index ++
                displayText.value = message.value.content!.slice(0,index)
            }else{
                enableRunning.value = false
            }
    }
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
    if (message.value.role === 'assistant' ){
        displayText.value = message.value.content!
    }else{
        if (n){
            timerRef.value = setInterval(savedCallback.value,100)
        }else{
            clearInterval(timerRef.value!)
        }
    }
    return ()=>clearInterval(timerRef.value!)
})

watch(message,(n)=>{
    if (n.role === 'assistant'){
        if (!enableRunning.value){
            enableRunning.value = true
        }else{
            displayText.value = n.content!
        }
    }
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
    margin-bottom: 2px;
    align-self: flex-end;
}

.message .message-text{
    padding: 12px 16px;
    max-width: 75%;
    word-wrap: break-word;
    white-space: pre-line;
    font-size: 0.9rem;
}
.bot-message .message-text {
    background-color: #f6f2ff;
    border-radius: 13px 13px 13px 3px;
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
}
</style>