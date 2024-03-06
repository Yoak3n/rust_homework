<script setup lang="ts">
import { ref, reactive, onMounted,computed } from 'vue'
import type { Category, Bookmark } from './api/type';
import {read_bookmarks, read_categories } from './api';

import ShowData from './components/common/ShowData.vue';
import Drawer from './components/common/Drawer.vue';
const leftDrawerOpen = ref(false)
let activeTab = ref(0)
const toggleLeftDrawer = () => {
  console.log(leftDrawerOpen.value);
  leftDrawerOpen.value = !leftDrawerOpen.value
}

interface category {
  name: string,
  id: number
  data: Bookmark[]
}
let tabData: category[] = reactive([{
      name: "未分类",
      id: 0,
      data: []
    } ])

const initializeCategories = async() => {
  let res:Category[] = await read_categories()
  for (const item of res) {
    const c: category = {
      name: item.name,
      id: item.id as number,
      data: []
    }
    tabData.push(c)
  }
}

onMounted(async() => {
  await initializeCategories()
  read_bookmarks().then(res => {
    console.log(res);
    res.forEach(item => {
      if (item.category_id == undefined) {
        tabData[0].data.push(item)
      }else{
        const index = tabData.findIndex(c => c.id == item.category_id)
        tabData[index].data.push(item)
      }
    })
    console.log(tabData);
    const lenght = tabData.length
    activeTab.value = tabData[lenght - 1].id
  })
})



let data = computed(() => {
  const  index = tabData.findIndex(c => c.id == activeTab.value)
  return tabData[index].data
})



</script>

<template>
  <q-layout view="hHh lpR fFf">

    <q-header elevated class="bg-white text-blue-grey-10" height-hint="98" >
      <q-toolbar>
        <q-toolbar-title>
          <q-avatar>
            <img src="../src-tauri/icons/Square142x142Logo.png">
          </q-avatar>
          Bookmark
        </q-toolbar-title>
        <q-space />
        <q-btn dense flat round icon="menu" @click="toggleLeftDrawer" />
      </q-toolbar>

      <q-tabs align="left" v-model="activeTab" >
        <q-tab v-for="item in tabData" :key="item.id" :label="item.name" :name="item.id" />
      </q-tabs>
    </q-header>

    <q-drawer v-model="leftDrawerOpen" side="right" behavior="desktop" class="bg-grey-1" overlay bordered>
      <!-- drawer content -->
      <Drawer/>
    </q-drawer>

    <q-page-container>
      <ShowData v-model:data="data"/>
    </q-page-container>
  </q-layout>

</template>

<style scoped lang="scss">

</style>
