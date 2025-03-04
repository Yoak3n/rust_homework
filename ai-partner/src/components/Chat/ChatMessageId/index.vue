<script setup lang="ts">
import { onMounted, ref, PropType } from 'vue'
import { listen } from '@tauri-apps/api/event'

import { MessageItem } from '../../../types';
import emitter from '../../../bus';
import { useAppStore } from '../../../store';

import MarkdownRender from '../../Markdown/index.vue'

let messageUpdate = ref<MessageItem>(
    {
        role: '',
        timestamp: 0,
        content: '',
        reasoning_content: ''
    }
)
let index = 0
let isThinking = ref(false)
const props = defineProps({
    message: {
        type: Object as PropType<MessageItem>,
        required: true
    },
})
const $AppStore = useAppStore()

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
            const payload = event.payload as Payload
            if (payload.id != props.message.timestamp) return
            index++
            if (payload.index != index) return
            emitter.emit('scrollToBottom')
            if (payload.message_type == 'reasoning_content') {
                isThinking.value = true
                $AppStore.setGenerating(true)
                messageUpdate.value!.reasoning_content += payload.data
            } else if (payload.message_type == 'content') {
                isThinking.value = false
                $AppStore.setGenerating(true)
                messageUpdate.value!.content += payload.data
            } else {
                unlistenFnData()
                $AppStore.setGenerating(false)
                isThinking.value = false
            }
        })
    } else {
        messageUpdate.value!.content = props.message.content
        $AppStore.setGenerating(false)
    }
})


</script>


<template>
    <div
        :class="`message ${message?.role == 'assistant' ? 'assistant' : message?.role == 'user' ? 'user' : message?.role == 'system' ? 'system' : ''}`">
        <div class="message-item">
            <img class="bot-avatar" v-if="message?.role == 'assistant' || message?.role == 'system'"
                src="../../../assets/bot.svg"></img>
            <div class="output">
                <div class="thinking">
                    <div class="reasoning-text" v-if="messageUpdate.reasoning_content != ''&& messageUpdate.reasoning_content != null &&messageUpdate.reasoning_content != undefined">
                        {{messageUpdate?.reasoning_content }}
                    </div>
                </div>
                <div class="message-content" v-if="messageUpdate?.content != ''">
                    <div class="message-text">
                        <MarkdownRender :source="messageUpdate?.content" />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="less" scoped>
.message {
    display: flex;
    align-items: center;
    gap: 11px;

    .message-item {
        max-width: 75%;
        .output {
            .thinking {
                .reasoning-text {
                    padding: 12px 10px;
                    position: relative;
                    font-size: 0.8rem;
                    white-space: pre-line;
                    color: #555;
                }
            }
            .message-content {
                background-color: #f6f2ff;
                .message-text {
                    padding: 12px 16px;
                    position: relative;
                    font-size: 0.9rem;
                    white-space: normal;
                }
            }
        }
    }

    &.assistant {
        .message-item {
            .message-content {
                border-radius: 13px 13px 13px 3px;
            }
        }
    }
}

.user {
    flex-direction: row-reverse;

    .message-item {
        flex-direction: row-reverse;
    }
}

.message-item {
    display: flex;
    align-items: flex-start;
    gap: 11px;

    .bot-avatar {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: #593BAB;
    }
}
</style>