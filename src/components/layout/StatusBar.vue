<script setup lang="ts">
import { computed } from "vue";
import { useEditorStore } from "@/stores/editor";
import { usePreviewStore } from "@/stores/preview";
import { useSettingsStore } from "@/stores/settings";

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
    case "compiling": return "Compiling…";
    case "success": return "✓ Compiled";
    case "error": return `✗ ${previewStore.errors.length} error(s)`;
    default: return "Idle";
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
    case "realtime": return "Real-time";
    case "on_save": return "On Save";
    case "manual": return "Manual";
    default: return "";
  }
});
</script>

<template>
  <div class="status-bar d-flex align-center px-3 text-caption">
    <!-- File path -->
    <span class="text-medium-emphasis text-truncate" style="max-width: 40%;">
      {{ currentFile || "No file open" }}
    </span>

    <v-spacer />

    <!-- Preview mode -->
    <span class="text-medium-emphasis mr-4">
      <v-icon size="12" class="mr-1">mdi-eye</v-icon>
      {{ previewModeLabel }}
    </span>

    <!-- Compile status -->
    <span :class="compileStatusColor" class="mr-4">
      <v-icon
        v-if="previewStore.status === 'compiling'"
        size="12"
        class="mr-1"
      >mdi-loading</v-icon>
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
  background: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  font-size: var(--ui-font-size-xs);
  flex-shrink: 0;
  overflow: hidden;
}

.status-bar .text-medium-emphasis {
  color: rgba(var(--v-theme-on-primary), 0.7) !important;
}

.status-bar .text-success {
  color: #a6e3a1 !important;
}

.status-bar .text-error {
  color: #f38ba8 !important;
}

.status-bar .text-info {
  color: #89dceb !important;
}
</style>
