import { createApp } from "vue";
import App from "./App.vue";
import{Quasar} from 'quasar'

import "@quasar/extras/material-icons/material-icons.css"
import "quasar/dist/quasar.css"
import store from './store'
const app = createApp(App)
app.use(store)
app.use(Quasar,{})
app.mount("#app");
