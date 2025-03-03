<script setup lang="ts">
import { onMounted, ref, PropType } from 'vue'
import { MessageItem } from '../../../types';
import { listen } from '@tauri-apps/api/event'

let messageUpdate = ref<MessageItem>()
let index = 0
let isThinking = ref(false)
const props = defineProps({
    message: {
        type: Object as PropType<MessageItem>,
        required: true
    },
})

interface Payload {
    id: number,
    message_type: string,
    index: number,
    data: string

}

onMounted(async () => {
    messageUpdate.value = props.message
    if (props.message.role == 'assistant') {
        const unlistenFnData = await listen('stream-data', (event) => {
            console.log(event);
            const payload = event.payload as Payload
            if (payload.id != props.message.timestamp) return
            index++
            if (payload.index != index) return
            if (payload.message_type == 'reasoning_content') {

                isThinking.value = true
                messageUpdate.value!.reasoning_content += payload.data
            } else {
                isThinking.value = false
                messageUpdate.value!.content += payload.data
            }
        })
        const unlistenEnd = await listen('stream-end', (event) => {
            const id = event.payload as number
            if (id != props.message.timestamp) return
            unlistenFnData()
            unlistenEnd()
            messageUpdate.value!.reasoning_content = props.message.reasoning_content
            messageUpdate.value!.content = props.message.content
            isThinking.value = false
        })
    }else{
        messageUpdate.value!.content = props.message.content
    }
})


</script>


<template>
    <div :class="`message ${message?.role == 'assistant' ? 'assistant' : ''}`">
        <div class="message-item">
            <div class="bot-avatar"></div>
            <div class="message-content">
                <div class="message-text" v-if="message.reasoning_content != ''">{{ messageUpdate?.reasoning_content }}</div>
                <div class="message-text">{{ messageUpdate?.content }}</div>
            </div>
        </div>
    </div>


</template>