<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import { ensureTypExtension } from "@/utils/path";
import FileTreeItem from "./FileTreeItem.vue";

const { t } = useI18n();

const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

const newFileDialog = ref(false);
const newFileValue = ref("");
const newFolderDialog = ref(false);
const newFolderValue = ref("");

async function openFolder(): Promise<void> {
  await fileOps.openFolderDialog();
}

async function refreshTree(): Promise<void> {
  await fileTreeStore.refresh();
}

function getRootName(path: string): string {
  return path.split("/").pop() ?? path;
}

function startNewFile(): void {
  newFileValue.value = "";
  newFileDialog.value = true;
}

async function confirmNewFile(): Promise<void> {
  const raw = newFileValue.value.trim();
  if (!raw || !fileTreeStore.rootPath) { newFileDialog.value = false; return; }
  const name = ensureTypExtension(raw);
  try {
    await fileOps.createFileOnDisk(`${fileTreeStore.rootPath}/${name}`, "");
  } catch (err) {
    console.error("Create file failed:", err);
  }
  newFileDialog.value = false;
}

function startNewFolder(): void {
  newFolderValue.value = "";
  newFolderDialog.value = true;
}

async function confirmNewFolder(): Promise<void> {
  const name = newFolderValue.value.trim();
  if (!name || !fileTreeStore.rootPath) { newFolderDialog.value = false; return; }
  try {
    await fileOps.createDirectory(`${fileTreeStore.rootPath}/${name}`);
  } catch (err) {
    console.error("Create folder failed:", err);
  }
  newFolderDialog.value = false;
}
</script>

<template>
  <div class="file-tree">
    <!-- Header -->
    <div class="file-tree-header d-flex align-center px-3">
      <span class="text-caption text-uppercase font-weight-medium tracking-widest flex-grow-1 root-name-label">
        <template v-if="fileTreeStore.rootPath">
          {{ getRootName(fileTreeStore.rootPath) }}
        </template>
        <template v-else>{{ t('fileTree.explorer') }}</template>
      </span>
      <button class="header-btn" :title="t('fileTree.refresh')" @click="refreshTree">
        <q-icon name="mdi-refresh" class="header-btn-icon" />
      </button>
      <button
        v-if="fileTreeStore.rootPath"
        class="header-btn"
        :title="t('fileTree.closeFolder')"
        @click="fileTreeStore.clearTree()"
      >
        <q-icon name="mdi-folder-remove-outline" class="header-btn-icon" />
      </button>
      <button class="header-btn" :title="t('fileTree.openFolder')" @click="openFolder">
        <q-icon name="mdi-folder-open-outline" class="header-btn-icon" />
      </button>
    </div>

    <!-- Tree content — right-click on empty space triggers root context menu -->
    <div class="file-tree-content py-1">
      <q-linear-progress
        v-if="fileTreeStore.isLoading"
        indeterminate
        size="2px"
        color="primary"
      />

      <template v-if="fileTreeStore.rootPath && !fileTreeStore.isLoading">
        <FileTreeItem
          v-for="node in fileTreeStore.nodes"
          :key="node.path"
          :node="node"
          :depth="0"
          @open-file="$emit('open-file', $event)"
        />
      </template>

      <div
        v-if="!fileTreeStore.rootPath && !fileTreeStore.isLoading"
        class="file-tree-empty d-flex flex-column align-center"
      >
        <div class="file-tree-empty__top" />
        <q-icon name="mdi-folder-outline" size="64px" color="grey-6" />
        <p class="text-medium-emphasis mt-4 empty-message">{{ t('fileTree.openFolderToBrowse') }}</p>
        <div class="file-tree-empty__bottom">
          <div class="file-tree-empty__actions">
            <q-btn
              flat
              no-caps
              no-wrap
              icon="mdi-folder-open"
              :label="t('fileTree.openFolder')"
              class="empty-action-btn"
              @click="openFolder"
            />
          </div>
        </div>
      </div>

      <div v-if="fileTreeStore.error" class="zen-alert zen-alert--error ma-2">
        {{ fileTreeStore.error }}
      </div>

      <!-- Root-level context menu (right-click on empty space) -->
      <q-menu touch-position context-menu>
        <q-list dense class="zen-menu-list">
          <q-item clickable v-close-popup @click="startNewFile">
            <q-item-section avatar><q-icon name="mdi-file-plus-outline" size="16px" /></q-item-section>
            <q-item-section>{{ t('fileTree.newFile') }}</q-item-section>
          </q-item>
          <q-item clickable v-close-popup @click="startNewFolder">
            <q-item-section avatar><q-icon name="mdi-folder-plus-outline" size="16px" /></q-item-section>
            <q-item-section>{{ t('fileTree.newFolder') }}</q-item-section>
          </q-item>
        </q-list>
      </q-menu>
    </div>

    <!-- New File dialog -->
    <q-dialog v-model="newFileDialog">
      <q-card class="zen-card" style="width: 400px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">{{ t('fileTree.newFile') }}</div></q-card-section>
        <q-card-section>
          <q-input
            v-model="newFileValue"
            :label="t('fileTree.fileName')"
            outlined
            dense
            autofocus
            @keyup.enter="confirmNewFile"
          />
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat :label="t('common.cancel')" @click="newFileDialog = false" />
          <q-btn flat color="primary" :label="t('common.create')" @click="confirmNewFile" />
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- New Folder dialog -->
    <q-dialog v-model="newFolderDialog">
      <q-card class="zen-card" style="width: 400px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">{{ t('fileTree.newFolder') }}</div></q-card-section>
        <q-card-section>
          <q-input
            v-model="newFolderValue"
            :label="t('fileTree.folderName')"
            outlined
            dense
            autofocus
            @keyup.enter="confirmNewFolder"
          />
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat :label="t('common.cancel')" @click="newFolderDialog = false" />
          <q-btn flat color="primary" :label="t('common.create')" @click="confirmNewFolder" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<style scoped>
.file-tree {
  position: absolute;
  inset: 0;
  background: var(--zen-surface);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.file-tree-header {
  height: var(--panel-header-height);
  min-height: var(--panel-header-height);
  flex-shrink: 0;
  border-bottom: 1px solid var(--zen-border);
}

.file-tree-content {
  flex: 1 1 0;
  overflow-y: auto;
  position: relative;
}

.header-btn {
  border-radius: 4px;
  min-width: 0;
  width: calc(var(--panel-header-height) - 8px);
  height: calc(var(--panel-header-height) - 8px);
  padding: 0;
  margin: 4px 0;
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.header-btn:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.08);
}

.header-btn-icon {
  font-size: calc(var(--panel-header-height) - 16px);
}

.file-tree-empty {
  height: 100%;
}

.file-tree-empty__top {
  flex: 2;
}

.file-tree-empty__bottom {
  flex: 3;
  position: relative;
  align-self: stretch;
}

.file-tree-empty__actions {
  position: absolute;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  gap: 8px;
  white-space: nowrap;
}

.empty-action-btn {
  justify-content: flex-start;
}

.empty-message {
  height: 1lh;
  overflow: visible;
  text-align: center;
  padding: 0 16px;
}

.root-name-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.zen-alert {
  padding: 8px 12px;
  border-radius: 4px;
  font-size: var(--ui-font-size-sm);
}

.zen-alert--error {
  background: rgba(var(--zen-error-rgb), 0.15);
  color: var(--zen-error);
  border: 1px solid rgba(var(--zen-error-rgb), 0.4);
}
</style>
