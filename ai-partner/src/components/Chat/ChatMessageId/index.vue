<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { MessageItem } from '../../../types';
import {listen} from '@tauri-apps/api/event'
import { registerNewListen } from '../../../bus';

let message = ref<MessageItem|null>(null)
let index = 0 
let isThinking = ref(false)
const props = defineProps({
    id: {
        type: Number,
        required: true
    }
})

interface Payload{
    id: number,
    message_type: string,
    index: number,
    data:string

}

onMounted(async() => {
    message.value!.timestamp = props.id
    const unlistenFn = await listen('stream-data', (event) => {
        const payload = event.payload as Payload
        if (payload.id != props.id) return
        index++
        if (payload.index != index) return
        if (payload.message_type == 'reasoning_content'){
            isThinking.value = true
            message.value!.reasoning_content += payload.data
        }else{
            isThinking.value = false
            message.value!.content += payload.data
        }
    })
    registerNewListen(props.id,unlistenFn)
})


</script>


<template>
    <div :class="`message ${message?.role == 'assistant' ? 'assistant' : ''}`">
        <div class="message-item">
            <div class="bot-avatar"></div>
            <div class="message-content">
                <div class="message-text" v-if="!isThinking">{{ message?.content }}</div>
                <div class="message-text" v-else>{{ message?.reasoning_content }}</div>
            </div>
        </div>
    </div>


</template>