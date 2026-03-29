<script setup lang="ts">
import { ref, watch, onUnmounted, nextTick, computed } from "vue";
import { EditorView } from "@codemirror/view";
import { Compartment } from "@codemirror/state";
import { useEditorStore } from "@/stores/editor";
import { useSettingsStore } from "@/stores/settings";
import { useCompiler } from "@/composables/useCompiler";
import { useTheme } from "@/composables/useTheme";
import { createEditorState, buildDynamicExtensions } from "./codemirror/setup";
import TabBar from "./TabBar.vue";

const editorStore = useEditorStore();
const settingsStore = useSettingsStore();
const { scheduleCompile, triggerCompile } = useCompiler();
const { activeTheme } = useTheme();

const editorContainer = ref<HTMLElement | null>(null);
let view: EditorView | null = null;

// Intercept IME toggle keys (全角半角 etc.) in the capture phase so they never
// reach CodeMirror's keydown handler, which would log a warning and potentially
// call preventDefault. stopPropagation() in capture phase prevents the event
// from reaching descendant elements (CM's contentDOM), while the OS/IME still
// receives the key normally.
function handleImeKey(e: KeyboardEvent): void {
  if (
    e.key === "ZenkakuHankaku" ||
    e.key === "Zenkaku" ||
    e.key === "Hankaku" ||
    e.key === "Romaji" ||
    e.key === "KanaMode" ||
    e.key === "HiraganaKatakana"
  ) {
    e.stopPropagation();
  }
}

watch(editorContainer, (el, prevEl) => {
  prevEl?.removeEventListener("keydown", handleImeKey, { capture: true });
  el?.addEventListener("keydown", handleImeKey, { capture: true });
});

// Compartments for live reconfiguration
const dynamicCompartment = new Compartment();

/** Mount or update the editor when the active tab changes. */
const activeTab = computed(() => editorStore.activeTab);

function mountEditor(content: string): void {
  if (!editorContainer.value) return;

  // Destroy existing view
  if (view) {
    view.destroy();
    view = null;
  }

  const state = createEditorState({
    doc: content,
    themeColors: activeTheme.value.colors,
    fontFamily: settingsStore.settings.fontFamily,
    fontFamilyFallback: settingsStore.settings.fontFamilyFallback,
    fontSize: settingsStore.settings.fontSize,
    showLineNumbers: settingsStore.settings.showLineNumbers,
    wordWrap: settingsStore.settings.wordWrap,
    onChange: (newContent: string) => {
      if (editorStore.activeTabId) {
        editorStore.updateContent(editorStore.activeTabId, newContent);
        scheduleCompile(editorStore.activeTab?.path ?? null, newContent);
      }
    },
  });

  view = new EditorView({
    state,
    parent: editorContainer.value,
  });
}

// Watch for jump requests from PDF viewer (double-click sync)
watch(
  () => editorStore.jumpRequest,
  (req) => {
    if (!req || !view) return;
    editorStore.clearJumpRequest();
    const doc = view.state.doc;
    // line is 1-indexed; CodeMirror doc.line() is also 1-indexed
    const lineNum = Math.min(Math.max(req.line, 1), doc.lines);
    const line = doc.line(lineNum);
    const pos = Math.min(line.from + req.col - 1, line.to);
    view.dispatch({
      selection: { anchor: pos },
      scrollIntoView: true,
    });
    view.focus();
  }
);

// Watch for active tab changes
watch(
  activeTab,
  async (tab, prevTab) => {
    await nextTick();
    if (tab) {
      mountEditor(tab.content);
      // In realtime mode, re-compile when switching tabs so the PDF matches
      if (prevTab !== undefined && settingsStore.settings.previewMode === "realtime") {
        triggerCompile().catch(console.error);
      }
    } else if (view) {
      view.destroy();
      view = null;
    }
  },
  { immediate: true }
);

// Watch for theme/font/editor-option changes
watch(
  [
    () => activeTheme.value.colors,
    () => settingsStore.settings.fontFamily,
    () => settingsStore.settings.fontFamilyFallback,
    () => settingsStore.settings.fontSize,
    () => settingsStore.settings.wordWrap,
    () => settingsStore.settings.showLineNumbers,
  ],
  ([colors, fontFamily, fontSize]) => {
    if (view) {
      // Remount to apply new settings
      if (activeTab.value) {
        const currentContent = view.state.doc.toString();
        mountEditor(currentContent);
      }
    }
  }
);

onUnmounted(() => {
  editorContainer.value?.removeEventListener("keydown", handleImeKey, { capture: true });
  if (view) {
    view.destroy();
  }
});
</script>

<template>
  <div class="editor-panel">
    <TabBar />
    <div
      v-if="activeTab"
      ref="editorContainer"
      class="editor-container"
    />
    <div v-else class="editor-empty">
      <div class="editor-empty__top" />
      <v-icon size="64" color="medium-emphasis">mdi-file-document-outline</v-icon>
      <p class="text-medium-emphasis mt-4">Open a file to start editing</p>
      <div class="editor-empty__bottom">
        <div class="editor-empty__actions">
          <v-btn variant="text" prepend-icon="mdi-file-plus" class="empty-action-btn" @click="$emit('new-file')">
            New File
          </v-btn>
          <v-btn variant="text" prepend-icon="mdi-folder-open" class="empty-action-btn" @click="$emit('open-file')">
            Open File
          </v-btn>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor-empty {
  position: absolute;
  inset: var(--panel-header-height) 0 0 0;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.editor-empty__top {
  flex: 2;
}

/* Bottom spacer is position:relative so actions can be absolutely placed
   without affecting the flex spacer calculation (icon aligns with PdfViewer) */
.editor-empty__bottom {
  flex: 3;
  position: relative;
  align-self: stretch;
}

.editor-empty__actions {
  position: absolute;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  gap: 8px;
  white-space: nowrap;
}

.empty-action-btn {
  justify-content: flex-start !important;
}

.editor-panel {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-container {
  flex: 1 1 0;
  overflow: hidden;
}

/* Make CodeMirror fill the container */
.editor-container :deep(.cm-editor) {
  height: 100%;
}

.editor-container :deep(.cm-scroller) {
  overflow: auto;
  height: 100%;
  scrollbar-width: thin;
  scrollbar-color: rgba(var(--v-border-color), 0.4) transparent;
}

.editor-container :deep(.cm-scroller::-webkit-scrollbar) {
  width: var(--scrollbar-size);
  height: var(--scrollbar-size);
}
.editor-container :deep(.cm-scroller::-webkit-scrollbar-thumb) {
  background: rgba(var(--v-border-color), 0.4);
  border-radius: var(--scrollbar-radius);
}
.editor-container :deep(.cm-scroller::-webkit-scrollbar-track) {
  background: transparent;
}

.editor-empty {
  background: rgb(var(--v-theme-background));
}
</style>
