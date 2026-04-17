<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";

const { t } = useI18n();

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

async function onInput(value: string | number | null): Promise<void> {
  const v = value == null ? "" : String(value);
  fontInput.value = v;
  fontQuery.value = v;
  showList.value = true;
  await settingsStore.update("fontFamily", v);
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

async function onFallbackInput(value: string | number | null): Promise<void> {
  const v = value == null ? "" : String(value);
  fallbackInput.value = v;
  fallbackQuery.value = v;
  showFallbackList.value = true;
  await settingsStore.update("fontFamilyFallback", v);
}

async function updateSize(value: string | number | null): Promise<void> {
  const num = Number(value);
  if (Number.isNaN(num)) return;
  const clamped = Math.max(8, Math.min(32, num));
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
    <p class="text-subtitle-2 mb-4">{{ t('fontSettings.title') }}</p>

    <div class="font-picker mb-4">
      <q-input
        :model-value="fontInput"
        :label="t('fontSettings.fontFamily')"
        outlined
        dense
        autocomplete="off"
        @focus="onFocus"
        @blur="onBlur"
        @update:model-value="onInput"
      />
      <div v-if="showList && filteredFonts.length" class="font-list">
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
      <q-input
        :model-value="fallbackInput"
        :label="t('fontSettings.fallbackFont')"
        outlined
        dense
        autocomplete="off"
        @focus="onFallbackFocus"
        @blur="onFallbackBlur"
        @update:model-value="onFallbackInput"
      />
      <div v-if="showFallbackList && filteredFallbackFonts.length" class="font-list">
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

    <q-input
      :model-value="settingsStore.settings.fontSize"
      :label="t('fontSettings.fontSize')"
      type="number"
      outlined
      dense
      :min="8"
      :max="32"
      class="mb-4"
      @update:model-value="updateSize"
    />

    <q-toggle
      :model-value="settingsStore.settings.showLineNumbers"
      :label="t('fontSettings.showLineNumbers')"
      dense
      class="mb-2"
      @update:model-value="toggleLineNumbers"
    />

    <br />

    <q-toggle
      :model-value="settingsStore.settings.wordWrap"
      :label="t('fontSettings.wordWrap')"
      dense
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
  background: var(--zen-surface);
  border: 1px solid var(--zen-border);
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.font-list-item {
  padding: 6px 12px;
  font-size: 0.875rem;
  cursor: pointer;
}

.font-list-item:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.08);
}
</style>
