<script lang="ts" setup>
import Header from './Header.vue'
import Body from './Body.vue'
let heigth = 30
</script>

<template>
    <div class="container">
        <Header :heigth="heigth"/>
        <Body :top="heigth"/>
    </div>
</template>