<script setup lang="ts">
import { useSettingsStore } from "@/stores/settings";
import type { AppSettings } from "@/types";

const settingsStore = useSettingsStore();

const PREVIEW_MODES: { value: AppSettings["previewMode"]; label: string; description: string }[] = [
  {
    value: "realtime",
    label: "Real-time",
    description: "Compile automatically as you type",
  },
  {
    value: "on_save",
    label: "On Save",
    description: "Compile when you save the file (Ctrl+S)",
  },
  {
    value: "manual",
    label: "Manual",
    description: "Compile only when you click the compile button",
  },
];

async function setMode(mode: AppSettings["previewMode"]): Promise<void> {
  await settingsStore.update("previewMode", mode);
}

async function updateDebounce(value: string | number | null): Promise<void> {
  const num = Number(value);
  if (Number.isNaN(num)) return;
  const clamped = Math.max(0, Math.min(5000, num));
  await settingsStore.update("realtimeDebounceMs", clamped);
}
</script>

<template>
  <div class="preview-settings">
    <p class="text-subtitle-2 mb-4">Preview Settings</p>

    <p class="text-body-2 text-medium-emphasis mb-2">Update Mode</p>
    <template v-for="mode in PREVIEW_MODES" :key="mode.value">
      <div class="mode-row">
        <q-radio
          :model-value="settingsStore.settings.previewMode"
          :val="mode.value"
          dense
          @update:model-value="setMode"
        >
          <div class="mode-label">
            <div>{{ mode.label }}</div>
            <div class="text-caption text-medium-emphasis">{{ mode.description }}</div>
          </div>
        </q-radio>
      </div>
      <div v-if="mode.value === 'realtime'" class="debounce-wrap">
        <q-input
          :model-value="settingsStore.settings.realtimeDebounceMs"
          :disable="settingsStore.settings.previewMode !== 'realtime'"
          label="Debounce (ms)"
          type="number"
          outlined
          dense
          :min="0"
          :max="5000"
          @update:model-value="updateDebounce"
        />
      </div>
    </template>
  </div>
</template>

<style scoped>
.mode-row {
  margin-bottom: 10px;
}

.mode-label {
  display: flex;
  flex-direction: column;
  margin-left: 4px;
}

.debounce-wrap {
  padding-bottom: 8px;
  margin-top: 0;
  padding-left: 36px;
  width: 200px;
}
</style>
