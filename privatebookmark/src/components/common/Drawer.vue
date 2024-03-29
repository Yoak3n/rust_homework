<template>
  <div>
    <q-list class="menu-list">
      <q-item clickable @click="click1" v-ripple>
        <q-item-section avatar>
          <q-icon name="inbox" />
        </q-item-section>

        <q-tooltip class="text-body2" self="center right" anchor="center left" :offset="[10, 10]">创建书签</q-tooltip>
      </q-item>
      <q-item clickable @click="click2" v-ripple>
        <q-item-section avatar>
            <q-icon name="fa-regular fa-plus"/>
        </q-item-section>
        <q-tooltip class="text-body2" self="center right" anchor="center left" :offset="[10, 10]">创建分类</q-tooltip>
      </q-item>
      <!-- <q-item clickable @click="click2" v-ripple>
        <q-item-section avatar>
          创建分类
        </q-item-section>
      </q-item> -->
    </q-list>
    <q-dialog v-model="showCreateBookmark">
      <q-card>
        <q-toolbar>
          <q-avatar>
            <img src="https://cdn.quasar.dev/logo-v2/svg/logo.svg">
          </q-avatar>
          <q-toolbar-title><span class="text-weight-bold">Create</span>Bookmark</q-toolbar-title>
          <q-btn flat round dense icon="close" v-close-popup />
        </q-toolbar>

        <q-card-section style="max-width: 400px">
          <q-form class="q-gutter-md" >
            <q-input filled label="书签名" v-model="data.name" />
            <q-input filled label="链接" v-model="data.url" />
            <q-input filled label="描述" v-model="data.content" />
            <q-select v-model="option" :options="categories" filled />
            <div class="row items-center">
              <q-btn class="col" label="Submit" @click="submit_bookmark"></q-btn>
              <!-- <q-btn label="重置" @click="click_category"></q-btn> -->
            </div>
          </q-form>
        </q-card-section>
      </q-card>

    </q-dialog>
  </div>


</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue';
import { useCategoryStore } from '../../store/modules/category';
import { create_bookmark, read_bookmarks } from '../../api';
import type { Bookmark } from '../../api/type';

const categoryStore = useCategoryStore()
let showCreateBookmark = ref(false)
let data = reactive<Bookmark>({ name: '', url: '', content: '' })
let option = ref({ label: '未分类', value: 0 })
let computed_url = computed(() => {
  let tmp = data.url
  if (!tmp.startsWith('http')) {
    tmp = 'https://' + data.url
  }
  return tmp
})

const submit_bookmark = () => {
  let record: Bookmark = { name: data.name, url: computed_url.value, content: data.content, category_id: option.value.value }
  create_bookmark(record).then(res => {
    if (res > 0) {
      showCreateBookmark.value = false
    }
  })
}
const categories = computed(() => {
  return categoryStore.categories.map((item) => {
    return { label: item.name, value: item.id }
  })
})
// const click_category = () => {
//   let record: Bookmark = { name: data.name, url: computed_url.value, content: data.content, category_id: option.value.value }
//   console.log(record);
// }



const click1 = () => {
  categoryStore.read_categories()
  showCreateBookmark.value = true
}
const click2 = () => {
  read_bookmarks().then(res => {
    console.log(res);
  })
}
</script>