import { createApp } from "vue";
import { createPinia } from "pinia";
import pageRouter from "./router";
import HamburgerDrawer from "@/components/ui/HamburgerDrawer.vue";

import "./style.css";
import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia();

app.component("HamburgerDrawer", HamburgerDrawer);

app.use(pinia);
app.use(pageRouter);

app.mount("#app");
