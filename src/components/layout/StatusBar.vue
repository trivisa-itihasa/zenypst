<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useEditorStore } from "@/stores/editor";
import { usePreviewStore } from "@/stores/preview";
import { useSettingsStore } from "@/stores/settings";

const { t } = useI18n();
const editorStore = useEditorStore();
const previewStore = usePreviewStore();
const settingsStore = useSettingsStore();

const currentFile = computed(() => {
  const tab = editorStore.activeTab;
  if (!tab) return "";
  return tab.path ?? tab.name;
});

const compileStatusText = computed(() => {
  switch (previewStore.status) {
    case "compiling": return t("statusBar.compiling");
    case "success": return t("statusBar.compiled");
    case "error": return t("statusBar.errorCount", { count: previewStore.errors.length });
    default: return t("statusBar.idle");
  }
});

const compileStatusColor = computed(() => {
  switch (previewStore.status) {
    case "compiling": return "text-info";
    case "success": return "text-success";
    case "error": return "text-error";
    default: return "text-medium-emphasis";
  }
});

const previewModeLabel = computed(() => {
  switch (settingsStore.settings.previewMode) {
    case "realtime": return t("statusBar.realtime");
    case "on_save": return t("statusBar.onSave");
    case "manual": return t("statusBar.manual");
    default: return "";
  }
});
</script>

<template>
  <div class="status-bar d-flex align-center px-3 text-caption">
    <!-- File path -->
    <span class="status-bar__file text-medium-emphasis text-truncate">
      {{ currentFile || t('statusBar.noFileOpen') }}
    </span>

    <q-space />

    <!-- Preview mode -->
    <span class="text-medium-emphasis mr-4">
      <q-icon name="mdi-eye" size="12px" class="mr-1" />
      {{ previewModeLabel }}
    </span>

    <!-- Compile status -->
    <span :class="compileStatusColor" class="mr-4">
      <q-icon
        v-if="previewStore.status === 'compiling'"
        name="mdi-loading"
        size="12px"
        class="mr-1"
      />
      {{ compileStatusText }}
    </span>

    <!-- Font info -->
    <span class="text-medium-emphasis">
      {{ settingsStore.settings.fontFamily }} {{ settingsStore.settings.fontSize }}px
    </span>
  </div>
</template>

<style scoped>
.status-bar {
  height: var(--statusbar-height);
  min-height: var(--statusbar-height);
  background: #239dad;
  color: #ffffff;
  font-size: var(--ui-font-size-xs);
  flex-shrink: 0;
  overflow: hidden;
}

.status-bar .text-medium-emphasis {
  color: rgba(255, 255, 255, 0.7) !important;
}

.status-bar .text-success {
  color: #a6e3a1 !important;
}

.status-bar .text-error {
  color: #ffffff !important;
}

.status-bar__file {
  max-width: 40%;
}

.status-bar .text-info {
  color: #89dceb !important;
}
</style>
