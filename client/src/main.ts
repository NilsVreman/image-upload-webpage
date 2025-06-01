import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import { definePreset } from "@primeuix/themes";
import Aura from "@primeuix/themes/aura";

// import "@/style.css";
import pageRouter from "@/router";
import HamburgerMenu from "@/components/ui/HamburgerMenu.vue";
import App from "@/App.vue";

const app = createApp(App);
const pinia = createPinia();

app.component("HamburgerMenu", HamburgerMenu);

const themePreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: "{emerald:50}",
      100: "{emerald:100}",
      200: "{emerald:200}",
      300: "{emerald:300}",
      400: "{emerald:400}",
      500: "{emerald:500}",
      600: "{emerald:600}",
      700: "{emerald:700}",
      800: "{emerald:800}",
      900: "{emerald:900}",
      950: "{emerald:950}",
    },
    colorScheme: {
      light: {
        primary: {
          color: "{primary.950}",
          contrastColor: "#ffffff",
          hoverColor: "{primary.900}",
          activeColor: "{primary.800}",
        },
        highlight: {
          background: "{primary.950}",
          focusBackground: "{primary.700}",
          color: "#ffffff",
          focusColor: "#ffffff",
        },
      },
      dark: {
        primary: {
          color: "{primary.50}",
          contrastColor: "{primary.950}",
          hoverColor: "{primary.100}",
          activeColor: "{primary.200}",
        },
        highlight: {
          background: "{primary.50}",
          focusBackground: "{primary.300}",
          color: "{primary.950}",
          focusColor: "{primary.950}",
        },
      },
    },
  },
});

app.use(PrimeVue, {
  theme: {
    preset: themePreset,
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
