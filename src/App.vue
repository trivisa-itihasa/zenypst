<script setup lang="ts">
import { watch } from "vue";
import { useTheme as useVuetifyTheme } from "vuetify";
import { useSettingsStore } from "@/stores/settings";
import AppShell from "@/components/layout/AppShell.vue";

const settingsStore = useSettingsStore();
const vuetifyTheme = useVuetifyTheme();

// Sync Vuetify theme with settings
watch(
  () => settingsStore.settings.colorScheme,
  (scheme) => { vuetifyTheme.global.name.value = scheme; },
  { immediate: false }
);

// Apply UI font size as CSS variable
watch(
  () => settingsStore.settings.uiFontSize,
  (size) => { document.documentElement.style.setProperty("--ui-font-size", `${size}px`); },
  { immediate: true }
);
</script>

<template>
  <v-app :theme="settingsStore.settings.colorScheme">
    <AppShell />
  </v-app>
</template>
