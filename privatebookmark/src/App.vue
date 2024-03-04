<script setup lang="ts">
import {ref,reactive}  from 'vue'

const leftDrawerOpen = ref(true)
const toggleLeftDrawer = () => {
  leftDrawerOpen.value = !leftDrawerOpen.value
}

let tabData = reactive([{
  tab: 'mails',
  name:'邮箱'
}]
)


</script>

<template>
  <q-layout view="hHh lpR fFf">

    <q-header elevated class="bg-white text-blue-grey-10" height-hint="98">
      <q-toolbar>
        <q-btn dense flat round icon="menu" @click="toggleLeftDrawer" />

        <q-toolbar-title>
          <q-avatar>
            <img src="../src-tauri/icons/Square142x142Logo.png">
          </q-avatar>
          Bookmark
        </q-toolbar-title>
      </q-toolbar>

      <q-tabs align="left">
        <q-tab v-for="item in tabData"  :label="item.name" />
      </q-tabs>
    </q-header>

    <q-drawer 
    show-if-above 
    v-model="leftDrawerOpen" 
    side="left" 
    behavior="desktop" 
    bordered>
      <!-- drawer content -->
      123454
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>

  </q-layout>

</template>




<style scoped></style>
