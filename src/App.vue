<script setup lang="ts">
import { watch } from "vue";
import { Dark, setCssVar } from "quasar";
import { useSettingsStore } from "@/stores/settings";
import AppShell from "@/components/layout/AppShell.vue";

const settingsStore = useSettingsStore();

// Brand colors that mirror our --zen-* tokens for each scheme. These are
// pushed into Quasar via setCssVar so any Quasar components we use (q-btn
// color="primary", q-linear-progress color="info", etc.) match the rest of
// the app.
const BRAND_DARK = {
  primary: "#9e9e9e",
  secondary: "#89b4fa",
  accent: "#bdbdbd",
  positive: "#a6e3a1",
  negative: "#ef5350",
  info: "#89dceb",
  warning: "#fab387",
};

const BRAND_LIGHT = {
  primary: "#616161",
  secondary: "#1e66f5",
  accent: "#9e9e9e",
  positive: "#40a02b",
  negative: "#d20f39",
  info: "#04a5e5",
  warning: "#fe640b",
};

function applyScheme(scheme: "dark" | "light"): void {
  Dark.set(scheme === "dark");
  document.body.classList.toggle("zen-light", scheme === "light");
  const brand = scheme === "dark" ? BRAND_DARK : BRAND_LIGHT;
  for (const [name, value] of Object.entries(brand)) {
    setCssVar(name, value);
  }
}

// Sync color scheme with Quasar Dark + custom CSS variables
watch(
  () => settingsStore.settings.colorScheme,
  (scheme) => applyScheme(scheme),
  { immediate: true }
);

// Apply UI font size as CSS variable
watch(
  () => settingsStore.settings.uiFontSize,
  (size) => { document.documentElement.style.setProperty("--ui-font-size", `${size}px`); },
  { immediate: true }
);
</script>

<template>
  <AppShell />
</template>
