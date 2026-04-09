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

const MIN_FILE_TREE = 180;
const MIN_PREVIEW = 200;
const MIN_EDITOR = 150;

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
  await stopWatcher();
});

onMounted(async () => {
  await settingsStore.load();
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
  <div class="app-shell d-flex flex-column fill-height">
    <Toolbar
      @new-file="templatePickerDialog = true"
      @open-templates="templateManagerDialog = true"
    />

    <div
      v-if="typstNotFoundBar"
      class="typst-not-found-bar d-flex align-center px-3"
    >
      <q-icon name="mdi-alert" size="14px" color="warning" class="mr-2" />
      <span class="text-caption">
        <strong>Typst CLI not found.</strong>
        Install from
        <a href="https://typst.app" target="_blank" rel="noopener">typst.app</a>
        and add to PATH.
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
          <div class="text-subtitle-2">Template Manager</div>
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
  background: transparent;
  position: relative;
  z-index: 10;
}

/* Fill header area with surface color to match adjacent panel headers,
   and draw the connecting border line at the bottom */
.splitter::before {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  height: var(--panel-header-height);
  background: var(--zen-surface);
  border-bottom: 1px solid var(--zen-border);
}

/* Vertical separator line — hidden at rest, visible on hover/drag */
.splitter::after {
  content: "";
  position: absolute;
  top: 0;
  bottom: 0;
  left: 50%;
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
  height: var(--toolbar-height);
  min-height: var(--toolbar-height);
  background: rgba(var(--zen-warning-rgb), 0.15);
  border-bottom: 1px solid rgba(var(--zen-warning-rgb), 0.4);
  flex-shrink: 0;
  font-size: var(--ui-font-size-sm);

  a {
    color: var(--zen-warning);
  }
}
</style>
