<template>
    <n-config-provider :theme-overrides="modalThemeOverrides">
        <n-modal :show="props.show" @update:show="()=>props.switchCallback(!show)" >
            <n-card size="large">
                <SettingView :switchCallback="props.switchCallback" v-if="props.modalKey==='setting'" />
                <About :switchCallback="props.switchCallback" v-if="props.modalKey==='about'" />
            </n-card>
        </n-modal>
    </n-config-provider>
</template>

<script lang="ts" setup>
import { NModal,NCard,NConfigProvider } from 'naive-ui';
import SettingView from './Setting.vue'
import About from '../Information/About.vue'
import { modalThemeOverrides } from '../../utils';

const props = defineProps({
    switchCallback:{
        type:Function,
        required:true
    },
    show:{
        type:Boolean,
        default:false
    },
    modalKey:{
        type:String,
        default:''
    }
})
</script>

<style scoped lang="less">

</style>