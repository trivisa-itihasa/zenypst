<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";

const { t } = useI18n();
const settingsStore = useSettingsStore();

async function toggleFullPath(): Promise<void> {
  await settingsStore.update(
    "showFullPathInTreeHeader",
    !settingsStore.settings.showFullPathInTreeHeader
  );
}
</script>

<template>
  <div class="general-settings">
    <p class="text-subtitle-2 mb-4">{{ t("generalSettings.title") }}</p>

    <q-toggle
      :model-value="settingsStore.settings.showFullPathInTreeHeader"
      :label="t('generalSettings.showFullPathInTreeHeader')"
      dense
      @update:model-value="toggleFullPath"
    />
  </div>
</template>
