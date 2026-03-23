<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick, computed } from "vue";
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
const { scheduleCompile } = useCompiler();
const { activeTheme } = useTheme();

const editorContainer = ref<HTMLElement | null>(null);
let view: EditorView | null = null;

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

// Watch for active tab changes
watch(
  activeTab,
  async (tab) => {
    await nextTick();
    if (tab) {
      mountEditor(tab.content);
    } else if (view) {
      view.destroy();
      view = null;
    }
  },
  { immediate: true }
);

// Watch for theme/font changes
watch(
  [
    () => activeTheme.value.colors,
    () => settingsStore.settings.fontFamily,
    () => settingsStore.settings.fontSize,
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
  if (view) {
    view.destroy();
  }
});
</script>

<template>
  <div class="editor-panel d-flex flex-column fill-height">
    <TabBar />
    <div
      v-if="activeTab"
      ref="editorContainer"
      class="editor-container flex-grow-1"
    />
    <div
      v-else
      class="editor-empty d-flex flex-column align-center justify-center fill-height"
    >
      <v-icon size="64" color="medium-emphasis">mdi-file-document-outline</v-icon>
      <p class="text-medium-emphasis mt-4">Open a file to start editing</p>
      <div class="d-flex gap-2 mt-4">
        <v-btn variant="outlined" prepend-icon="mdi-file-plus" @click="$emit('new-file')">
          New File
        </v-btn>
        <v-btn variant="outlined" prepend-icon="mdi-folder-open" @click="$emit('open-file')">
          Open File
        </v-btn>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor-panel {
  overflow: hidden;
}

.editor-container {
  overflow: hidden;
  flex: 1;
}

/* Make CodeMirror fill the container */
.editor-container :deep(.cm-editor) {
  height: 100%;
}

.editor-container :deep(.cm-scroller) {
  overflow: auto;
  height: 100%;
}

.editor-empty {
  background: rgb(var(--v-theme-background));
}
</style>

<script lang="ts">
export default { emits: ["new-file", "open-file"] };
</script>
