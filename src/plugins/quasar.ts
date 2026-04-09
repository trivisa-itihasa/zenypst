import { Dark, Notify } from "quasar";
import "@quasar/extras/mdi-v7/mdi-v7.css";
import "quasar/src/css/index.sass";

/**
 * User options for the Quasar plugin.
 *
 * Notify is enabled to replace Vuetify's `v-snackbar`. Dark is enabled so the
 * app can imperatively switch dark/light mode in sync with the user setting.
 */
export const quasarUserOptions = {
  plugins: {
    Dark,
    Notify,
  },
  config: {
    dark: true,
    notify: {
      position: "bottom" as const,
      timeout: 4000,
    },
  },
};
