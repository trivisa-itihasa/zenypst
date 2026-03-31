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
        class="error-item mb-2"
      >
        <div class="text-caption text-error font-weight-medium">
          <template v-if="err.file">{{ err.file }}<template v-if="err.line">:{{ err.line }}</template><template v-if="err.column">:{{ err.column }}</template> — </template>{{ err.message }}
        </div>
        <div v-if="err.sourceLine" class="source-snippet mt-1">
          <span class="source-text">{{ err.sourceLine }}</span>
          <br v-if="err.column" />
          <span v-if="err.column" class="caret-line">{{ ' '.repeat(err.column - 1) }}^</span>
        </div>
        <div v-for="(hint, hi) in err.hints" :key="hi" class="text-caption hint-text mt-1">
          hint: {{ hint }}
        </div>
      </div>
      <div
        v-for="(warn, i) in previewStore.warnings"
        :key="`w${i}`"
        class="warning-item mb-2"
      >
        <div class="text-caption text-warning font-weight-medium">
          <v-icon size="12" class="mr-1">mdi-alert</v-icon>
          <template v-if="warn.file">{{ warn.file }}<template v-if="warn.line">:{{ warn.line }}</template><template v-if="warn.column">:{{ warn.column }}</template> — </template>{{ warn.message }}
        </div>
        <div v-if="warn.sourceLine" class="source-snippet mt-1">
          <span class="source-text">{{ warn.sourceLine }}</span>
          <br v-if="warn.column" />
          <span v-if="warn.column" class="caret-line">{{ ' '.repeat(warn.column - 1) }}^</span>
        </div>
        <div v-for="(hint, hi) in warn.hints" :key="hi" class="text-caption hint-text mt-1">
          hint: {{ hint }}
        </div>
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

.source-snippet {
  background: rgba(var(--v-theme-surface-variant), 0.5);
  border-left: 2px solid rgb(var(--v-theme-error));
  padding: 2px 6px;
  font-family: monospace;
  font-size: 11px;
  white-space: pre;
  overflow-x: auto;
}

.warning-item .source-snippet {
  border-left-color: rgb(var(--v-theme-warning));
}

.caret-line {
  color: rgb(var(--v-theme-error));
  white-space: pre;
}

.hint-text {
  color: rgb(var(--v-theme-info));
  font-family: monospace;
}
</style>
