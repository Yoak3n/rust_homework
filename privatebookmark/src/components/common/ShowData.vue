<script setup lang="ts">
import { ref } from 'vue';
import type { Bookmark } from '../../api/type';
import {search_bookmark} from '../../api/index'
const props = defineProps({
  data: {
    type: Array<Bookmark>,
    required: true
  }
})

let bookmark = ref<Bookmark>()

const edit_bookmark = async (id? :number) => {
  if (id) {
    bookmark.value = await search_bookmark(id)
    console.log(bookmark.value);
    return
  }
}

</script>

<template>
  <div>
    <q-card>
      <q-card-section v-for="item in props.data">
        <a href="https://www.baidu.com" target="_blank">
          <q-chip clickable>
            {{ item.name }}
          </q-chip>
        </a>
        {{ item }}
        <q-menu touch-position context-menu>
          <q-list dense style="min-width: 100px">
            <q-item clickable v-close-popup>
              <q-item-section>
                <q-button @click="edit_bookmark(item.id)">
                  Edit...
                </q-button>

              </q-item-section>
            </q-item>
            <q-separator />
            <q-item clickable>
              <q-item-section>Preferences</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item v-for="n in 3" :key="n" dense clickable>
                    <q-item-section>Submenu Label</q-item-section>
                    <q-item-section side>
                      <q-icon name="keyboard_arrow_right" />
                    </q-item-section>
                    <q-menu auto-close anchor="top end" self="top start">
                      <q-list>
                        <q-item v-for="n in 3" :key="n" dense clickable>
                          <q-item-section>3rd level Label</q-item-section>
                        </q-item>
                      </q-list>
                    </q-menu>
                  </q-item>
                </q-list>
              </q-menu>

            </q-item>
            <q-separator />
            <q-item clickable v-close-popup>
              <q-item-section>Quit</q-item-section>
            </q-item>
          </q-list>
        </q-menu>
      </q-card-section>
    </q-card>


  </div>

</template>