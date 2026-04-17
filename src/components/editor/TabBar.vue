<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useEditorStore } from "@/stores/editor";
import { useSettingsStore } from "@/stores/settings";
import { useCompiler } from "@/composables/useCompiler";
import type { FileTab } from "@/types";

const { t } = useI18n();

const editorStore = useEditorStore();
const settingsStore = useSettingsStore();
const { triggerCompile } = useCompiler();

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
      <q-icon size="14px" class="mr-1 flex-shrink-0" :name="getTabIcon(tab)" />
      <span class="tab-name">
        {{ tab.name }}
      </span>
      <span v-if="tab.isDirty" class="tab-dirty ml-1 flex-shrink-0">●</span>
      <button
        class="tab-close ml-auto flex-shrink-0"
        @click.stop="closeTab($event, tab)"
      >
        <q-icon name="mdi-close" size="12px" />
      </button>
    </div>

    <div v-if="editorStore.tabs.length === 0" class="tab-placeholder px-4 text-medium-emphasis text-caption">
      {{ t('tabBar.noFilesOpen') }}
    </div>

    <q-space />

    <q-btn
      v-if="settingsStore.settings.previewMode === 'manual'"
      dense
      flat
      color="primary"
      icon="mdi-play"
      :disable="!editorStore.activeTab"
      :title="t('tabBar.compile')"
      class="compile-btn mx-2 flex-shrink-0"
      @click="triggerCompile"
    />
  </div>
</template>

<style scoped>
.tab-bar {
  height: var(--panel-header-height);
  min-height: var(--panel-header-height);
  background: var(--zen-surface);
  border-bottom: 1px solid var(--zen-border);
  gap: 0;
  scrollbar-width: thin;
}

.tab-item {
  height: 100%;
  width: 160px;
  flex-shrink: 0;
  cursor: pointer;
  border-right: 1px solid var(--zen-border);
  user-select: none;
  font-size: var(--ui-font-size-sm);
  color: rgba(var(--zen-on-surface-rgb), 0.6);
  transition: background 0.1s;
}

.tab-item:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.06);
}

.tab-item--active {
  background: var(--zen-background);
  color: var(--zen-on-surface);
  border-bottom: 2px solid var(--zen-primary);
}

.tab-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tab-dirty {
  color: var(--zen-warning);
  font-size: var(--ui-font-size-xxs);
}

.tab-close {
  opacity: 0;
  transition: opacity 0.1s;
  border: none;
  background: transparent;
  color: inherit;
  padding: 2px;
  cursor: pointer;
  border-radius: 3px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.tab-close:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.12);
}

.tab-item:hover .tab-close,
.tab-item--active .tab-close {
  opacity: 1;
}

.tab-placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  padding-left: 12px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.compile-btn {
  border-radius: 4px;
  min-width: 0;
  width: calc(var(--panel-header-height) - 8px);
  height: calc(var(--panel-header-height) - 8px);
  padding: 0;
}
</style>
