import { definePreset } from "@primeuix/themes";
import Aura from "@primeuix/themes/aura";

const Noir = definePreset(Aura, {
  semantic: {
    primary: {
      50: "{lime.50}",
      100: "{lime.100}",
      200: "{lime.200}",
      300: "{lime.300}",
      400: "{lime.400}",
      500: "{lime.500}",
      600: "{lime.600}",
      700: "{lime.700}",
      800: "{lime.800}",
      900: "{lime.900}",
      950: "{lime.950}",
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

export default Noir;
