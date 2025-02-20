import type { RouteRecordRaw } from 'vue-router'


const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
    // 路由需要重新设计，看是否需要添加其他功能，如果不需要，则直接跳转chat页面
    children:[
      {
        path:'/chat',
        name:'Chat',
        component: () => import('../views/Chat.vue')
      },{
        path:'/setting',
        name:'Setting',
        component: () => import('../views/Setting.vue')
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