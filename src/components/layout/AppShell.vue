<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Splitpanes, Pane } from "splitpanes";
import "splitpanes/dist/splitpanes.css";

import Toolbar from "./Toolbar.vue";
import StatusBar from "./StatusBar.vue";
import FileTree from "@/components/sidebar/FileTree.vue";
import EditorPanel from "@/components/editor/EditorPanel.vue";
import PdfViewer from "@/components/preview/PdfViewer.vue";
import SettingsDialog from "@/components/settings/SettingsDialog.vue";
import TemplatePickerDialog from "@/components/template/TemplatePickerDialog.vue";
import TemplateManager from "@/components/template/TemplateManager.vue";

import { useSettingsStore } from "@/stores/settings";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import { useCompiler } from "@/composables/useCompiler";
import { useTheme } from "@/composables/useTheme";
import { useKeybindings } from "@/composables/useKeybindings";
import { useTemplateStore } from "@/stores/template";
import type { Template } from "@/types";

const settingsStore = useSettingsStore();
const fileOps = useFileOps();
const { checkTypstInstalled } = useCompiler();
const { loadThemes } = useTheme();
const templateStore = useTemplateStore();

useKeybindings();

const settingsDialog = ref(false);
const templatePickerDialog = ref(false);
const templateManagerDialog = ref(false);
const typstNotFoundBar = ref(false);

// Splitpanes sizes (percentages)
const fileTreeSize = ref(18); // ~250px in 1400px window
const previewSize = ref(35);

onMounted(async () => {
  await settingsStore.load();
  await loadThemes();
  await templateStore.loadTemplates();

  const installed = await checkTypstInstalled();
  if (!installed) {
    typstNotFoundBar.value = true;
  }

  // Restore last opened path
  if (settingsStore.settings.lastOpenedPath) {
    try {
      const path = settingsStore.settings.lastOpenedPath;
      // Try as directory first
      const fileTreeStore = useFileTreeStore();
      await fileTreeStore.loadDirectory(path).catch(() => {
        // If failed, try as file
        fileOps.openFile(path).catch(() => {});
      });
    } catch {}
  }

  // Listen for new-file events from keybindings
  window.addEventListener("zenypst:new-file", () => {
    templatePickerDialog.value = true;
  });
});

async function handleTemplateSelected(template: Template): Promise<void> {
  await fileOps.newFile(template.content, "untitled.typ");
}

async function handleOpenFile(path: string): Promise<void> {
  await fileOps.openFile(path);
}

async function toggleFileTree(): Promise<void> {
  await settingsStore.update("fileTreeVisible", !settingsStore.settings.fileTreeVisible);
}

async function togglePreview(): Promise<void> {
  await settingsStore.update("previewVisible", !settingsStore.settings.previewVisible);
}
</script>

<template>
  <div class="app-shell d-flex flex-column fill-height">
    <!-- Toolbar -->
    <Toolbar
      @new-file="templatePickerDialog = true"
      @open-settings="settingsDialog = true"
      @open-templates="templateManagerDialog = true"
      @toggle-file-tree="toggleFileTree"
      @toggle-preview="togglePreview"
    />

    <!-- Typst not found banner (Toolbarの直下) -->
    <div
      v-if="typstNotFoundBar"
      class="typst-not-found-bar d-flex align-center px-3"
    >
      <v-icon size="14" color="warning" class="mr-2">mdi-alert</v-icon>
      <span class="text-caption">
        <strong>Typst CLI not found.</strong>
        Install from
        <a href="https://typst.app" target="_blank" rel="noopener">typst.app</a>
        and add to PATH.
      </span>
      <v-spacer />
      <v-btn icon size="x-small" variant="text" @click="typstNotFoundBar = false">
        <v-icon size="14">mdi-close</v-icon>
      </v-btn>
    </div>

    <!-- Main 3-panel body -->
    <div class="app-body">
      <Splitpanes style="height: 100%;">
        <!-- File tree pane -->
        <Pane
          v-if="settingsStore.settings.fileTreeVisible"
          :size="fileTreeSize"
          :min-size="11"
          class="pane-clip"
          style="min-width: 150px;"
        >
          <FileTree @open-file="handleOpenFile" />
        </Pane>

        <!-- Editor pane -->
        <Pane :min-size="22" class="pane-clip" style="min-width: 300px;">
          <EditorPanel
            @new-file="templatePickerDialog = true"
            @open-file="fileOps.openFileDialog"
          />
        </Pane>

        <!-- PDF preview pane -->
        <Pane
          v-if="settingsStore.settings.previewVisible"
          :size="previewSize"
          :min-size="15"
          class="pane-clip"
          style="min-width: 200px;"
        >
          <PdfViewer />
        </Pane>
      </Splitpanes>
    </div>

    <!-- Status bar -->
    <StatusBar />

    <!-- Dialogs -->
    <SettingsDialog v-model="settingsDialog" />

    <TemplatePickerDialog
      v-model="templatePickerDialog"
      @selected="handleTemplateSelected"
    />

    <v-dialog v-model="templateManagerDialog" max-width="640" scrollable>
      <v-card>
        <v-card-title class="d-flex align-center">
          Template Manager
          <v-spacer />
          <v-btn icon variant="text" @click="templateManagerDialog = false">
            <v-icon>mdi-close</v-icon>
          </v-btn>
        </v-card-title>
        <v-divider />
        <v-card-text>
          <TemplateManager />
        </v-card-text>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped>
.app-shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ペイン内コンテンツが position: absolute で配置できるよう基準点を設定 */
:deep(.pane-clip) {
  position: relative !important;
  overflow: hidden !important;
}

.app-body {
  /* height: 0 + flex-grow: 1 により子要素の height: 100% が正しく解決される */
  height: 0;
  flex-grow: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.typst-not-found-bar {
  height: 24px;
  min-height: 24px;
  background: rgba(var(--v-theme-warning), 0.15);
  border-bottom: 1px solid rgba(var(--v-theme-warning), 0.4);
  flex-shrink: 0;
  font-size: 12px;

  a {
    color: rgb(var(--v-theme-warning));
  }
}

/* Style splitpanes gutters */
:deep(.splitpanes__splitter) {
  background: rgba(var(--v-border-color), var(--v-border-opacity));
  position: relative;
}

:deep(.splitpanes__splitter:hover) {
  background: rgb(var(--v-theme-primary));
}

/* 横並び（デフォルト）: 縦の仕切り線 */
:deep(.splitpanes--vertical > .splitpanes__splitter) {
  width: 4px;
  cursor: col-resize;
}

/* 縦積み: 横の仕切り線 */
:deep(.splitpanes--horizontal > .splitpanes__splitter) {
  height: 4px;
  cursor: row-resize;
}
</style>
