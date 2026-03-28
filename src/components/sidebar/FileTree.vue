<script setup lang="ts">
import { ref } from "vue";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import { useContextMenu } from "@/composables/useContextMenu";
import { ensureTypExtension } from "@/utils/path";
import FileTreeItem from "./FileTreeItem.vue";

const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

const ctxMenu = useContextMenu();
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

function showRootContextMenu(event: MouseEvent): void {
  if (!fileTreeStore.rootPath) return;
  ctxMenu.show(event);
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
      <span class="text-caption text-uppercase font-weight-medium tracking-widest flex-grow-1">
        <template v-if="fileTreeStore.rootPath">
          {{ getRootName(fileTreeStore.rootPath) }}
        </template>
        <template v-else>Explorer</template>
      </span>
      <v-btn icon variant="text" title="Refresh" class="header-btn" @click="refreshTree">
        <v-icon class="header-btn-icon">mdi-refresh</v-icon>
      </v-btn>
      <v-btn
        v-if="fileTreeStore.rootPath"
        icon
        variant="text"
        title="Close Folder"
        class="header-btn"
        @click="fileTreeStore.clearTree()"
      >
        <v-icon class="header-btn-icon">mdi-folder-remove-outline</v-icon>
      </v-btn>
      <v-btn icon variant="text" title="Open Folder" class="header-btn" @click="openFolder">
        <v-icon class="header-btn-icon">mdi-folder-open-outline</v-icon>
      </v-btn>
    </div>

    <!-- Tree content — right-click on empty space triggers root context menu -->
    <div class="file-tree-content py-1" @contextmenu.self.prevent="showRootContextMenu">
      <v-progress-linear v-if="fileTreeStore.isLoading" indeterminate color="primary" height="2" />

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
        <v-icon size="64" color="medium-emphasis">mdi-folder-outline</v-icon>
        <p class="text-medium-emphasis mt-4">Open a folder to browse files</p>
        <div class="file-tree-empty__bottom">
          <div class="file-tree-empty__actions">
            <v-btn variant="text" prepend-icon="mdi-folder-open" class="empty-action-btn" @click="openFolder">
              Open Folder
            </v-btn>
          </div>
        </div>
      </div>

      <v-alert
        v-if="fileTreeStore.error"
        type="error"
        density="compact"
        class="ma-2"
      >
        {{ fileTreeStore.error }}
      </v-alert>
    </div>

    <!-- Root-level context menu (empty space) -->
    <v-menu
      v-model="ctxMenu.visible.value"
      :style="{ left: `${ctxMenu.x.value}px`, top: `${ctxMenu.y.value}px` }"
    >
      <v-list density="compact">
        <v-list-item
          prepend-icon="mdi-file-plus-outline"
          title="New File"
          @click="startNewFile"
        />
        <v-list-item
          prepend-icon="mdi-folder-plus-outline"
          title="New Folder"
          @click="startNewFolder"
        />
      </v-list>
    </v-menu>

    <!-- New File dialog -->
    <v-dialog v-model="newFileDialog" max-width="400">
      <v-card>
        <v-card-title>New File</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="newFileValue"
            label="File name"
            autofocus
            @keyup.enter="confirmNewFile"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="newFileDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="confirmNewFile">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- New Folder dialog -->
    <v-dialog v-model="newFolderDialog" max-width="400">
      <v-card>
        <v-card-title>New Folder</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="newFolderValue"
            label="Folder name"
            autofocus
            @keyup.enter="confirmNewFolder"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="newFolderDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="confirmNewFolder">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped>
.file-tree {
  position: absolute;
  inset: 0;
  background: rgb(var(--v-theme-surface));
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.file-tree-header {
  height: var(--panel-header-height);
  min-height: var(--panel-header-height);
  flex-shrink: 0;
  border-bottom: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
}

.file-tree-content {
  flex: 1 1 0;
  overflow-y: auto;
}

.header-btn {
  border-radius: 4px !important;
  min-width: 0 !important;
  width: calc(var(--panel-header-height) - 8px) !important;
  height: calc(var(--panel-header-height) - 8px) !important;
  padding: 0 !important;
  margin-top: 4px !important;
  margin-bottom: 4px !important;
}

.header-btn :deep(.v-icon) {
  font-size: calc(var(--panel-header-height) - 16px) !important;
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
  justify-content: flex-start !important;
}
</style>
