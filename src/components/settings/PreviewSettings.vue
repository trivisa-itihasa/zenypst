<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
import type { AppSettings } from "@/types";

const { t } = useI18n();
const settingsStore = useSettingsStore();

const PREVIEW_MODES: { value: AppSettings["previewMode"]; label: string; description: string }[] = [
  {
    value: "realtime",
    label: t("previewSettings.realtime"),
    description: t("previewSettings.realtimeDescription"),
  },
  {
    value: "on_save",
    label: t("previewSettings.onSave"),
    description: t("previewSettings.onSaveDescription"),
  },
  {
    value: "manual",
    label: t("previewSettings.manual"),
    description: t("previewSettings.manualDescription"),
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
    <p class="text-subtitle-2 mb-4">{{ t('previewSettings.title') }}</p>

    <p class="text-body-2 text-medium-emphasis mb-2">{{ t('previewSettings.updateMode') }}</p>
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
          :label="t('previewSettings.debounce')"
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
