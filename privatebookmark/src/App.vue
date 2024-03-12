<script setup lang="ts">
import { ref, onMounted,computed, reactive } from 'vue'
import type { Category, Bookmark } from './api/type';
import {read_bookmarks } from './api';
import { useCategoryStore } from './store/modules/category';

import ShowData from './components/common/ShowData.vue';
import Drawer from './components/common/Drawer.vue';
const leftDrawerOpen = ref(false)
const categoryStore = useCategoryStore()
let activeTab = ref(0)
let miniState = ref(false)
const toggleLeftDrawer = () => {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
interface category {
  name: string,
  id: number
  data: Bookmark[]
}
let tabData: category[] = reactive([])

const initializeCategories = async() => {
  await categoryStore.read_categories()
  let res:Category[] = categoryStore.categories
  console.log(res);
  
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
    activeTab.value = tabData[tabData.length - 1].id
  })
})

let data = computed(() => {
  const  index = tabData.findIndex(c => c.id == activeTab.value)
  console.log(index);
  if (index >= 0){
    return tabData[index].data
  }
  return []
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

    <q-drawer 
    v-model="leftDrawerOpen" 
    side="right" 
    behavior="desktop" 
    class="bg-grey-1" 
    overlay 
    bordered 
    :width="150" 
    :mini="miniState"
    @mouseover="miniState = false"
    @mouseout="miniState = true"
    >
      <Drawer/>
    </q-drawer>

    <q-page-container>
      <ShowData v-model:data="data"/>
    </q-page-container>
  </q-layout>

</template>

<style scoped lang="scss">

</style>