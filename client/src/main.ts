import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";

import "@/style.css";
import pageRouter from "@/routers";
import HamburgerMenu from "@/components/ui/HamburgerMenu.vue";
import Noir from "@/presets/Noir";
import App from "@/App.vue";

const app = createApp(App);
const pinia = createPinia();

app.component("HamburgerMenu", HamburgerMenu);

app.use(PrimeVue, {
  theme: {
    preset: Noir,
    options: {
      prefix: "p",
      darkModeSelector: ".p-dark",
      cssLayer: false,
    },
  },
});
app.use(pinia);
app.use(pageRouter);

app.mount("#app");
