<script setup lang="ts">
import { ref } from 'vue';
import type { Bookmark } from '../../api/type';

const current = ref(1);
const props = defineProps({
    data: {
        type: Array<Bookmark>,
        required: true
    }
})
</script>

<template>
    <div>
        <q-card>
            <q-card-section v-for="item in props.data">
                {{ item }}
                <a href="https://www.baidu.com" target="_blank">
                    <q-chip clickable>
                        {{ item.name }}
                    </q-chip>
                </a>
            </q-card-section>

        </q-card>
        <q-pagination v-model="current" max="5" />
    </div>
</template>