<script setup lang="ts">
import { useSettingsStore } from "@/stores/settings";
import type { AppSettings } from "@/types";

const settingsStore = useSettingsStore();

const PREVIEW_MODES: { value: AppSettings["previewMode"]; label: string; description: string }[] = [
  {
    value: "realtime",
    label: "Real-time",
    description: "Compile automatically as you type (debounced)",
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
  const clamped = Math.max(100, Math.min(5000, value));
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
      <v-radio
        v-for="mode in PREVIEW_MODES"
        :key="mode.value"
        :value="mode.value"
        :label="mode.label"
        density="compact"
      >
        <template #label>
          <div>
            <span>{{ mode.label }}</span>
            <br />
            <span class="text-caption text-medium-emphasis">{{ mode.description }}</span>
          </div>
        </template>
      </v-radio>
    </v-radio-group>

    <v-text-field
      v-if="settingsStore.settings.previewMode === 'realtime'"
      :model-value="settingsStore.settings.realtimeDebounceMs"
      label="Debounce (ms)"
      type="number"
      density="compact"
      variant="outlined"
      :min="100"
      :max="5000"
      hide-details
      class="mt-2"
      @update:model-value="(v) => updateDebounce(Number(v))"
    />
  </div>
</template>
