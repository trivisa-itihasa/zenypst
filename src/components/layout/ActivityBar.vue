<script setup lang="ts">
defineProps<{
  fileTreeVisible: boolean;
  previewVisible: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle-file-tree"): void;
  (e: "toggle-preview"): void;
  (e: "open-settings"): void;
}>();
</script>

<template>
  <div class="activity-bar d-flex flex-column align-center py-1">
    <!-- File tree toggle -->
    <button
      class="activity-btn"
      :class="{ 'activity-btn--active': fileTreeVisible }"
      @click="emit('toggle-file-tree')"
      title="Toggle Explorer"
    >
      <v-icon size="20">mdi-folder-outline</v-icon>
    </button>

    <!-- PDF viewer toggle -->
    <button
      class="activity-btn mt-1"
      :class="{ 'activity-btn--active': previewVisible }"
      @click="emit('toggle-preview')"
      title="Toggle PDF Viewer"
    >
      <v-icon size="26">mdi-file-pdf-box</v-icon>
    </button>

    <div class="flex-grow-1" />

    <!-- Settings -->
    <button class="activity-btn mb-1" @click="emit('open-settings')" title="Settings">
      <v-icon size="20">mdi-cog-outline</v-icon>
    </button>
  </div>
</template>

<style scoped>
.activity-bar {
  width: var(--activity-bar-width);
  flex-shrink: 0;
  border-right: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  background: rgb(var(--v-theme-surface));
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
  color: rgba(var(--v-theme-on-surface), 0.6);
  transition: background 0.1s, color 0.1s;
}

.activity-btn:hover {
  background: rgba(var(--v-theme-on-surface), 0.08);
  color: rgba(var(--v-theme-on-surface), 1);
}

.activity-btn--active {
  color: rgb(var(--v-theme-primary));
  background: rgba(var(--v-theme-primary), 0.12);
}

.app-icon {
  height: 20px;
  width: auto;
}
</style>
