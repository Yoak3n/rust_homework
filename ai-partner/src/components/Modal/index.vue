<template>
    <n-modal :show="props.show" @update:show="()=>props.switchCallback(!show)" closable>
        <n-card>
            <SettingView :switchCallback="props.switchCallback" v-if="props.modalKey==='setting'" />
            <AboutView :switchCallback="props.switchCallback" v-if="props.modalKey==='about'" />
        </n-card>
    </n-modal>
</template>

<script lang="ts" setup>
import { NModal,NCard } from 'naive-ui';
import SettingView from './Setting.vue'
import AboutView from '../../components/Infomation/About/index.vue'
import { onMounted } from 'vue';
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
onMounted(()=>{
    console.log(props.modalKey)
})
</script>

<style scoped lang="less">

</style>