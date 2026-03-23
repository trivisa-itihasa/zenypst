import { createRouter, createWebHistory } from "vue-router";
import AppShell from "@/components/layout/AppShell.vue";

const router = createRouter({
  history: createWebHistory("/"),
  routes: [
    {
      path: "/",
      component: AppShell,
    },
  ],
});

export default router;
