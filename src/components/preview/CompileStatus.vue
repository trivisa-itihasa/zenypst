<script setup lang="ts">
import { computed } from "vue";
import { usePreviewStore } from "@/stores/preview";

const previewStore = usePreviewStore();

const statusText = computed(() => {
  switch (previewStore.status) {
    case "compiling": return "Compiling…";
    case "success": return "Compiled successfully";
    case "error": return `${previewStore.errors.length} error(s)`;
    default: return "Idle";
  }
});

const statusColor = computed(() => {
  switch (previewStore.status) {
    case "compiling": return "info";
    case "success": return "success";
    case "error": return "error";
    default: return "default";
  }
});
</script>

<template>
  <div v-if="previewStore.status === 'error'" class="compile-status-overlay">
    <div class="compile-error-panel pa-3">
      <div class="d-flex align-center mb-2">
        <v-icon color="error" class="mr-2">mdi-alert-circle</v-icon>
        <span class="text-subtitle-2 text-error">Compilation Errors</span>
      </div>
      <div
        v-for="(err, i) in previewStore.errors"
        :key="i"
        class="error-item mb-1"
      >
        <span class="text-caption text-error">
          <template v-if="err.file">{{ err.file }}<template v-if="err.line">:{{ err.line }}</template><template v-if="err.column">:{{ err.column }}</template> — </template>
          {{ err.message }}
        </span>
      </div>
      <div
        v-for="(warn, i) in previewStore.warnings"
        :key="`w${i}`"
        class="warning-item mb-1"
      >
        <span class="text-caption text-warning">
          <v-icon size="12" class="mr-1">mdi-alert</v-icon>
          {{ warn.message }}
        </span>
      </div>
    </div>
  </div>

  <div v-else-if="previewStore.status === 'compiling'" class="compile-indicator pa-2">
    <v-progress-linear indeterminate color="info" height="2" />
  </div>
</template>

<style scoped>
.compile-status-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 10;
  pointer-events: auto;
}

.compile-error-panel {
  background: rgba(var(--v-theme-surface), 0.95);
  border-top: 2px solid rgb(var(--v-theme-error));
  max-height: 200px;
  overflow-y: auto;
  backdrop-filter: blur(4px);
}

.compile-indicator {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
}

.error-item, .warning-item {
  font-family: monospace;
}
</style>
