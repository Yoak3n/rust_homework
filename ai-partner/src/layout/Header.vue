<script lang="ts" setup>
import { computed, toRefs } from 'vue'
const props = defineProps({
    heigth:{
        type: Number,
        default: 30
    }
})
const {heigth} = toRefs(props)
const heightString  = computed(() => `${heigth.value}px`)
import {window} from '@tauri-apps/api'

const minimize = () => {
    let w = window.Window.getCurrent()
    w.minimize()
}
const toggleFullscreen = async() => {
    let w = window.Window.getCurrent()
    const isFullscreen = await w.isFullscreen()
    w.setFullscreen(!isFullscreen)
}
const close = () => {
    let w = window.Window.getCurrent()
    w.close()
}

</script>
<template>
    <div class="header" data-tauri-drag-region>
        <div class="title">
            <img src="../assets/tauri.svg" alt="tauri" width="20" height="20">
        </div>
        <div class="option">
            <button @click="minimize">
                <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
            </button>
            <button @click="toggleFullscreen"> 
                <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" /></button>
            <button @click="close">
                <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </button>
        </div>
    </div>

</template>


<style scoped lang="less">
.header {
    position: fixed;
    top: 0;
    width: 100%;
    height: v-bind(heightString);
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #fff;
    background-color: #fff;
    box-shadow: 1px 0px 5px 0px rgba(3, 3, 3, 0.3);
    z-index: 9999;
}

.title {
    width: 30px;
    padding-left: .5rem;
    img{
        padding-top: 5px;
        line-height: 30px;
        background: transparent;
    }
}

.option {
    height: 100%;
    display: flex;
    button {
        cursor: pointer;
        background-color: transparent;
        border-radius: 5px;
        height: 100%;
        width: 30px;
        padding: auto;
        &:hover{
            background-color: #eee;
        }
        img{
            line-height: 30px;
            background: transparent;
        }
    }
}
</style>
