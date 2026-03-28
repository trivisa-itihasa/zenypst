<script setup lang="ts">
import { useEditorStore } from "@/stores/editor";
import type { FileTab } from "@/types";

const editorStore = useEditorStore();

function selectTab(tab: FileTab): void {
  editorStore.setActiveTab(tab.id);
}

function closeTab(event: MouseEvent, tab: FileTab): void {
  event.stopPropagation();
  editorStore.closeTab(tab.id);
}

function getTabIcon(tab: FileTab): string {
  if (tab.isUntitled) return "mdi-file-outline";
  const ext = tab.name.split(".").pop();
  if (ext === "typ") return "mdi-file-document-outline";
  if (ext === "pdf") return "mdi-file-pdf-box";
  return "mdi-file-outline";
}
</script>

<template>
  <div class="tab-bar d-flex align-center overflow-x-auto">
    <div
      v-for="tab in editorStore.tabs"
      :key="tab.id"
      class="tab-item d-flex align-center px-2"
      :class="{ 'tab-item--active': tab.id === editorStore.activeTabId }"
      @click="selectTab(tab)"
    >
      <v-icon size="14" class="mr-1 flex-shrink-0" :icon="getTabIcon(tab)" />
      <span class="tab-name">
        {{ tab.name }}
      </span>
      <span v-if="tab.isDirty" class="tab-dirty ml-1 flex-shrink-0">●</span>
      <v-btn
        icon
        density="compact"
        size="x-small"
        variant="text"
        class="tab-close ml-auto flex-shrink-0"
        @click.stop="closeTab($event, tab)"
      >
        <v-icon size="12">mdi-close</v-icon>
      </v-btn>
    </div>

    <div v-if="editorStore.tabs.length === 0" class="tab-placeholder px-4 text-medium-emphasis text-caption">
      No files open
    </div>
  </div>
</template>

<style scoped>
.tab-bar {
  height: var(--panel-header-height);
  min-height: var(--panel-header-height);
  background: rgb(var(--v-theme-surface));
  border-bottom: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  gap: 0;
  scrollbar-width: thin;
}

.tab-item {
  height: 100%;
  width: 160px;
  flex-shrink: 0;
  cursor: pointer;
  border-right: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  user-select: none;
  font-size: var(--ui-font-size-sm);
  color: rgba(var(--v-theme-on-surface), 0.6);
  transition: background 0.1s;
}

.tab-item:hover {
  background: rgba(var(--v-theme-on-surface), 0.06);
}

.tab-item--active {
  background: rgb(var(--v-theme-background));
  color: rgb(var(--v-theme-on-surface));
  border-bottom: 2px solid rgb(var(--v-theme-primary));
}

.tab-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tab-dirty {
  color: rgb(var(--v-theme-warning));
  font-size: var(--ui-font-size-xxs);
}

.tab-close {
  opacity: 0;
  transition: opacity 0.1s;
}

.tab-item:hover .tab-close,
.tab-item--active .tab-close {
  opacity: 1;
}

.tab-placeholder {
  flex: 1;
  display: flex;
  align-items: center;
}
</style>
