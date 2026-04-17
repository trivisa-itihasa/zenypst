import { createApp } from "vue";
import { createPinia } from "pinia";
import { Quasar } from "quasar";
import App from "./App.vue";
import router from "./router";
import { quasarUserOptions } from "./plugins/quasar";
import i18n from "./plugins/i18n";
import "./assets/styles/global.scss";
import "pdfjs-dist/web/pdf_viewer.css";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(Quasar, quasarUserOptions);
app.use(i18n);

app.mount("#app");
