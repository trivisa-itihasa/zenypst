<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import FileTreeItem from "./FileTreeItem.vue";

const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

const contextMenu = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);
const newFileDialog = ref(false);
const newFileValue = ref("");
const newFolderDialog = ref(false);
const newFolderValue = ref("");

function closeContextMenu(): void { contextMenu.value = false; }
onMounted(() => { document.addEventListener("zenypst:close-context-menus", closeContextMenu); });
onUnmounted(() => { document.removeEventListener("zenypst:close-context-menus", closeContextMenu); });

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
  event.preventDefault();
  document.dispatchEvent(new Event("zenypst:close-context-menus"));
  contextMenuX.value = event.clientX;
  contextMenuY.value = event.clientY;
  setTimeout(() => {
    contextMenu.value = true;
  }, 0);
}

function startNewFile(): void {
  newFileValue.value = "";
  newFileDialog.value = true;
}

async function confirmNewFile(): Promise<void> {
  let name = newFileValue.value.trim();
  if (!name || !fileTreeStore.rootPath) { newFileDialog.value = false; return; }
  if (!name.includes(".")) name += ".typ";
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
      <v-btn icon size="x-small" variant="text" title="Refresh" @click="refreshTree">
        <v-icon size="14">mdi-refresh</v-icon>
      </v-btn>
      <v-btn icon size="x-small" variant="text" title="Open Folder" @click="openFolder">
        <v-icon size="14">mdi-folder-open-outline</v-icon>
      </v-btn>
    </div>

    <v-divider />

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
        class="d-flex flex-column align-center justify-center pa-4 mt-4"
      >
        <v-icon size="40" color="medium-emphasis">mdi-folder-outline</v-icon>
        <p class="text-caption text-medium-emphasis mt-2 text-center">
          Open a folder to browse files
        </p>
        <v-btn
          size="small"
          variant="outlined"
          class="mt-3"
          prepend-icon="mdi-folder-open"
          @click="openFolder"
        >
          Open Folder
        </v-btn>
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
      v-model="contextMenu"
      :style="{ left: `${contextMenuX}px`, top: `${contextMenuY}px` }"
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
}

.file-tree-content {
  flex: 1 1 0;
  overflow-y: auto;
}
</style>
