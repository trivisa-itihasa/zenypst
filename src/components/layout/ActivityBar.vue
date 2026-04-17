<script setup lang="ts">
import { useI18n } from "vue-i18n";

defineProps<{
  fileTreeVisible: boolean;
  previewVisible: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle-file-tree"): void;
  (e: "toggle-preview"): void;
  (e: "open-settings"): void;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="activity-bar d-flex flex-column align-center py-1">
    <!-- File tree toggle -->
    <button
      class="activity-btn"
      :class="{ 'activity-btn--active': fileTreeVisible }"
      @click="emit('toggle-file-tree')"
      :title="t('activityBar.toggleExplorer')"
    >
      <q-icon name="mdi-folder-outline" size="20px" />
    </button>

    <!-- PDF viewer toggle -->
    <button
      class="activity-btn mt-1"
      :class="{ 'activity-btn--active': previewVisible }"
      @click="emit('toggle-preview')"
      :title="t('activityBar.togglePdfViewer')"
    >
      <q-icon name="mdi-file-pdf-box" size="26px" />
    </button>

    <div class="flex-grow-1" />

    <!-- Settings -->
    <button class="activity-btn mb-1" @click="emit('open-settings')" :title="t('activityBar.settings')">
      <q-icon name="mdi-cog-outline" size="20px" />
    </button>
  </div>
</template>

<style scoped>
.activity-bar {
  width: var(--activity-bar-width);
  flex-shrink: 0;
  border-right: 1px solid var(--zen-border);
  background: var(--zen-surface);
}

.activity-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 6px;
  background: transparent;
  cursor: pointer;
  color: rgba(var(--zen-on-surface-rgb), 0.6);
  transition: background 0.1s, color 0.1s;
}

.activity-btn:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.08);
  color: rgba(var(--zen-on-surface-rgb), 1);
}

.activity-btn--active {
  color: var(--zen-primary);
  background: rgba(var(--zen-primary-rgb), 0.12);
}

.app-icon {
  height: 20px;
  width: auto;
}
</style>
