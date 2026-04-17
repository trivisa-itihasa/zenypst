<script setup lang="ts">
import { watch } from "vue";
import { Dark, setCssVar } from "quasar";
import { useSettingsStore } from "@/stores/settings";
import { useTheme } from "@/composables/useTheme";
import type { Theme } from "@/types";
import AppShell from "@/components/layout/AppShell.vue";

const settingsStore = useSettingsStore();
const { activeTheme } = useTheme();

function hexToRgb(value: string): string {
  // Handles "#rrggbb" only — rgba(...) inputs are passed through unchanged via the var itself.
  const h = value.replace("#", "");
  if (h.length !== 6) return value;
  const r = parseInt(h.substring(0, 2), 16);
  const g = parseInt(h.substring(2, 4), 16);
  const b = parseInt(h.substring(4, 6), 16);
  return `${r}, ${g}, ${b}`;
}

function applyTheme(theme: Theme): void {
  Dark.set(theme.isDark);

  const c = theme.colors;
  const root = document.documentElement.style;

  root.setProperty("--zen-background", c.appBackground);
  root.setProperty("--zen-surface", c.surface);
  root.setProperty("--zen-surface-variant", c.surfaceVariant);
  root.setProperty("--zen-border", c.border);
  root.setProperty("--zen-on-surface", c.uiText);
  root.setProperty("--zen-on-surface-rgb", hexToRgb(c.uiText));
  root.setProperty("--zen-on-surface-medium", c.uiTextMuted);
  root.setProperty("--zen-on-surface-disabled", c.uiTextMuted);
  root.setProperty("--zen-primary", c.primary);
  root.setProperty("--zen-primary-rgb", hexToRgb(c.primary));
  root.setProperty("--zen-secondary", c.info);

  root.setProperty("--zen-statusbar", c.statusBar);
  root.setProperty("--zen-statusbar-text", c.statusBarText);

  root.setProperty("--zen-error", c.error);
  root.setProperty("--zen-error-rgb", hexToRgb(c.error));
  root.setProperty("--zen-warning", c.warning);
  root.setProperty("--zen-warning-rgb", hexToRgb(c.warning));
  root.setProperty("--zen-info", c.info);
  root.setProperty("--zen-info-rgb", hexToRgb(c.info));
  root.setProperty("--zen-success", c.success);

  // Mirror brand colors into Quasar so q-btn color="primary", q-linear-progress
  // color="info" etc. match the rest of the app.
  setCssVar("primary", c.primary);
  setCssVar("secondary", c.info);
  setCssVar("accent", c.uiTextMuted);
  setCssVar("positive", c.success);
  setCssVar("negative", c.error);
  setCssVar("info", c.info);
  setCssVar("warning", c.warning);
}

watch(activeTheme, applyTheme, { immediate: true });

watch(
  () => settingsStore.settings.uiFontSize,
  (size) => { document.documentElement.style.setProperty("--ui-font-size", `${size}px`); },
  { immediate: true }
);
</script>

<template>
  <AppShell />
</template>
