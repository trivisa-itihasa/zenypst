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

async function setMode(mode: AppSettings["previewMode"] | null): Promise<void> {
  if (!mode) return;
  await settingsStore.update("previewMode", mode);
}

async function updateDebounce(value: number): Promise<void> {
  const clamped = Math.max(0, Math.min(5000, value));
  await settingsStore.update("realtimeDebounceMs", clamped);
}
</script>

<template>
  <div class="preview-settings">
    <p class="text-subtitle-2 mb-4">Preview Settings</p>

    <p class="text-body-2 text-medium-emphasis mb-2">Update Mode</p>
    <v-radio-group
      :model-value="settingsStore.settings.previewMode"
      @update:model-value="setMode"
    >
      <template v-for="mode in PREVIEW_MODES" :key="mode.value">
        <v-radio :value="mode.value" density="compact" class="mode-radio">
          <template #label>
            <div>
              <div>{{ mode.label }}</div>
              <div class="text-caption text-medium-emphasis">{{ mode.description }}</div>
            </div>
          </template>
        </v-radio>
        <div v-if="mode.value === 'realtime'" class="debounce-wrap">
          <v-text-field
            :model-value="settingsStore.settings.realtimeDebounceMs"
            :disabled="settingsStore.settings.previewMode !== 'realtime'"
            label="Debounce (ms)"
            type="number"
            density="compact"
            variant="outlined"
            :min="0"
            :max="5000"
            hide-details
            @update:model-value="(v) => updateDebounce(Number(v))"
          />
        </div>
      </template>
    </v-radio-group>
  </div>
</template>

<style scoped>
:deep(.mode-radio) {
  margin-bottom: 10px;
}

:deep(.mode-radio .v-selection-control) {
  align-items: flex-start;
}
:deep(.mode-radio .v-selection-control__wrapper) {
  margin-top: 4px;
  align-self: flex-start;
}
.debounce-wrap {
  padding-bottom: 8px;
  margin-top: 0;
  padding-left: 24px;
  width: 160px;
}
</style>
