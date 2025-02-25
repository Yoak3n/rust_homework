<script lang="ts" setup>
import Header from './Header.vue'
import Body from './Body.vue'
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
let heigth = 30
const $route = useRoute()
let name = ref($route.name)
onMounted(()=>{
})
</script>

<template>
    <div class="container">
        <Header :heigth="heigth" v-if="name!=='Dialog'"/>
        <Body :top="name!=='Dialog'? heigth:0"/>
    </div>
</template>