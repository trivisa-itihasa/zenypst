<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
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

const systemFonts = ref<string[]>([]);

onMounted(async () => {
  if ("__TAURI_INTERNALS__" in window) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const fonts = await invoke<string[]>("list_system_fonts");
      systemFonts.value = fonts;
    } catch (e) {
      console.error("Failed to load system fonts:", e);
    }
  }
});

// Merged, deduplicated list: system fonts first, then common fonts not already present
const allFonts = computed(() => {
  if (systemFonts.value.length > 0) {
    const sys = new Set(systemFonts.value.map((f) => f.toLowerCase()));
    const extras = COMMON_FONTS.filter((f) => !sys.has(f.toLowerCase()));
    return [...systemFonts.value, ...extras];
  }
  return COMMON_FONTS;
});

// Display value (selected font name shown in the text field)
const fontInput = ref(settingsStore.settings.fontFamily);
// Separate filter query — empty means "show all"
const fontQuery = ref("");
const showList = ref(false);

const fallbackInput = ref(settingsStore.settings.fontFamilyFallback);
const fallbackQuery = ref("");
const showFallbackList = ref(false);

const filteredFonts = computed(() => {
  const q = fontQuery.value.toLowerCase().trim();
  if (!q) return allFonts.value;
  return allFonts.value.filter((f) => f.toLowerCase().includes(q));
});

const filteredFallbackFonts = computed(() => {
  const q = fallbackQuery.value.toLowerCase().trim();
  if (!q) return allFonts.value;
  return allFonts.value.filter((f) => f.toLowerCase().includes(q));
});

function onFocus() {
  fontInput.value = settingsStore.settings.fontFamily;
  fontQuery.value = "";
  showList.value = true;
}

function onBlur() {
  setTimeout(() => {
    showList.value = false;
    fontQuery.value = "";
    fontInput.value = settingsStore.settings.fontFamily;
  }, 150);
}

async function selectFont(font: string): Promise<void> {
  fontInput.value = font;
  fontQuery.value = "";
  showList.value = false;
  await settingsStore.update("fontFamily", font);
}

async function onInput(value: string): Promise<void> {
  fontInput.value = value;
  fontQuery.value = value;
  showList.value = true;
  await settingsStore.update("fontFamily", value);
}

function onFallbackFocus() {
  fallbackInput.value = settingsStore.settings.fontFamilyFallback;
  fallbackQuery.value = "";
  showFallbackList.value = true;
}

function onFallbackBlur() {
  setTimeout(() => {
    showFallbackList.value = false;
    fallbackQuery.value = "";
    fallbackInput.value = settingsStore.settings.fontFamilyFallback;
  }, 150);
}

async function selectFallbackFont(font: string): Promise<void> {
  fallbackInput.value = font;
  fallbackQuery.value = "";
  showFallbackList.value = false;
  await settingsStore.update("fontFamilyFallback", font);
}

async function onFallbackInput(value: string): Promise<void> {
  fallbackInput.value = value;
  fallbackQuery.value = value;
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
          :style="{ fontFamily: font }"
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
          :style="{ fontFamily: font }"
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
