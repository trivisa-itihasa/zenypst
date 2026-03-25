<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
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
const { checkTypstInstalled, stopWatcher } = useCompiler();
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

onUnmounted(async () => {
  await stopWatcher();
});

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
      const fileTreeStore = useFileTreeStore();
      await fileTreeStore.loadDirectory(path).catch(() => {
        fileOps.openFile(path).catch(() => {});
      });
    } catch {}
  }

  // Listen for new-file events from keybindings
  window.addEventListener("zenypst:new-file", () => {
    templatePickerDialog.value = true;
  });

  // manual モードに切り替えたら watcher を停止する
  watch(
    () => settingsStore.settings.previewMode,
    async (newMode) => {
      if (newMode === "manual") {
        await stopWatcher();
      }
    }
  );
});

async function handleTemplateSelected(template: Template): Promise<void> {
  await fileOps.newFile(template.content, "untitled.typ");
}

async function handleOpenFile(path: string): Promise<void> {
  await fileOps.openFile(path);
}

// View menu: show only (ドラッグで非表示になったパネルを復元する)
async function toggleFileTree(): Promise<void> {
  if (!settingsStore.settings.fileTreeVisible) {
    await settingsStore.update("fileTreeVisible", true);
  }
}

async function togglePreview(): Promise<void> {
  if (!settingsStore.settings.previewVisible) {
    await settingsStore.update("previewVisible", true);
  }
}

// ドラッグ中に幅が閾値を下回ったら非表示にスナップ。
// min-size は閾値より小さい値（1%）にして、@resize が閾値通過を確実に検知できるようにする。
const FILE_TREE_HIDE = 7; // % 以下で非表示
const PREVIEW_HIDE = 10; // % 以下で非表示
const PANE_MIN_SIZE = 1; // min-size は閾値より小さく設定（閾値到達前にmin-sizeで止まらないよう）

function onPanesResized(panes: { size: number }[]): void {
  if (settingsStore.settings.fileTreeVisible && settingsStore.settings.previewVisible) {
    // [fileTree, editor, preview]
    if (panes[0]?.size <= FILE_TREE_HIDE) settingsStore.update("fileTreeVisible", false);
    if (panes[2]?.size <= PREVIEW_HIDE) settingsStore.update("previewVisible", false);
  } else if (settingsStore.settings.fileTreeVisible) {
    // [fileTree, editor]
    if (panes[0]?.size <= FILE_TREE_HIDE) settingsStore.update("fileTreeVisible", false);
  } else if (settingsStore.settings.previewVisible) {
    // [editor, preview]
    if (panes[1]?.size <= PREVIEW_HIDE) settingsStore.update("previewVisible", false);
  }
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
      <Splitpanes style="height: 100%;" @resize="onPanesResized">
        <!-- File tree pane -->
        <Pane
          v-if="settingsStore.settings.fileTreeVisible"
          :size="fileTreeSize"
          :min-size="PANE_MIN_SIZE"
          class="pane-clip"
        >
          <FileTree @open-file="handleOpenFile" />
        </Pane>

        <!-- Editor pane -->
        <Pane :min-size="20" class="pane-clip">
          <EditorPanel
            @new-file="templatePickerDialog = true"
            @open-file="fileOps.openFileDialog"
          />
        </Pane>

        <!-- PDF preview pane -->
        <Pane
          v-if="settingsStore.settings.previewVisible"
          :size="previewSize"
          :min-size="PANE_MIN_SIZE"
          class="pane-clip"
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
