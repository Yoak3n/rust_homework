<template>
    <div class="home-wrapper">
        <n-layout has-sider>
            <n-layout-sider
                content-style="display: flex; flex-direction: column; justify-content: space-between;overflow: hidden;"
                show-trigger :width="180" collapse-mode="width" bordered :collapsed-width="72" :collapsed="collapsed"
                @collapse="collapsed = true" @expand="collapsed = false">
                <n-menu v-model:value="activeKey" :collapsed="collapsed" :collapsed-width="72" :collapsed-icon-size="22"
                    :options="menuOptions" />
                <div class="option-btn" style="display: flex;flex-direction: column;">
                    <n-button text 
                    @click="()=>{modalKey = 'about';showModal = !showModal}"
                    style="font-size: 48px;margin-bottom:2rem" 
                    size="large" 
                    class="setting"
                        >
                        <template #icon>
                            <n-icon>
                                <ChatboxEllipsesOutline/>
                            </n-icon>
                        </template>
                    </n-button>
                    <n-button text style="font-size: 48px;margin-bottom:2rem" size="large" class="setting"
                        @click="()=>{modalKey = 'setting';showModal = !showModal}">
                        <template #icon>
                            <n-icon>
                                <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
                                    viewBox="0 0 512 512">
                                    <circle cx="256" cy="256" r="48" fill="currentColor"></circle>
                                    <path
                                        d="M470.39 300l-.47-.38l-31.56-24.75a16.11 16.11 0 0 1-6.1-13.33v-11.56a16 16 0 0 1 6.11-13.22L469.92 212l.47-.38a26.68 26.68 0 0 0 5.9-34.06l-42.71-73.9a1.59 1.59 0 0 1-.13-.22A26.86 26.86 0 0 0 401 92.14l-.35.13l-37.1 14.93a15.94 15.94 0 0 1-14.47-1.29q-4.92-3.1-10-5.86a15.94 15.94 0 0 1-8.19-11.82l-5.59-39.59l-.12-.72A27.22 27.22 0 0 0 298.76 26h-85.52a26.92 26.92 0 0 0-26.45 22.39l-.09.56l-5.57 39.67a16 16 0 0 1-8.13 11.82a175.21 175.21 0 0 0-10 5.82a15.92 15.92 0 0 1-14.43 1.27l-37.13-15l-.35-.14a26.87 26.87 0 0 0-32.48 11.34l-.13.22l-42.77 73.95a26.71 26.71 0 0 0 5.9 34.1l.47.38l31.56 24.75a16.11 16.11 0 0 1 6.1 13.33v11.56a16 16 0 0 1-6.11 13.22L42.08 300l-.47.38a26.68 26.68 0 0 0-5.9 34.06l42.71 73.9a1.59 1.59 0 0 1 .13.22a26.86 26.86 0 0 0 32.45 11.3l.35-.13l37.07-14.93a15.94 15.94 0 0 1 14.47 1.29q4.92 3.11 10 5.86a15.94 15.94 0 0 1 8.19 11.82l5.56 39.59l.12.72A27.22 27.22 0 0 0 213.24 486h85.52a26.92 26.92 0 0 0 26.45-22.39l.09-.56l5.57-39.67a16 16 0 0 1 8.18-11.82c3.42-1.84 6.76-3.79 10-5.82a15.92 15.92 0 0 1 14.43-1.27l37.13 14.95l.35.14a26.85 26.85 0 0 0 32.48-11.34a2.53 2.53 0 0 1 .13-.22l42.71-73.89a26.7 26.7 0 0 0-5.89-34.11zm-134.48-40.24a80 80 0 1 1-83.66-83.67a80.21 80.21 0 0 1 83.66 83.67z"
                                        fill="currentColor"></path>
                                </svg>
                            </n-icon>
                        </template>
                    </n-button>
                </div>
            </n-layout-sider>
            <n-layout-content>
                <router-view />
            </n-layout-content>

        </n-layout>
        <Modal v-model:show="showModal" :switch-callback="showModalFun" :modalKey="modalKey"/>
    </div>
</template>

<script lang="ts" setup>
import { ref, h, onMounted, onActivated } from 'vue'
import type { Component } from 'vue'
import {ChatboxEllipsesOutline} from '@vicons/ionicons5'
import { NLayout, NLayoutSider, NLayoutContent, NMenu, NIcon, NButton } from 'naive-ui'
import type { MenuOption } from 'naive-ui'
import Modal from '../components/Modal/index.vue'
let collapsed = ref<boolean>(true)
import Message from '../components/Icon/Message.vue'
import { RouterLink, useRoute } from 'vue-router'
let activeKey = ref<string>('')
const $route = useRoute()
const renderIcon = (icon: Component) => {
    return () => h(NIcon, null, { default: () => h(icon) })
}
let showModal = ref<boolean>(false)
const showModalFun = (v:boolean) => showModal.value = v
let modalKey = ref('')
onMounted(() => {
    activeKey.value = $route.name?.toString() || ''
})
onActivated(() => {
    activeKey.value = $route.name?.toString() || ''
})

const menuOptions: Array<MenuOption> = [
    {
        label: () =>
            h(
                RouterLink,
                {
                    to: '/chat'
                },
                { default: () => '对话' }
            )
        ,
        key: 'Chat',
        icon: renderIcon(Message)
    }
]
</script>

<style lang="less">
.home-wrapper {
    height: 100%;
    .n-layout {
        height: 100%;
    }
}
</style>