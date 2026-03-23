<script setup lang="ts">
import { useSettingsStore } from "@/stores/settings";

const settingsStore = useSettingsStore();

const COMMON_FONTS = [
  "Fira Code",
  "JetBrains Mono",
  "Cascadia Code",
  "Source Code Pro",
  "Consolas",
  "Inconsolata",
  "Hack",
  "Roboto Mono",
  "Ubuntu Mono",
  "Liberation Mono",
  "Courier New",
  "monospace",
];

async function updateFont(value: string): Promise<void> {
  await settingsStore.update("fontFamily", value);
}

async function updateSize(value: number): Promise<void> {
  const clamped = Math.max(8, Math.min(32, value));
  await settingsStore.update("fontSize", clamped);
}

async function toggleLineNumbers(): Promise<void> {
  await settingsStore.update("showLineNumbers", !settingsStore.settings.showLineNumbers);
}

async function toggleWordWrap(): Promise<void> {
  await settingsStore.update("wordWrap", !settingsStore.settings.wordWrap);
}
</script>

<template>
  <div class="font-settings">
    <p class="text-subtitle-2 mb-4">Font Settings</p>

    <v-autocomplete
      :model-value="settingsStore.settings.fontFamily"
      :items="COMMON_FONTS"
      label="Font Family"
      density="compact"
      variant="outlined"
      hide-details
      class="mb-4"
      @update:model-value="updateFont"
    />

    <v-text-field
      :model-value="settingsStore.settings.fontSize"
      label="Font Size (px)"
      type="number"
      density="compact"
      variant="outlined"
      :min="8"
      :max="32"
      hide-details
      class="mb-4"
      @update:model-value="(v) => updateSize(Number(v))"
    />

    <v-switch
      :model-value="settingsStore.settings.showLineNumbers"
      label="Show Line Numbers"
      density="compact"
      hide-details
      class="mb-2"
      @update:model-value="toggleLineNumbers"
    />

    <v-switch
      :model-value="settingsStore.settings.wordWrap"
      label="Word Wrap"
      density="compact"
      hide-details
      @update:model-value="toggleWordWrap"
    />
  </div>
</template>
