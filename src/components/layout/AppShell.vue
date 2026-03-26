<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from "vue";

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
const { stopWatcher } = useCompiler();
const { loadThemes } = useTheme();
const templateStore = useTemplateStore();

useKeybindings();

const settingsDialog = ref(false);
const templatePickerDialog = ref(false);
const templateManagerDialog = ref(false);
const typstNotFoundBar = ref(false);

// --- Panel sizing constants (px) ---
const MIN_FILE_TREE = 180;
const MIN_PREVIEW = 200;
const MIN_EDITOR = 150;

// --- Stored panel widths (px, updated after each drag) ---
const fileTreeWidth = ref(240);
const previewWidth = ref(320);

// --- Drag state ---
type DragSide = "left" | "right";
const dragging = ref<DragSide | null>(null);
const dragStartX = ref(0);
const dragStartWidth = ref(0);

// --- Virtual widths (track drag position; can go below MIN to detect hide threshold) ---
const fileTreeVirtualWidth = ref(240);
const previewVirtualWidth = ref(320);

// --- Body ref for total width measurement ---
const bodyRef = ref<HTMLElement | null>(null);

// --- Computed visibility ---
// During a drag: derived from virtual width; otherwise from persisted settings.
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

// --- Computed display widths ---
// Clamped to MIN when visible so the panel never shrinks below its minimum.
const fileTreeDisplayWidth = computed<number>(() => {
  if (!fileTreeShown.value) return 0;
  return Math.max(fileTreeVirtualWidth.value, MIN_FILE_TREE);
});

const previewDisplayWidth = computed<number>(() => {
  if (!previewShown.value) return 0;
  return Math.max(previewVirtualWidth.value, MIN_PREVIEW);
});

// --- Drag start ---
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

// --- Mouse move ---
function onMouseMove(e: MouseEvent): void {
  const dx = e.clientX - dragStartX.value;
  const bodyWidth = bodyRef.value?.clientWidth ?? window.innerWidth;

  if (dragging.value === "left") {
    // Don't let file tree eat into editor's minimum width
    const reservedForOthers = (previewShown.value ? previewDisplayWidth.value : 0) + MIN_EDITOR;
    const maxWidth = bodyWidth - reservedForOthers;
    fileTreeVirtualWidth.value = Math.min(dragStartWidth.value + dx, maxWidth);
  } else if (dragging.value === "right") {
    // Don't let preview eat into editor's minimum width
    const reservedForOthers = (fileTreeShown.value ? fileTreeDisplayWidth.value : 0) + MIN_EDITOR;
    const maxWidth = bodyWidth - reservedForOthers;
    previewVirtualWidth.value = Math.min(dragStartWidth.value - dx, maxWidth);
  }
}

// --- Mouse up: commit final state ---
function onMouseUp(): void {
  if (dragging.value === "left") {
    if (fileTreeShown.value) {
      fileTreeWidth.value = fileTreeDisplayWidth.value;
      settingsStore.update("fileTreeVisible", true);
    } else {
      settingsStore.update("fileTreeVisible", false);
    }
  } else if (dragging.value === "right") {
    if (previewShown.value) {
      previewWidth.value = previewDisplayWidth.value;
      settingsStore.update("previewVisible", true);
    } else {
      settingsStore.update("previewVisible", false);
    }
  }

  dragging.value = null;
  document.removeEventListener("mousemove", onMouseMove);
  document.removeEventListener("mouseup", onMouseUp);
}

// Apply col-resize cursor and suppress text selection during drag
watch(dragging, (val) => {
  document.body.style.cursor = val ? "col-resize" : "";
  (document.body.style as CSSStyleDeclaration & { userSelect: string }).userSelect = val ? "none" : "";
});

onUnmounted(async () => {
  document.removeEventListener("mousemove", onMouseMove);
  document.removeEventListener("mouseup", onMouseUp);
  document.body.style.cursor = "";
  (document.body.style as CSSStyleDeclaration & { userSelect: string }).userSelect = "";
  await stopWatcher();
});

onMounted(async () => {
  await settingsStore.load();
  await loadThemes();
  await templateStore.loadTemplates();

  if (settingsStore.settings.lastOpenedPath) {
    try {
      const path = settingsStore.settings.lastOpenedPath;
      const fileTreeStore = useFileTreeStore();
      await fileTreeStore.loadDirectory(path).catch(() => {
        fileOps.openFile(path).catch(() => {});
      });
    } catch {}
  }

  window.addEventListener("zenypst:new-file", () => {
    templatePickerDialog.value = true;
  });

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

// Toolbar show-panel actions (only re-show; hiding is via drag)
async function toggleFileTree(): Promise<void> {
  if (!settingsStore.settings.fileTreeVisible) {
    fileTreeVirtualWidth.value = Math.max(fileTreeWidth.value, MIN_FILE_TREE);
    fileTreeWidth.value = fileTreeVirtualWidth.value;
    await settingsStore.update("fileTreeVisible", true);
  }
}

async function togglePreview(): Promise<void> {
  if (!settingsStore.settings.previewVisible) {
    previewVirtualWidth.value = Math.max(previewWidth.value, MIN_PREVIEW);
    previewWidth.value = previewVirtualWidth.value;
    await settingsStore.update("previewVisible", true);
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

    <!-- Typst not found banner -->
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
    <div class="app-body" ref="bodyRef">
      <!-- File tree panel:
           Stays in the DOM while dragging (so the splitter tracks the cursor),
           but the FileTree content is hidden via v-show when below the threshold. -->
      <div
        v-if="fileTreeShown || dragging === 'left'"
        class="pane pane-side"
        :style="{ width: fileTreeDisplayWidth + 'px' }"
      >
        <FileTree v-show="fileTreeShown" @open-file="handleOpenFile" />
      </div>

      <!-- Left splitter -->
      <div
        v-if="fileTreeShown || dragging === 'left'"
        class="splitter"
        :class="{ 'splitter--active': dragging === 'left' }"
        @mousedown="startLeftDrag"
      />

      <!-- Editor panel (flex: 1, takes remaining space) -->
      <div class="pane pane-editor">
        <EditorPanel
          @new-file="templatePickerDialog = true"
          @open-file="fileOps.openFileDialog"
        />
      </div>

      <!-- Right splitter -->
      <div
        v-if="previewShown || dragging === 'right'"
        class="splitter"
        :class="{ 'splitter--active': dragging === 'right' }"
        @mousedown="startRightDrag"
      />

      <!-- Preview panel -->
      <div
        v-if="previewShown || dragging === 'right'"
        class="pane pane-side"
        :style="{ width: previewDisplayWidth + 'px' }"
      >
        <PdfViewer v-show="previewShown" />
      </div>
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

/* Side panels have fixed pixel widths */
.pane-side {
  flex-shrink: 0;
}

/* Editor fills remaining space */
.pane-editor {
  flex: 1;
  min-width: 0;
}

/* Divider between panels */
.splitter {
  width: 4px;
  flex-shrink: 0;
  cursor: col-resize;
  background: rgba(var(--v-border-color), var(--v-border-opacity));
  transition: background 0.1s;
  z-index: 10;
}

.splitter:hover,
.splitter--active {
  background: rgb(var(--v-theme-primary));
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
</style>
