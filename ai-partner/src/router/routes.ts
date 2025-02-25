import type { RouteRecordRaw } from 'vue-router'


const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
    redirect:'/chat',
    // 路由需要重新设计，看是否需要添加其他功能，如果不需要，则直接跳转chat页面
    children:[
      {
        path:'/chat',
        name:'Chat',
        component: () => import('../views/Chat.vue')
      }
    ]
  },{
    path: '/dialog',
    name: 'Dialog',
    component: () => import('../views/Dialog.vue'),
  }
]


export default routes