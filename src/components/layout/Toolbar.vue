<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useFileOps } from "@/composables/useFileOps";
import { useCompiler } from "@/composables/useCompiler";
import { useSettingsStore } from "@/stores/settings";
import { useEditorStore } from "@/stores/editor";

const emit = defineEmits<{
  (e: "new-file"): void;
  (e: "open-settings"): void;
  (e: "open-templates"): void;
  (e: "toggle-file-tree"): void;
  (e: "toggle-preview"): void;
}>();

const fileOps = useFileOps();
const { triggerCompile, exportPdf } = useCompiler();
const settingsStore = useSettingsStore();
const editorStore = useEditorStore();

// Menu state
const fileMenu = ref(false);
const editMenu = ref(false);
const viewMenu = ref(false);
const helpMenu = ref(false);

// About dialog
const aboutDialog = ref(false);

// Snackbar
const snackbar = ref(false);
const snackbarText = ref("");
const snackbarColor = ref("error");

function showSnackbar(text: string, color = "error"): void {
  snackbarText.value = text;
  snackbarColor.value = color;
  snackbar.value = true;
}

async function handleNewFile(): Promise<void> {
  fileMenu.value = false;
  emit("new-file");
}

async function handleOpenFile(): Promise<void> {
  fileMenu.value = false;
  await fileOps.openFileDialog();
}

async function handleOpenFolder(): Promise<void> {
  fileMenu.value = false;
  await fileOps.openFolderDialog();
}

async function handleSave(): Promise<void> {
  fileMenu.value = false;
  await fileOps.saveActiveFile();
  if (settingsStore.settings.previewMode === "on_save") {
    await triggerCompile();
  }
}

async function handleSaveAs(): Promise<void> {
  fileMenu.value = false;
  await fileOps.saveAsActiveFile();
}

async function handleExportPdf(): Promise<void> {
  fileMenu.value = false;
  const tab = editorStore.activeTab;
  if (!tab) return;

  const defaultName = tab.name.replace(/\.typ$/, "") + ".pdf";
  const outputPath = await invoke<string | null>("save_file_dialog", { defaultName });
  if (!outputPath) return;

  const root = tab.path ? getDirectory(tab.path) : undefined;
  try {
    const result = await exportPdf(tab.content, root, outputPath);
    if (result.success) {
      showSnackbar("PDF exported successfully.", "success");
    } else {
      const msg = result.errors.map((e) => e.message).join("; ");
      showSnackbar("Export failed: " + msg, "error");
    }
  } catch (err) {
    showSnackbar("Export failed: " + String(err), "error");
  }
}

async function handleCompile(): Promise<void> {
  await triggerCompile();
}

function getDirectory(filePath: string): string {
  const sep = filePath.includes("/") ? "/" : "\\";
  const parts = filePath.split(sep);
  parts.pop();
  return parts.join(sep) || "/";
}
</script>

<template>
  <div class="toolbar d-flex align-center" style="min-height: 36px;">
    <!-- App name -->
    <div class="app-name px-3 d-flex align-center">
      <v-icon size="18" color="primary" class="mr-2">mdi-typewriter</v-icon>
      <span class="text-body-2 font-weight-medium">Zenypst</span>
    </div>

    <v-divider vertical class="mx-1" style="height: 24px;" />

    <!-- File menu -->
    <v-menu v-model="fileMenu" :close-on-content-click="true">
      <template #activator="{ props }">
        <v-btn variant="text" size="small" v-bind="props" class="menu-btn">File</v-btn>
      </template>
      <v-list density="compact">
        <v-list-item prepend-icon="mdi-file-plus" title="New File" subtitle="Ctrl+N" @click="handleNewFile" />
        <v-divider />
        <v-list-item prepend-icon="mdi-file-outline" title="Open File…" subtitle="Ctrl+O" @click="handleOpenFile" />
        <v-list-item prepend-icon="mdi-folder-open" title="Open Folder…" subtitle="Ctrl+Shift+O" @click="handleOpenFolder" />
        <v-divider />
        <v-list-item prepend-icon="mdi-content-save" title="Save" subtitle="Ctrl+S" :disabled="!editorStore.activeTab" @click="handleSave" />
        <v-list-item prepend-icon="mdi-content-save-edit" title="Save As…" :disabled="!editorStore.activeTab" @click="handleSaveAs" />
        <v-divider />
        <v-list-item prepend-icon="mdi-file-pdf-box" title="Export PDF…" :disabled="!editorStore.activeTab" @click="handleExportPdf" />
        <v-divider />
        <v-list-item prepend-icon="mdi-file-document-multiple" title="Manage Templates" @click="emit('open-templates')" />
      </v-list>
    </v-menu>

    <!-- View menu -->
    <v-menu v-model="viewMenu" :close-on-content-click="true">
      <template #activator="{ props }">
        <v-btn variant="text" size="small" v-bind="props" class="menu-btn">View</v-btn>
      </template>
      <v-list density="compact">
        <v-list-item
          prepend-icon="mdi-file-tree"
          title="Show File Tree"
          :disabled="settingsStore.settings.fileTreeVisible"
          @click="emit('toggle-file-tree')"
        />
        <v-list-item
          prepend-icon="mdi-file-pdf-box"
          title="Show Preview"
          :disabled="settingsStore.settings.previewVisible"
          @click="emit('toggle-preview')"
        />
      </v-list>
    </v-menu>

    <!-- Settings -->
    <v-btn
      variant="text"
      size="small"
      class="menu-btn"
      @click="emit('open-settings')"
    >
      Settings
    </v-btn>

    <!-- Help menu -->
    <v-menu v-model="helpMenu" :close-on-content-click="true">
      <template #activator="{ props }">
        <v-btn variant="text" size="small" v-bind="props" class="menu-btn">Help</v-btn>
      </template>
      <v-list density="compact">
        <v-list-item prepend-icon="mdi-information" title="About Zenypst" @click="aboutDialog = true" />
      </v-list>
    </v-menu>

    <v-spacer />

    <!-- Compile button (for manual / quick access) -->
    <v-btn
      v-if="settingsStore.settings.previewMode === 'manual'"
      size="small"
      color="primary"
      variant="tonal"
      prepend-icon="mdi-play"
      class="mr-2"
      :disabled="!editorStore.activeTab"
      @click="handleCompile"
    >
      Compile
    </v-btn>
  </div>

  <!-- About dialog -->
  <v-dialog v-model="aboutDialog" max-width="400">
    <v-card>
      <v-card-title class="d-flex align-center">
        <v-icon size="24" color="primary" class="mr-2">mdi-typewriter</v-icon>
        Zenypst
      </v-card-title>
      <v-card-text>
        <p class="mb-2">A desktop Typst editor with live PDF preview.</p>
        <p class="text-caption text-medium-emphasis">Version 0.1.0</p>
        <p class="text-caption text-medium-emphasis">Built with Tauri, Vue 3, Vuetify, and CodeMirror 6.</p>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn @click="aboutDialog = false">Close</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <!-- Snackbar -->
  <v-snackbar v-model="snackbar" :color="snackbarColor" timeout="4000" location="bottom">
    {{ snackbarText }}
    <template #actions>
      <v-btn variant="text" @click="snackbar = false">Close</v-btn>
    </template>
  </v-snackbar>
</template>

<style scoped>
.toolbar {
  background: rgb(var(--v-theme-surface));
  border-bottom: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  height: 36px;
  flex-shrink: 0;
  overflow: hidden;
}

.app-name {
  font-size: 13px;
  min-width: 120px;
}

.menu-btn {
  height: 36px !important;
  border-radius: 0 !important;
  font-size: 13px;
  min-width: 0;
  padding: 0 12px;
}

.menu-btn:hover {
  background: rgba(var(--v-theme-on-surface), 0.08) !important;
}
</style>
