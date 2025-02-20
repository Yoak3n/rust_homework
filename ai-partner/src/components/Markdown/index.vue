<template>
    <div style="render" v-html="markdown.render(source)" />
</template>

<script lang="ts" setup>
import MarkdownIt from 'markdown-it'
import MarkdownItHighlightjs from 'markdown-it-highlightjs'
import { onUpdated } from 'vue'

const markdown=  new MarkdownIt().use(MarkdownItHighlightjs)
defineProps({
    source:{
        type:String,
        default:''
    }
})
onUpdated(()=>{
    addCopyButtons()
})
const addCopyButtons = ()=>{
    const codeBlocks = document.querySelectorAll<HTMLElement>('pre code');
    if (!codeBlocks.length) return;
    codeBlocks.forEach((codeBlock) => {
        // 创建复制按钮
        if (codeBlock && !codeBlock.querySelector('.copy-button')){
            const copyButton = document.createElement('button');
            copyButton.className = 'copy-button';
            copyButton.textContent = '复制代码';

            // 将按钮插入到代码块容器中
            const pre = codeBlock.parentElement;
            if (!pre) return;

            pre.style.position = 'relative';
            pre.appendChild(copyButton);
            
            // 添加点击事件
            copyButton.addEventListener('click', () => {
            navigator.clipboard.writeText(codeBlock.innerText).then(() => {
            copyButton.textContent = '已复制';
            setTimeout(() => (copyButton.textContent = '复制代码'), 1500);
            }).catch(err => console.error('复制失败: ', err));
        });
        }
       
    });
    
}
</script>

<style lang="less" >
div{
    margin: 0;
    padding: 0;
    height: auto;

}

pre code {
    display: block;
    padding: 15px;
    background-color: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.5;
    color: #333;
}


.hljs-keyword {
    color: #d73a49; /* 红色 */
    font-weight: bold;
}

/* 字符串 */
.hljs-string {
    color: #032f62; /* 蓝色 */
}

/* 注释 */
.hljs-comment {
    color: #6a737d; /* 灰色 */
    font-style: italic;
}

/* 数字 */
.hljs-number {
    color: #005cc5; /* 蓝色 */
}

/* 函数名 */
.hljs-title {
    color: #6f42c1; /* 紫色 */
}

/* 标签名 */
.hljs-tag {
    color: #22863a; /* 绿色 */
}

/* 属性名 */
.hljs-attr {
    color: #e36209; /* 橙色 */
}

/* 其他 */
.hljs-built_in,
.hljs-selector-class,
.hljs-selector-id {
    color: #6f42c1; /* 紫色 */
}


</style>