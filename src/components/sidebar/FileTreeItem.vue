<script setup lang="ts">
import { ref, computed, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import { ensureTypExtension } from "@/utils/path";
import TemplatePickerDialog from "@/components/template/TemplatePickerDialog.vue";
import type { FileNode, Template } from "@/types";

const props = defineProps<{ node: FileNode; depth?: number }>();
const emit = defineEmits<{ (e: "open-file", path: string): void }>();

const fileTreeStore = useFileTreeStore();
const fileOps = useFileOps();

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
  if (props.node.isDir) return "grey-6";
  const ext = props.node.extension;
  if (ext === "typ") return "primary";
  if (ext === "pdf") return "negative";
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
  const raw = newFileValue.value.trim();
  if (!raw) { newFileDialog.value = false; return; }
  const name = ensureTypExtension(raw);
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
    >
      <!-- Expand arrow for directories -->
      <q-icon
        v-if="node.isDir"
        size="14px"
        class="mr-1"
        :name="fileTreeStore.isExpanded(node.path) ? 'mdi-chevron-down' : 'mdi-chevron-right'"
      />
      <span v-else class="mr-1" style="width: var(--tree-indent); display: inline-block;" />

      <q-icon size="14px" :name="fileIcon" :color="fileIconColor" class="mr-1" />

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

      <!-- Context menu attached to the row -->
      <q-menu touch-position context-menu>
        <q-list dense class="zen-menu-list">
          <template v-if="node.isDir">
            <q-item clickable v-close-popup @click="startNewFile">
              <q-item-section avatar><q-icon name="mdi-file-plus-outline" size="16px" /></q-item-section>
              <q-item-section>New File</q-item-section>
            </q-item>
            <q-item clickable v-close-popup @click="startNewFolder">
              <q-item-section avatar><q-icon name="mdi-folder-plus-outline" size="16px" /></q-item-section>
              <q-item-section>New Folder</q-item-section>
            </q-item>
            <q-separator />
          </template>
          <q-item clickable v-close-popup @click="deleteDialog = true">
            <q-item-section avatar><q-icon name="mdi-delete" size="16px" /></q-item-section>
            <q-item-section>Delete</q-item-section>
          </q-item>
          <q-item clickable v-close-popup @click="openInFileManager">
            <q-item-section avatar><q-icon name="mdi-folder-open" size="16px" /></q-item-section>
            <q-item-section>Reveal in File Manager</q-item-section>
          </q-item>
        </q-list>
      </q-menu>
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

    <!-- New File dialog (folder context) -->
    <q-dialog v-model="newFileDialog">
      <q-card class="zen-card" style="width: 400px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">New File</div></q-card-section>
        <q-card-section>
          <q-input
            v-model="newFileValue"
            label="File name"
            outlined
            dense
            autofocus
            @keyup.enter="confirmNewFile"
          />
          <q-chip
            v-if="newFileTemplate"
            removable
            dense
            class="q-mt-sm"
            icon="mdi-file-document-outline"
            @remove="newFileTemplate = null"
          >
            {{ newFileTemplate.name }}
          </q-chip>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn
            flat
            no-caps
            icon="mdi-file-document-multiple-outline"
            label="From Template"
            @click="newFileTemplatePicker = true"
          />
          <q-space />
          <q-btn flat label="Cancel" @click="newFileDialog = false" />
          <q-btn flat color="primary" label="Create" @click="confirmNewFile" />
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- Template picker for new file -->
    <TemplatePickerDialog
      v-model="newFileTemplatePicker"
      @selected="handleNewFileTemplateSelected"
    />

    <!-- New Folder dialog (folder context) -->
    <q-dialog v-model="newFolderDialog">
      <q-card class="zen-card" style="width: 400px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">New Folder</div></q-card-section>
        <q-card-section>
          <q-input
            v-model="newFolderValue"
            label="Folder name"
            outlined
            dense
            autofocus
            @keyup.enter="confirmNewFolder"
          />
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" @click="newFolderDialog = false" />
          <q-btn flat color="primary" label="Create" @click="confirmNewFolder" />
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- Delete confirm dialog -->
    <q-dialog v-model="deleteDialog">
      <q-card class="zen-card" style="width: 400px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">Delete "{{ node.name }}"?</div></q-card-section>
        <q-card-section>
          This action cannot be undone.
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" @click="deleteDialog = false" />
          <q-btn flat color="negative" label="Delete" @click="confirmDelete" />
        </q-card-actions>
      </q-card>
    </q-dialog>
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
  background: rgba(var(--zen-on-surface-rgb), 0.08);
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: var(--ui-font-size-sm);
}

.file-name-input {
  background: transparent;
  border: 1px solid var(--zen-primary);
  border-radius: 2px;
  color: inherit;
  font-size: var(--ui-font-size-sm);
  font-family: inherit;
  padding: 0 3px;
  outline: none;
  min-width: 0;
  flex: 1;
}
</style>
