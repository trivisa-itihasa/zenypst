<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import TemplatePickerDialog from "@/components/template/TemplatePickerDialog.vue";
import type { FileNode, Template } from "@/types";

const props = defineProps<{ node: FileNode; depth?: number }>();
const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

const contextMenu = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);
const deleteDialog = ref(false);
const newFileDialog = ref(false);
const newFileValue = ref("");
const newFileTemplatePicker = ref(false);
const newFileTemplate = ref<Template | null>(null);
const newFolderDialog = ref(false);
const newFolderValue = ref("");

// Inline rename state
const isRenaming = ref(false);
const renameInputValue = ref("");
const renameInputRef = ref<HTMLInputElement | null>(null);

const itemDepth = computed(() => props.depth ?? 0);

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
  if (isRenaming.value) return;
  if (props.node.isDir) {
    fileTreeStore.toggleExpanded(props.node.path);
  } else {
    emit("open-file", props.node.path);
  }
}

const CLOSE_CONTEXT_MENUS_EVENT = "zenypst:close-context-menus";

onMounted(() => {
  document.addEventListener(CLOSE_CONTEXT_MENUS_EVENT, closeContextMenu);
});
onUnmounted(() => {
  document.removeEventListener(CLOSE_CONTEXT_MENUS_EVENT, closeContextMenu);
});

function closeContextMenu(): void {
  contextMenu.value = false;
}

function showContextMenu(event: MouseEvent): void {
  event.preventDefault();
  event.stopPropagation();
  document.dispatchEvent(new Event(CLOSE_CONTEXT_MENUS_EVENT));
  contextMenuX.value = event.clientX;
  contextMenuY.value = event.clientY;
  setTimeout(() => {
    contextMenu.value = true;
  }, 0);
}

// Inline rename
function startInlineRename(event: MouseEvent): void {
  event.stopPropagation();
  renameInputValue.value = props.node.name;
  isRenaming.value = true;
  nextTick(() => {
    renameInputRef.value?.focus();
    renameInputRef.value?.select();
  });
}

async function confirmInlineRename(): Promise<void> {
  if (!isRenaming.value) return;
  isRenaming.value = false;
  const newName = renameInputValue.value.trim();
  if (!newName || newName === props.node.name) return;
  const dir = props.node.path.substring(0, props.node.path.lastIndexOf("/"));
  const newPath = `${dir}/${newName}`;
  try {
    await fileOps.renamePath(props.node.path, newPath);
  } catch (err) {
    console.error("Rename failed:", err);
  }
}

function cancelInlineRename(): void {
  isRenaming.value = false;
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

function startNewFile(): void {
  newFileValue.value = "";
  newFileTemplate.value = null;
  newFileDialog.value = true;
}

function handleNewFileTemplateSelected(template: Template): void {
  newFileTemplate.value = template;
}

async function confirmNewFile(): Promise<void> {
  let name = newFileValue.value.trim();
  if (!name) { newFileDialog.value = false; return; }
  if (!name.includes(".")) name += ".typ";
  const path = `${props.node.path}/${name}`;
  try {
    await fileOps.createFileOnDisk(path, newFileTemplate.value?.content ?? "");
    if (!fileTreeStore.isExpanded(props.node.path)) {
      fileTreeStore.toggleExpanded(props.node.path);
    }
  } catch (err) {
    console.error("Create file failed:", err);
  }
  newFileDialog.value = false;
  newFileTemplate.value = null;
}

function startNewFolder(): void {
  newFolderValue.value = "";
  newFolderDialog.value = true;
}

async function confirmNewFolder(): Promise<void> {
  const name = newFolderValue.value.trim();
  if (!name) { newFolderDialog.value = false; return; }
  const path = `${props.node.path}/${name}`;
  try {
    await fileOps.createDirectory(path);
    if (!fileTreeStore.isExpanded(props.node.path)) {
      fileTreeStore.toggleExpanded(props.node.path);
    }
  } catch (err) {
    console.error("Create folder failed:", err);
  }
  newFolderDialog.value = false;
}
</script>

<template>
  <div>
    <!-- Node row -->
    <div
      class="file-tree-item d-flex align-center"
      :style="{ '--depth': itemDepth }"
      @click="handleClick"
      @dblclick.stop.prevent="startInlineRename"
      @contextmenu="showContextMenu"
    >
      <!-- Expand arrow for directories -->
      <v-icon
        v-if="node.isDir"
        size="16"
        class="mr-1"
        :icon="fileTreeStore.isExpanded(node.path) ? 'mdi-chevron-down' : 'mdi-chevron-right'"
      />
      <span v-else class="mr-1" style="width: var(--tree-indent); display: inline-block;" />

      <v-icon size="16" :icon="fileIcon" :color="fileIconColor" class="mr-2" />

      <!-- Inline rename input or static label -->
      <input
        v-if="isRenaming"
        ref="renameInputRef"
        v-model="renameInputValue"
        class="file-name-input"
        @blur="confirmInlineRename"
        @keyup.enter="confirmInlineRename"
        @keyup.escape.stop="cancelInlineRename"
        @click.stop
        @dblclick.stop
        @contextmenu.stop
      />
      <span
        v-else
        class="file-name text-body-2"
      >{{ node.name }}</span>
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

    <!-- Context menu: folders only show New File / New Folder -->
    <v-menu
      v-model="contextMenu"
      :style="{ left: `${contextMenuX}px`, top: `${contextMenuY}px` }"
    >
      <v-list density="compact">
        <template v-if="node.isDir">
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
          <v-divider />
        </template>
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

    <!-- New File dialog (folder context) -->
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
          <v-chip
            v-if="newFileTemplate"
            size="small"
            prepend-icon="mdi-file-document-outline"
            closable
            @click:close="newFileTemplate = null"
          >
            {{ newFileTemplate.name }}
          </v-chip>
        </v-card-text>
        <v-card-actions>
          <v-btn
            variant="text"
            prepend-icon="mdi-file-document-multiple-outline"
            @click="newFileTemplatePicker = true"
          >From Template</v-btn>
          <v-spacer />
          <v-btn @click="newFileDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="confirmNewFile">Create</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Template picker for new file -->
    <TemplatePickerDialog
      v-model="newFileTemplatePicker"
      @selected="handleNewFileTemplateSelected"
    />

    <!-- New Folder dialog (folder context) -->
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
  height: 24px;
  padding-left: calc(var(--depth, 0) * var(--tree-indent) + 4px);
  padding-right: 8px;
  cursor: pointer;
  user-select: none;
  border-radius: 4px;
  margin: 1px 4px;
  flex-shrink: 0;
}

.file-tree-item:hover {
  background: rgba(var(--v-theme-on-surface), 0.08);
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: var(--ui-font-size);
}

.file-name-input {
  background: transparent;
  border: 1px solid rgb(var(--v-theme-primary));
  border-radius: 2px;
  color: inherit;
  font-size: var(--ui-font-size);
  font-family: inherit;
  padding: 0 3px;
  outline: none;
  min-width: 0;
  flex: 1;
}
</style>
