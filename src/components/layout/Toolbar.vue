<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Notify } from "quasar";
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

function notify(message: string, type: "negative" | "positive" = "negative"): void {
  Notify.create({ message, type, position: "bottom", timeout: 4000 });
}

async function handleNewFile(): Promise<void> {
  emit("new-file");
}

async function handleOpenFile(): Promise<void> {
  await fileOps.openFileDialog();
}

async function handleOpenFolder(): Promise<void> {
  await fileOps.openFolderDialog();
}

async function handleSave(): Promise<void> {
  await fileOps.saveActiveFile();
  if (settingsStore.settings.previewMode === "on_save") {
    await triggerCompile();
  }
}

async function handleSaveAs(): Promise<void> {
  await fileOps.saveAsActiveFile();
}

async function handleExportPdf(): Promise<void> {
  const tab = editorStore.activeTab;
  if (!tab) return;

  const defaultName = tab.name.replace(/\.typ$/, "") + ".pdf";
  const outputPath = await invoke<string | null>("save_file_dialog", { defaultName });
  if (!outputPath) return;

  const root = tab.path ? getDirectory(tab.path) : undefined;
  try {
    const result = await exportPdf(tab.content, root, outputPath);
    if (result.success) {
      notify("PDF exported successfully.", "positive");
    } else {
      const msg = result.errors.map((e) => e.message).join("; ");
      notify("Export failed: " + msg);
    }
  } catch (err) {
    notify("Export failed: " + String(err));
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

    <q-btn flat dense no-caps class="menu-btn" label="File">
      <q-menu auto-close :offset="[0, 4]">
        <q-list dense class="zen-menu-list">
          <q-item clickable @click="handleNewFile">
            <q-item-section avatar><q-icon name="mdi-file-plus" size="16px" /></q-item-section>
            <q-item-section>New File</q-item-section>
            <q-item-section side><span class="menu-shortcut">Ctrl+N</span></q-item-section>
          </q-item>
          <q-separator />
          <q-item clickable @click="handleOpenFile">
            <q-item-section avatar><q-icon name="mdi-file-outline" size="16px" /></q-item-section>
            <q-item-section>Open File…</q-item-section>
            <q-item-section side><span class="menu-shortcut">Ctrl+O</span></q-item-section>
          </q-item>
          <q-item clickable @click="handleOpenFolder">
            <q-item-section avatar><q-icon name="mdi-folder-open" size="16px" /></q-item-section>
            <q-item-section>Open Folder…</q-item-section>
            <q-item-section side><span class="menu-shortcut">Ctrl+Shift+O</span></q-item-section>
          </q-item>
          <q-separator />
          <q-item clickable :disable="!editorStore.activeTab" @click="handleSave">
            <q-item-section avatar><q-icon name="mdi-content-save" size="16px" /></q-item-section>
            <q-item-section>Save</q-item-section>
            <q-item-section side><span class="menu-shortcut">Ctrl+S</span></q-item-section>
          </q-item>
          <q-item clickable :disable="!editorStore.activeTab" @click="handleSaveAs">
            <q-item-section avatar><q-icon name="mdi-content-save-edit" size="16px" /></q-item-section>
            <q-item-section>Save As…</q-item-section>
          </q-item>
          <q-separator />
          <q-item clickable :disable="!editorStore.activeTab" @click="handleExportPdf">
            <q-item-section avatar><q-icon name="mdi-file-pdf-box" size="16px" /></q-item-section>
            <q-item-section>Export PDF…</q-item-section>
          </q-item>
          <q-separator />
          <q-item clickable @click="emit('open-templates')">
            <q-item-section avatar><q-icon name="mdi-file-document-multiple" size="16px" /></q-item-section>
            <q-item-section>Manage Templates</q-item-section>
          </q-item>
        </q-list>
      </q-menu>
    </q-btn>

    <q-space />

    <template v-if="isTauri">
      <button class="winctl-btn" @click="minimize" title="最小化">
        <q-icon name="mdi-minus" size="14px" />
      </button>
      <button class="winctl-btn" @click="toggleMaximize" :title="isMaximized ? '元のサイズに戻す' : '最大化'">
        <q-icon :name="isMaximized ? 'mdi-window-restore' : 'mdi-window-maximize'" size="14px" />
      </button>
      <button class="winctl-btn winctl-btn--close" @click="closeWindow" title="閉じる">
        <q-icon name="mdi-close" size="14px" />
      </button>
    </template>
  </div>

  <q-dialog v-model="aboutDialog">
    <q-card class="zen-card" style="width: 400px; max-width: 90vw;">
      <q-card-section class="row items-center q-pa-md">
        <img :src="appIconUrl" class="about-icon" alt="" />
        <div class="text-subtitle-2">Zenypst</div>
      </q-card-section>
      <q-card-section>
        <p class="mb-2">A desktop Typst editor with live PDF preview.</p>
        <p class="text-caption text-medium-emphasis">Version 0.1.0</p>
        <p class="text-caption text-medium-emphasis">Built with Tauri, Vue 3, Quasar, and CodeMirror 6.</p>
      </q-card-section>
      <q-card-actions align="right">
        <q-btn flat label="Close" @click="aboutDialog = false" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<style scoped>
.toolbar {
  background: var(--zen-surface);
  border-bottom: 1px solid var(--zen-border);
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
  height: 28px;
  width: 28px;
  padding: 0;
  margin: 0 6px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 6px;
}

.icon-btn:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.08);
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
  height: 28px;
  border-radius: 6px;
  font-size: var(--ui-font-size-sm);
  min-width: 0;
  padding: 0 10px;
}

.menu-btn:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.08);
}

.menu-shortcut {
  color: rgba(var(--zen-on-surface-rgb), 0.45);
  font-size: 0.75rem;
  white-space: nowrap;
  margin-left: auto;
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
  color: rgba(var(--zen-on-surface-rgb), 0.7);
  border-radius: 0;
  flex-shrink: 0;
  transition: background 0.1s, color 0.1s;
}

.winctl-btn:hover {
  background: rgba(var(--zen-on-surface-rgb), 0.12);
  color: rgba(var(--zen-on-surface-rgb), 1);
}

.winctl-btn--close:hover {
  background: #e81123;
  color: #fff;
}
</style>

<style>
/* Global tweaks for the menus rendered into a Quasar portal */
.zen-menu-list .q-item {
  min-height: 28px;
  padding-top: 2px;
  padding-bottom: 2px;
  padding-left: 8px;
  font-size: var(--ui-font-size-sm);
}
.zen-menu-list .q-item__section--avatar {
  min-width: 24px;
  padding-right: 4px;
}
</style>
