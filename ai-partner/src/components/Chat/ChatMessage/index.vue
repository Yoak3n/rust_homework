<template>
    <div :class="`message ${message.role === 'assistant' ? 'bot' : message.role === 'system-error' ? 'error' : 'user'}-message`">  
        <Bot v-if="message.role==='assistant' || message.role === 'system-error'" />
        <p class="message-text">
            <MarkdownRender :source="displayText" v-if="smoothing" />
            <MarkdownRender :source="message.content!" v-else class="native-render"/>
        </p>
    </div>
</template>

<script lang="ts" setup>
import { ref,onMounted,watch, onBeforeUnmount } from 'vue';
import type{ PropType} from 'vue'
import MarkdownRender from '../../Markdown/index.vue'
import type { MessageItem } from '../../../types/index'
import Bot from '../../Icon/Bot.vue'
import emitter from '../../../bus';

onMounted(async()=>{
    savedCallback.value = ()=>{
        // 检测定时器是否在运行，防止内存泄漏
        console.log('markdown render running')
        if (index <= props.message.content!.length){
                displayText.value += props.message.content!.charAt(index) 
                index ++
            }else{
                enableRunning.value = false
            }
    }
    if (props.message.role === 'assistant' && props.smoothing){
        if (!enableRunning.value){
            enableRunning.value = true
        }
    }else{
        displayText.value = props.message.content!
        messageText.value = props.message.content!
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
let messageText = ref<string>('')
let timerRef = ref<number|null>(null)
let savedCallback = ref<Function>(()=>{})
let enableRunning = ref<boolean>(false)
const props = defineProps({
    message: {
        type: Object as PropType<MessageItem>,
        required: true
    },
    smoothing: {
        type: Boolean,
        default: false
    }
})


watch(enableRunning,(n)=>{
    if (props.message.role !== 'assistant' ){
        displayText.value = props.message.content!
        
    }else{
        if (props.smoothing){
            if (n){
                timerRef.value = setInterval(savedCallback.value,50)
            }else{
                clearInterval(timerRef.value!)
            }
        }else{
            displayText.value = props.message.content!
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