<script setup lang="ts">
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import FileTreeItem from "./FileTreeItem.vue";

const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

async function openFolder(): Promise<void> {
  await fileOps.openFolderDialog();
}

async function refreshTree(): Promise<void> {
  await fileTreeStore.refresh();
}

function getRootName(path: string): string {
  return path.split("/").pop() ?? path;
}
</script>

<template>
  <div class="file-tree d-flex flex-column fill-height">
    <!-- Header -->
    <div class="file-tree-header d-flex align-center px-3 py-1">
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

    <!-- Tree content -->
    <div class="file-tree-content flex-grow-1 overflow-y-auto py-1">
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
  </div>
</template>

<style scoped>
.file-tree {
  background: rgb(var(--v-theme-surface));
  overflow: hidden;
}

.file-tree-header {
  min-height: 36px;
  flex-shrink: 0;
}

.file-tree-content {
  scrollbar-width: thin;
}
</style>
