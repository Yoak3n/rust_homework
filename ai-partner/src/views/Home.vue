<template>
    <div class="home-wrapper">
        <n-layout has-sider>
            <n-layout-sider show-trigger :width="180" collapse-mode="width" bordered :collapsed-width="64"
                :collapsed="collapsed" @collapse="collapsed = true" @expand="collapsed = false">
                <n-menu v-model:value="activeKey" :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22"
                    :options="menuOptions" />
            </n-layout-sider>
            <n-layout-content>
                <router-view />
            </n-layout-content>
        </n-layout>

    </div>
</template>

<script lang="ts" setup>
import { ref, h,onMounted, onActivated } from 'vue'
import type { Component } from 'vue'
import { NLayout, NLayoutSider, NLayoutContent, NMenu, NIcon } from 'naive-ui'
import type { MenuOption } from 'naive-ui'

let collapsed = ref<boolean>(true)
import Message from '../components/Icon/Message.vue'
import { RouterLink,useRoute } from 'vue-router'
let activeKey = ref<string>('')
const $route = useRoute()
const renderIcon = (icon: Component) => {
    return () => h(NIcon, null, { default: () => h(icon) })
}
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
                    to:'/chat'
                },
                { default: () => '对话' }
            )
        ,
        key: 'Chat',
        icon: renderIcon(Message)
    },
    {
        label: 'Table',
        key: 'table'
    }
]
</script>

<style scoped lang="less">
.home-wrapper {
    height: 100%;

    .n-layout {
        height: 100%;
    }
}
</style>