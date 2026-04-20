<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from "vue";

import Toolbar from "./Toolbar.vue";
import ActivityBar from "./ActivityBar.vue";
import StatusBar from "./StatusBar.vue";
import FileTree from "@/components/sidebar/FileTree.vue";
import EditorPanel from "@/components/editor/EditorPanel.vue";
import PdfViewer from "@/components/preview/PdfViewer.vue";
import SettingsDialog from "@/components/settings/SettingsDialog.vue";
import TemplatePickerDialog from "@/components/template/TemplatePickerDialog.vue";
import TemplateManager from "@/components/template/TemplateManager.vue";

import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
import { useEditorStore } from "@/stores/editor";
import { useFileTreeStore } from "@/stores/fileTree";
import { useFileOps } from "@/composables/useFileOps";
import { useCompiler } from "@/composables/useCompiler";
import { useTheme } from "@/composables/useTheme";
import { useKeybindings } from "@/composables/useKeybindings";
import { useTemplateStore } from "@/stores/template";
import { getDirectory } from "@/utils/path";
import { Notify } from "quasar";
import type { Template } from "@/types";

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
const isMac = navigator.platform.toUpperCase().includes("MAC");

const { t } = useI18n();

const settingsStore = useSettingsStore();
const editorStore = useEditorStore();
const fileOps = useFileOps();
const { stopWatcher, triggerCompile, exportPdf } = useCompiler();
const { loadThemes } = useTheme();
const templateStore = useTemplateStore();

useKeybindings();

const settingsDialog = ref(false);
const templatePickerDialog = ref(false);
const templateManagerDialog = ref(false);
const typstNotFoundBar = ref(false);

const MIN_FILE_TREE = 180;
const MIN_PREVIEW = 200;
const MIN_EDITOR = 200;

const fileTreeWidth = ref(240);
const previewWidth = ref(320);

type DragSide = "left" | "right";
const dragging = ref<DragSide | null>(null);
const dragStartX = ref(0);
const dragStartWidth = ref(0);

const fileTreeVirtualWidth = ref(240);
const previewVirtualWidth = ref(320);

const bodyRef = ref<HTMLElement | null>(null);

const fileTreeShown = computed<boolean>(() => {
  if (dragging.value === "left") {
    return fileTreeVirtualWidth.value >= MIN_FILE_TREE / 2;
  }
  return settingsStore.settings.fileTreeVisible;
});

const previewShown = computed<boolean>(() => {
  if (dragging.value === "right") {
    return previewVirtualWidth.value >= MIN_PREVIEW / 2;
  }
  return settingsStore.settings.previewVisible;
});

const fileTreeDisplayWidth = computed<number>(() => {
  if (!fileTreeShown.value) return 0;
  return Math.max(fileTreeVirtualWidth.value, MIN_FILE_TREE);
});

const previewDisplayWidth = computed<number>(() => {
  if (!previewShown.value) return 0;
  return Math.max(previewVirtualWidth.value, MIN_PREVIEW);
});

function startLeftDrag(e: MouseEvent): void {
  e.preventDefault();
  fileTreeVirtualWidth.value = fileTreeWidth.value;
  dragStartX.value = e.clientX;
  dragStartWidth.value = fileTreeWidth.value;
  dragging.value = "left";
  document.addEventListener("mousemove", onMouseMove);
  document.addEventListener("mouseup", onMouseUp);
}

function startRightDrag(e: MouseEvent): void {
  e.preventDefault();
  previewVirtualWidth.value = previewWidth.value;
  dragStartX.value = e.clientX;
  dragStartWidth.value = previewWidth.value;
  dragging.value = "right";
  document.addEventListener("mousemove", onMouseMove);
  document.addEventListener("mouseup", onMouseUp);
}

function onMouseMove(e: MouseEvent): void {
  const dx = e.clientX - dragStartX.value;
  const bodyWidth = bodyRef.value?.clientWidth ?? window.innerWidth;

  if (dragging.value === "left") {
    const reservedForOthers = (previewShown.value ? previewDisplayWidth.value : 0) + MIN_EDITOR;
    const maxWidth = bodyWidth - reservedForOthers;
    fileTreeVirtualWidth.value = Math.min(dragStartWidth.value + dx, maxWidth);
  } else if (dragging.value === "right") {
    const reservedForOthers = (fileTreeShown.value ? fileTreeDisplayWidth.value : 0) + MIN_EDITOR;
    const maxWidth = bodyWidth - reservedForOthers;
    previewVirtualWidth.value = Math.min(dragStartWidth.value - dx, maxWidth);
  }
}

function onMouseUp(): void {
  if (dragging.value === "left") {
    const nowVisible = fileTreeShown.value;
    if (nowVisible) fileTreeWidth.value = fileTreeDisplayWidth.value;
    if (nowVisible !== settingsStore.settings.fileTreeVisible) {
      settingsStore.update("fileTreeVisible", nowVisible);
    }
  } else if (dragging.value === "right") {
    const nowVisible = previewShown.value;
    if (nowVisible) previewWidth.value = previewDisplayWidth.value;
    if (nowVisible !== settingsStore.settings.previewVisible) {
      settingsStore.update("previewVisible", nowVisible);
    }
  }

  dragging.value = null;
  document.removeEventListener("mousemove", onMouseMove);
  document.removeEventListener("mouseup", onMouseUp);
}

watch(dragging, (val) => {
  document.body.style.cursor = val ? "col-resize" : "";
  (document.body.style as CSSStyleDeclaration & { userSelect: string }).userSelect = val ? "none" : "";
});

// Stop the realtime watcher when preview mode switches to manual
watch(
  () => settingsStore.settings.previewMode,
  async (newMode) => {
    if (newMode === "manual") {
      await stopWatcher();
    }
  }
);

onUnmounted(async () => {
  document.removeEventListener("mousemove", onMouseMove);
  document.removeEventListener("mouseup", onMouseUp);
  document.body.style.cursor = "";
  (document.body.style as CSSStyleDeclaration & { userSelect: string }).userSelect = "";
  menuUnlisteners.forEach((fn) => fn());
  await stopWatcher();
});

function applyUiFont(): void {
  const { fontFamily, fontFamilyFallback } = settingsStore.settings;
  document.documentElement.style.setProperty(
    "--ui-font-family",
    `"${fontFamily}", "${fontFamilyFallback}", monospace`
  );
}

watch(
  [() => settingsStore.settings.fontFamily, () => settingsStore.settings.fontFamilyFallback],
  applyUiFont
);

async function handleMenuSave(): Promise<void> {
  await fileOps.saveActiveFile();
  if (settingsStore.settings.previewMode === "on_save") {
    await triggerCompile();
  }
}

async function handleMenuExportPdf(): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  const tab = editorStore.activeTab;
  if (!tab) return;

  const defaultName = tab.name.replace(/\.typ$/, "") + ".pdf";
  const outputPath = await invoke<string | null>("save_file_dialog", { defaultName });
  if (!outputPath) return;

  const root = tab.path ? getDirectory(tab.path) : undefined;
  try {
    const result = await exportPdf(tab.content, root, outputPath);
    if (result.success) {
      Notify.create({ message: t("toolbar.pdfExportedSuccessfully"), type: "positive", position: "bottom", timeout: 4000 });
    } else {
      const msg = result.errors.map((e) => e.message).join("; ");
      Notify.create({ message: t("toolbar.exportFailed", { msg }), type: "negative", position: "bottom", timeout: 4000 });
    }
  } catch (err) {
    Notify.create({ message: t("toolbar.exportFailed", { msg: String(err) }), type: "negative", position: "bottom", timeout: 4000 });
  }
}

const menuUnlisteners: Array<() => void> = [];

onMounted(async () => {
  await settingsStore.load();
  applyUiFont();
  await Promise.all([loadThemes(), templateStore.loadTemplates()]);

  if (settingsStore.settings.lastOpenedPath) {
    const path = settingsStore.settings.lastOpenedPath;
    const fileTreeStore = useFileTreeStore();
    await fileTreeStore.loadDirectory(path);
    if (fileTreeStore.error) {
      fileTreeStore.error = null;
      settingsStore.settings.lastOpenedPath = null;
      await settingsStore.save();
    }
  }

  window.addEventListener("zenypst:new-file", () => {
    templatePickerDialog.value = true;
  });

  // Listen for native menu events (Tauri only)
  if (isTauri) {
    const { listen } = await import("@tauri-apps/api/event");
    const handlers: Record<string, () => void | Promise<void>> = {
      "new-file": () => { templatePickerDialog.value = true; },
      "open-file": () => { fileOps.openFileDialog(); },
      "open-folder": () => { fileOps.openFolderDialog(); },
      "save": () => { handleMenuSave(); },
      "save-as": () => { fileOps.saveAsActiveFile(); },
      "export-pdf": () => { handleMenuExportPdf(); },
      "manage-templates": () => { templateManagerDialog.value = true; },
    };
    for (const [event, handler] of Object.entries(handlers)) {
      const unlisten = await listen(event, handler);
      menuUnlisteners.push(unlisten);
    }
  }
});

async function handleTemplateSelected(template: Template): Promise<void> {
  await fileOps.newFile(template.content, "untitled.typ");
}

async function handleOpenFile(path: string): Promise<void> {
  await fileOps.openFile(path);
}

async function toggleFileTree(): Promise<void> {
  if (!settingsStore.settings.fileTreeVisible) {
    fileTreeVirtualWidth.value = Math.max(fileTreeWidth.value, MIN_FILE_TREE);
    await settingsStore.update("fileTreeVisible", true);
  } else {
    await settingsStore.update("fileTreeVisible", false);
  }
}

async function togglePreview(): Promise<void> {
  if (!settingsStore.settings.previewVisible) {
    previewVirtualWidth.value = Math.max(previewWidth.value, MIN_PREVIEW);
    await settingsStore.update("previewVisible", true);
  } else {
    await settingsStore.update("previewVisible", false);
  }
}
</script>

<template>
  <div class="app-shell d-flex flex-column fill-height" :class="{ 'app-shell--native-menu': isMac }">
    <Toolbar
      v-if="!isMac"
      @new-file="templatePickerDialog = true"
      @open-templates="templateManagerDialog = true"
    />

    <div
      v-if="typstNotFoundBar"
      class="typst-not-found-bar d-flex align-center px-3"
    >
      <q-icon name="mdi-alert" size="14px" color="warning" class="mr-2" />
      <span class="text-caption">
        <strong>{{ t('appShell.typstNotFound') }}</strong>
        {{ t('appShell.typstInstallFrom') }}
        <a href="https://typst.app" target="_blank" rel="noopener">{{ t('appShell.typstInstallSite') }}</a>
        {{ t('appShell.typstInstallSuffix') }}
      </span>
      <q-space />
      <q-btn dense flat round size="xs" icon="mdi-close" @click="typstNotFoundBar = false" />
    </div>

    <div class="app-body" ref="bodyRef">
      <ActivityBar
        :file-tree-visible="settingsStore.settings.fileTreeVisible"
        :preview-visible="settingsStore.settings.previewVisible"
        @toggle-file-tree="toggleFileTree"
        @toggle-preview="togglePreview"
        @open-settings="settingsDialog = true"
      />

      <!-- File tree panel stays in the DOM while dragging so the splitter tracks
           the cursor, but FileTree content is hidden via v-show below the threshold. -->
      <div
        v-if="fileTreeShown || dragging === 'left'"
        class="pane pane-side"
        :style="{ width: fileTreeDisplayWidth + 'px', pointerEvents: dragging ? 'none' : undefined }"
      >
        <FileTree v-show="fileTreeShown" @open-file="handleOpenFile" />
      </div>

      <div
        v-if="fileTreeShown || dragging === 'left'"
        class="splitter splitter--left"
        :class="{ 'splitter--active': dragging === 'left' }"
        @mousedown="startLeftDrag"
      />

      <div class="pane pane-editor">
        <EditorPanel
          @new-file="templatePickerDialog = true"
          @open-file="fileOps.openFileDialog"
        />
      </div>

      <div
        v-if="previewShown || dragging === 'right'"
        class="splitter splitter--right"
        :class="{ 'splitter--active': dragging === 'right' }"
        @mousedown="startRightDrag"
      />

      <div
        v-if="previewShown || dragging === 'right'"
        class="pane pane-side"
        :style="{ width: previewDisplayWidth + 'px', pointerEvents: dragging ? 'none' : undefined }"
      >
        <PdfViewer v-show="previewShown" />
      </div>
    </div>

    <StatusBar />

    <SettingsDialog v-model="settingsDialog" />

    <TemplatePickerDialog
      v-model="templatePickerDialog"
      @selected="handleTemplateSelected"
    />

    <q-dialog v-model="templateManagerDialog">
      <q-card class="zen-card" style="width: 640px; max-width: 90vw;">
        <q-card-section class="row items-center q-pa-md">
          <div class="text-subtitle-2">{{ t('appShell.templateManager') }}</div>
          <q-space />
          <q-btn flat dense round icon="mdi-close" @click="templateManagerDialog = false" />
        </q-card-section>
        <q-separator />
        <q-card-section>
          <TemplateManager />
        </q-card-section>
      </q-card>
    </q-dialog>
  </div>
</template>

<style scoped>
.app-shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.app-body {
  /* height: 0 + flex-grow: 1 ensures child height: 100% resolves correctly */
  height: 0;
  flex-grow: 1;
  overflow: hidden;
  display: flex;
  flex-direction: row;
}

.pane {
  position: relative;
  overflow: hidden;
}

.pane-side {
  flex-shrink: 0;
}

.pane-editor {
  flex: 1;
  min-width: 0;
}

.splitter {
  width: var(--splitter-width);
  flex-shrink: 0;
  cursor: col-resize;
  position: relative;
  z-index: 10;
}

/* Wide transparent hit area so dragging works near the 0-width boundary */
.splitter::before {
  content: "";
  position: absolute;
  top: 0;
  bottom: 0;
  left: -4px;
  width: 8px;
}

/* Vertical separator line — hidden at rest, visible on hover/drag */
.splitter::after {
  content: "";
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  transform: translateX(-50%);
  width: 1px;
  background: transparent;
  transition: width 0.15s, background 0.15s;
}

.splitter:hover::after,
.splitter--active::after {
  width: 3px;
  background: rgba(var(--zen-on-surface-rgb), 0.4);
}

.typst-not-found-bar {
  height: 34px;
  min-height: 34px;
  background: rgba(var(--zen-warning-rgb), 0.15);
  border-bottom: 1px solid rgba(var(--zen-warning-rgb), 0.4);
  flex-shrink: 0;
  font-size: var(--ui-font-size-sm);

  a {
    color: var(--zen-warning);
  }
}

/* On macOS with native menu bar, no custom toolbar is shown */
.app-shell--native-menu {
  --toolbar-height: 0px;
}
</style>
