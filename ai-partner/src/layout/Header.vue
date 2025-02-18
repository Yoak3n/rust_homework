<script lang="ts" setup>
import {window} from '@tauri-apps/api'

const toggleFullscreen = () => {
    let w = window.Window.getCurrent()
    w.setFullscreen(true)
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
            <button>
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


<style scoped>
.header {
    position: fixed;
    top: 0;
    width: 100%;
    height: 30px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #fff;
    box-shadow: 1px 0px 5px 0px rgba(3, 3, 3, 0.3);
}

.title {
    width: 30px;

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
        background-color: transparent;
        border-radius: 5px;
        height: 100%;
        width: 30px;
        padding: auto;
        img{
            line-height: 30px;
            background: transparent;
        }
    }
}
</style>
