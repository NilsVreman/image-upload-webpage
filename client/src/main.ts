import { createApp } from "vue";
import { createPinia } from "pinia";

import "@/style.css";
import pageRouter from "@/routers";
import HamburgerMenu from "@/components/ui/HamburgerMenu.vue";
import App from "@/App.vue";

const app = createApp(App);
const pinia = createPinia();

app.component("HamburgerMenu", HamburgerMenu);

app.use(pinia);
app.use(pageRouter);

app.mount("#app");
