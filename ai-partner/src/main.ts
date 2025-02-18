import { createApp } from "vue";
import App from "./App.vue";
import "./assets/style/variables.less"
const app = createApp(App)
import router from "./router";

app.use(router)
app.mount("#app");
