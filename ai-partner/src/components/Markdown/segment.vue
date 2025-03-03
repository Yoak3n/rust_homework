<script setup lang="ts">
// 使用组合式API
import { ref, watch, nextTick } from 'vue'
import MarkdownIt from 'markdown-it'
import type { Segment } from '../../types'
import { parseInWorker, simpleHash } from './utils'
const md = new MarkdownIt()

const rawText = ref('')
const segments = ref<Array<Segment>>([]) // 存储分段数据
const renderQueue = ref<Array<Segment>>([]) // 渲染队列
let isRendering = false



// 带缓存的分段处理器
const processSegments = (newText: string) => {
    const newSegments = newText.split(/(?:\n\s*){2,}/g) // 按两个以上换行符分割
    const oldSegments = segments.value
    // 差异对比算法
    for (let i = 0; i < newSegments.length; i++) {
        if (!oldSegments[i] || oldSegments[i].raw !== newSegments[i]) {
            renderQueue.value.push({
                index: i,
                raw: newSegments[i],
                hash: simpleHash(newSegments[i]),
            })
        }
    }

    segments.value = newSegments.map((s, i) => ({
        raw: s,
        hash: simpleHash(s),
        html: oldSegments[i]?.hash === simpleHash(s) ? oldSegments[i].html : ''
    }))
}



// 使用异步队列渲染
const processRenderQueue = async () => {
    if (isRendering || !renderQueue.value.length) return

    isRendering = true
    const MAX_CHUNK = 3 // 每批渲染3个段落

    while (renderQueue.value.length) {
        const chunk = renderQueue.value.splice(0, MAX_CHUNK)

        await Promise.all(chunk.map(async ({ index, raw }) => {
            // Web Worker解析（可选）
            const html = await parseInWorker(raw)
            segments.value[index!].html = html
        }))

        // 等待Vue更新
        await nextTick()
        // 控制渲染节奏
        await new Promise(r => requestAnimationFrame(r))
    }

    isRendering = false
}
</script>


<template>
    <div v-for="(segment, index) in segments" :key="index">
        <div v-html="segment.html">
        </div>
    </div>
</template>
