<template>
    <div class="body">
        <RouterView/>
    </div>
</template>

<script lang="ts" setup>
import { computed, toRefs } from 'vue';
const props = defineProps({
    top: {
        type: Number,
        default: 30
    }
})
const {top}  =  toRefs(props)
const margin_top = computed(() => {
    return top.value + 'px'
})
</script>

<style scoped lang="less">
.body{
    margin-top: v-bind(margin_top);
    width: 100%;
    height: 100%;
    padding: 0 20px;
    box-sizing: border-box;
}
</style>