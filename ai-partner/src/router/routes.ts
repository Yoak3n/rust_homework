import type { RouteRecordRaw } from 'vue-router'


const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
    children:[
      {
        path:'/chat',
        name:'Chat',
        component: () => import('../views/Chat.vue')
      }
    ]
  
  },
//   {
//     path: '/about',
//     name: 'About',
//     component: () => import(/* webpackChunkName: "about" */ '../views/About.vue'),
//   }
]


export default routes