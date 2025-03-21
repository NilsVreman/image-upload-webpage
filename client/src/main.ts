import { createApp } from "vue";
import { createPinia } from "pinia";
import pageRouter from "./router";

import "./style.css";
import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia();
app.use(pinia);

app.use(pageRouter);

app.mount("#app");
