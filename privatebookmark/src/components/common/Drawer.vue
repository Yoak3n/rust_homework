<template>
    <q-list class="menu-list">
        <q-item clickable @click="click1" v-ripple>
            <q-item-section avatar>
                创建书签
            </q-item-section>
        </q-item>
        <q-item clickable @click="click2" v-ripple>
            <q-item-section avatar>
                读取书签
            </q-item-section>
        </q-item>
        <q-space />
        <q-item clickable @click="click2" v-ripple>
            <q-item-section avatar>
                创建分类
            </q-item-section>
        </q-item>
    </q-list>
    
</template>

<script setup lang="ts">
import { create_bookmark, read_bookmarks} from '../../api';

const click1 = () => {
  create_bookmark({ url: 'url', name: 'name', content: 'url' }).then(res => {
    console.log(res);
  })
}
const click2 = () => {
  read_bookmarks().then(res => {
    console.log(res);
  })
}
</script>