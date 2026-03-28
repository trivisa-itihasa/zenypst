<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import appIconUrl from "@/assets/icons/icon-str.svg";
import { useFileOps } from "@/composables/useFileOps";
import { useCompiler } from "@/composables/useCompiler";
import { useSettingsStore } from "@/stores/settings";
import { useEditorStore } from "@/stores/editor";
import { getDirectory } from "@/utils/path";

const emit = defineEmits<{
  (e: "new-file"): void;
  (e: "open-templates"): void;
}>();

const fileOps = useFileOps();
const { triggerCompile, exportPdf } = useCompiler();
const settingsStore = useSettingsStore();
const editorStore = useEditorStore();

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
const appWindow = isTauri ? getCurrentWindow() : null;
const isMaximized = ref(false);

onMounted(async () => {
  if (!appWindow) return;
  isMaximized.value = await appWindow.isMaximized();
});

async function minimize(): Promise<void> {
  await appWindow!.minimize();
}

async function toggleMaximize(): Promise<void> {
  await appWindow!.toggleMaximize();
  isMaximized.value = !isMaximized.value;
}

async function closeWindow(): Promise<void> {
  await appWindow!.close();
}

const aboutDialog = ref(false);
const fileMenu = ref(false);

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
</script>

<template>
  <div class="toolbar d-flex align-center" data-tauri-drag-region>
    <div class="toolbar-logo">
      <button class="icon-btn" @click="aboutDialog = true" title="About Zenypst">
        <img :src="appIconUrl" class="app-icon" alt="Zenypst" />
      </button>
    </div>

    <v-menu v-model="fileMenu" :close-on-content-click="true" :transition="false">
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

    <v-spacer />

    <template v-if="isTauri">
      <button class="winctl-btn" @click="minimize" title="最小化">
        <v-icon size="14">mdi-minus</v-icon>
      </button>
      <button class="winctl-btn" @click="toggleMaximize" :title="isMaximized ? '元のサイズに戻す' : '最大化'">
        <v-icon size="14">{{ isMaximized ? 'mdi-window-restore' : 'mdi-window-maximize' }}</v-icon>
      </button>
      <button class="winctl-btn winctl-btn--close" @click="closeWindow" title="閉じる">
        <v-icon size="14">mdi-close</v-icon>
      </button>
    </template>
  </div>

  <v-dialog v-model="aboutDialog" max-width="400">
    <v-card>
      <v-card-title class="d-flex align-center">
        <img :src="appIconUrl" class="about-icon" alt="" />
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
  height: var(--toolbar-height);
  flex-shrink: 0;
  overflow: hidden;
}

.toolbar-logo {
  width: var(--activity-bar-width);
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: flex-start;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  height: var(--toolbar-height);
  padding: 0 10px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 0;
}

.icon-btn:hover {
  background: rgba(var(--v-theme-on-surface), 0.08);
}

.app-icon {
  height: 16px;
  width: auto;
  display: block;
}

.about-icon {
  height: 22px;
  width: auto;
  margin-right: 8px;
}

.menu-btn {
  height: var(--toolbar-height) !important;
  border-radius: 0 !important;
  font-size: var(--ui-font-size-sm);
  min-width: 0;
  padding: 0 10px;
  align-self: stretch;
}

.menu-btn:hover {
  background: rgba(var(--v-theme-on-surface), 0.08) !important;
}

.winctl-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 46px;
  height: var(--toolbar-height);
  border: none;
  background: transparent;
  cursor: pointer;
  color: rgba(var(--v-theme-on-surface), 0.7);
  border-radius: 0;
  flex-shrink: 0;
  transition: background 0.1s, color 0.1s;
}

.winctl-btn:hover {
  background: rgba(var(--v-theme-on-surface), 0.12);
  color: rgba(var(--v-theme-on-surface), 1);
}

.winctl-btn--close:hover {
  background: #e81123;
  color: #fff;
}
</style>
