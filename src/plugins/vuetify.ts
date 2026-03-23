import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";

import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi";

export default createVuetify({
  icons: {
    defaultSet: "mdi",
    aliases,
    sets: { mdi },
  },
  theme: {
    defaultTheme: "dark",
    themes: {
      dark: {
        dark: true,
        colors: {
          primary: "#cba6f7",
          secondary: "#89b4fa",
          accent: "#f5c2e7",
          error: "#f38ba8",
          warning: "#fab387",
          info: "#89dceb",
          success: "#a6e3a1",
          background: "#1e1e2e",
          surface: "#181825",
          "surface-variant": "#313244",
          "on-surface": "#cdd6f4",
          "on-background": "#cdd6f4",
          "on-primary": "#1e1e2e",
        },
      },
      light: {
        dark: false,
        colors: {
          primary: "#8839ef",
          secondary: "#1e66f5",
          accent: "#ea76cb",
          error: "#d20f39",
          warning: "#fe640b",
          info: "#04a5e5",
          success: "#40a02b",
          background: "#eff1f5",
          surface: "#e6e9f0",
          "surface-variant": "#dce0e8",
          "on-surface": "#4c4f69",
          "on-background": "#4c4f69",
          "on-primary": "#eff1f5",
        },
      },
    },
  },
  defaults: {
    VBtn: { variant: "text" },
    VCard: { elevation: 2 },
  },
});
