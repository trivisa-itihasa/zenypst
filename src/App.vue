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
  (scheme) => {
    vuetifyTheme.global.name.value = scheme;
  },
  { immediate: false }
);
</script>

<template>
  <v-app :theme="settingsStore.settings.colorScheme">
    <AppShell />
  </v-app>
</template>
