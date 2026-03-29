<script setup lang="ts">
import { ref, computed } from "vue";
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

const fontInput = ref(settingsStore.settings.fontFamily);
const showList = ref(false);
const fallbackInput = ref(settingsStore.settings.fontFamilyFallback);
const showFallbackList = ref(false);

const filteredFonts = computed(() => {
  const q = fontInput.value.toLowerCase();
  return COMMON_FONTS.filter((f) => f.toLowerCase().includes(q));
});

const filteredFallbackFonts = computed(() => {
  const q = fallbackInput.value.toLowerCase();
  return COMMON_FONTS.filter((f) => f.toLowerCase().includes(q));
});

function onFocus() {
  fontInput.value = settingsStore.settings.fontFamily;
  showList.value = true;
}

function onBlur() {
  setTimeout(() => { showList.value = false; }, 150);
}

async function selectFont(font: string): Promise<void> {
  fontInput.value = font;
  showList.value = false;
  await settingsStore.update("fontFamily", font);
}

async function onInput(value: string): Promise<void> {
  fontInput.value = value;
  showList.value = true;
  await settingsStore.update("fontFamily", value);
}

function onFallbackFocus() {
  fallbackInput.value = settingsStore.settings.fontFamilyFallback;
  showFallbackList.value = true;
}

function onFallbackBlur() {
  setTimeout(() => { showFallbackList.value = false; }, 150);
}

async function selectFallbackFont(font: string): Promise<void> {
  fallbackInput.value = font;
  showFallbackList.value = false;
  await settingsStore.update("fontFamilyFallback", font);
}

async function onFallbackInput(value: string): Promise<void> {
  fallbackInput.value = value;
  showFallbackList.value = true;
  await settingsStore.update("fontFamilyFallback", value);
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

    <div class="font-picker mb-4">
      <v-text-field
        :model-value="fontInput"
        label="Font Family"
        density="compact"
        variant="outlined"
        hide-details
        autocomplete="off"
        @focus="onFocus"
        @blur="onBlur"
        @update:model-value="onInput"
      />
      <div v-if="showList && filteredFonts.length" class="font-list elevation-2">
        <div
          v-for="font in filteredFonts"
          :key="font"
          class="font-list-item"
          @mousedown.prevent="selectFont(font)"
        >
          {{ font }}
        </div>
      </div>
    </div>

    <div class="font-picker mb-4">
      <v-text-field
        :model-value="fallbackInput"
        label="Fallback Font"
        density="compact"
        variant="outlined"
        hide-details
        autocomplete="off"
        @focus="onFallbackFocus"
        @blur="onFallbackBlur"
        @update:model-value="onFallbackInput"
      />
      <div v-if="showFallbackList && filteredFallbackFonts.length" class="font-list elevation-2">
        <div
          v-for="font in filteredFallbackFonts"
          :key="font"
          class="font-list-item"
          @mousedown.prevent="selectFallbackFont(font)"
        >
          {{ font }}
        </div>
      </div>
    </div>

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

<style scoped>
.font-picker {
  position: relative;
}

.font-list {
  position: absolute;
  left: 0;
  right: 0;
  top: 100%;
  z-index: 100;
  background: rgb(var(--v-theme-surface));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
}

.font-list-item {
  padding: 6px 12px;
  font-size: 0.875rem;
  cursor: pointer;
}

.font-list-item:hover {
  background: rgba(var(--v-theme-on-surface), 0.08);
}
</style>
