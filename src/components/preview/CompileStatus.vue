<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { usePreviewStore } from "@/stores/preview";

const { t } = useI18n();
const previewStore = usePreviewStore();

const statusText = computed(() => {
  switch (previewStore.status) {
    case "compiling": return t("compileStatus.compiling");
    case "success": return t("compileStatus.compiledSuccessfully");
    case "error": return t("compileStatus.errorCount", { count: previewStore.errors.length });
    default: return t("compileStatus.idle");
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
        <q-icon name="mdi-alert-circle" color="negative" class="mr-2" />
        <span class="text-subtitle-2 text-error">{{ t('compileStatus.compilationErrors') }}</span>
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
          {{ t('compileStatus.hint', { hint }) }}
        </div>
      </div>
      <div
        v-for="(warn, i) in previewStore.warnings"
        :key="`w${i}`"
        class="warning-item mb-2"
      >
        <div class="text-caption text-warning font-weight-medium">
          <q-icon name="mdi-alert" size="12px" class="mr-1" />
          <template v-if="warn.file">{{ warn.file }}<template v-if="warn.line">:{{ warn.line }}</template><template v-if="warn.column">:{{ warn.column }}</template> — </template>{{ warn.message }}
        </div>
        <div v-if="warn.sourceLine" class="source-snippet mt-1">
          <span class="source-text">{{ warn.sourceLine }}</span>
          <br v-if="warn.column" />
          <span v-if="warn.column" class="caret-line">{{ ' '.repeat(warn.column - 1) }}^</span>
        </div>
        <div v-for="(hint, hi) in warn.hints" :key="hi" class="text-caption hint-text mt-1">
          {{ t('compileStatus.hint', { hint }) }}
        </div>
      </div>
    </div>
  </div>

  <div v-else-if="previewStore.status === 'compiling'" class="compile-indicator pa-2">
    <q-linear-progress indeterminate color="info" size="2px" />
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
  background: rgba(var(--zen-on-surface-rgb), 0.06);
  border-top: 2px solid var(--zen-error);
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
  background: rgba(var(--zen-on-surface-rgb), 0.06);
  border-left: 2px solid var(--zen-error);
  padding: 2px 6px;
  font-family: monospace;
  font-size: 11px;
  white-space: pre;
  overflow-x: auto;
}

.warning-item .source-snippet {
  border-left-color: var(--zen-warning);
}

.caret-line {
  color: var(--zen-error);
  white-space: pre;
}

.hint-text {
  color: var(--zen-info);
  font-family: monospace;
}
</style>
