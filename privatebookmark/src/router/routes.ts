import {RouteRecordRaw} from 'vue-router'


const routes:RouteRecordRaw[] = [
    {
        path:'/',
        component:()=>import('../views/HomeView.vue')
    },
]

export default routes