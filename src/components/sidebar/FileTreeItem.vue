<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import type { FileNode } from "@/types";

const props = defineProps<{ node: FileNode; depth?: number }>();
const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

const contextMenu = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);
const renameDialog = ref(false);
const renameValue = ref("");
const deleteDialog = ref(false);

const paddingLeft = computed(() => `${((props.depth ?? 0) + 1) * 16}px`);

const fileIcon = computed(() => {
  if (props.node.isDir) {
    return fileTreeStore.isExpanded(props.node.path)
      ? "mdi-folder-open"
      : "mdi-folder";
  }
  const ext = props.node.extension;
  if (ext === "typ") return "mdi-file-document-outline";
  if (ext === "pdf") return "mdi-file-pdf-box";
  if (["png", "jpg", "jpeg", "gif", "webp", "svg"].includes(ext ?? ""))
    return "mdi-file-image";
  return "mdi-file-outline";
});

const fileIconColor = computed(() => {
  if (props.node.isDir) return "warning";
  const ext = props.node.extension;
  if (ext === "typ") return "primary";
  if (ext === "pdf") return "error";
  return "";
});

function handleClick(): void {
  if (props.node.isDir) {
    fileTreeStore.toggleExpanded(props.node.path);
  } else {
    emit("open-file", props.node.path);
  }
}

function showContextMenu(event: MouseEvent): void {
  event.preventDefault();
  contextMenu.value = false;
  contextMenuX.value = event.clientX;
  contextMenuY.value = event.clientY;
  setTimeout(() => {
    contextMenu.value = true;
  }, 0);
}

function startRename(): void {
  renameValue.value = props.node.name;
  renameDialog.value = true;
}

async function confirmRename(): Promise<void> {
  if (!renameValue.value || renameValue.value === props.node.name) {
    renameDialog.value = false;
    return;
  }
  const dir = props.node.path.substring(0, props.node.path.lastIndexOf("/"));
  const newPath = `${dir}/${renameValue.value}`;
  try {
    await fileOps.renamePath(props.node.path, newPath);
  } catch (err) {
    console.error("Rename failed:", err);
  }
  renameDialog.value = false;
}

async function confirmDelete(): Promise<void> {
  try {
    await fileOps.deletePath(props.node.path);
  } catch (err) {
    console.error("Delete failed:", err);
  }
  deleteDialog.value = false;
}

async function openInFileManager(): Promise<void> {
  await invoke("open_in_file_manager", { path: props.node.path });
}
</script>

<template>
  <div>
    <!-- Node row -->
    <div
      class="file-tree-item d-flex align-center"
      :style="{ paddingLeft }"
      @click="handleClick"
      @contextmenu="showContextMenu"
    >
      <!-- Expand arrow for directories -->
      <v-icon
        v-if="node.isDir"
        size="16"
        class="mr-1"
        :icon="fileTreeStore.isExpanded(node.path) ? 'mdi-chevron-down' : 'mdi-chevron-right'"
      />
      <span v-else class="mr-1" style="width: 16px; display: inline-block;" />

      <v-icon size="16" :icon="fileIcon" :color="fileIconColor" class="mr-2" />
      <span class="file-name text-body-2">{{ node.name }}</span>
    </div>

    <!-- Children (if directory and expanded) -->
    <template v-if="node.isDir && fileTreeStore.isExpanded(node.path)">
      <FileTreeItem
        v-for="child in node.children ?? []"
        :key="child.path"
        :node="child"
        :depth="(depth ?? 0) + 1"
        @open-file="$emit('open-file', $event)"
      />
    </template>

    <!-- Context menu -->
    <v-menu
      v-model="contextMenu"
      :style="{ left: `${contextMenuX}px`, top: `${contextMenuY}px` }"
    >
      <v-list density="compact">
        <v-list-item
          v-if="!node.isDir"
          prepend-icon="mdi-pencil"
          title="Rename"
          @click="startRename"
        />
        <v-list-item
          prepend-icon="mdi-delete"
          title="Delete"
          @click="deleteDialog = true"
        />
        <v-list-item
          prepend-icon="mdi-folder-open"
          title="Reveal in File Manager"
          @click="openInFileManager"
        />
      </v-list>
    </v-menu>

    <!-- Rename dialog -->
    <v-dialog v-model="renameDialog" max-width="400">
      <v-card>
        <v-card-title>Rename</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="renameValue"
            label="New name"
            autofocus
            @keyup.enter="confirmRename"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="renameDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="confirmRename">Rename</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete confirm dialog -->
    <v-dialog v-model="deleteDialog" max-width="400">
      <v-card>
        <v-card-title>Delete "{{ node.name }}"?</v-card-title>
        <v-card-text>
          This action cannot be undone.
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="confirmDelete">Delete</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped>
.file-tree-item {
  padding-top: 2px;
  padding-bottom: 2px;
  padding-right: 8px;
  cursor: pointer;
  user-select: none;
  border-radius: 4px;
  margin: 1px 4px;
}

.file-tree-item:hover {
  background: rgba(var(--v-theme-on-surface), 0.08);
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
}
</style>
