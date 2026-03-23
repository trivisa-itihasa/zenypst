import { onMounted, onUnmounted } from "vue";
import { useFileOps } from "./useFileOps";
import { useCompiler } from "./useCompiler";
import { useEditorStore } from "@/stores/editor";
import { useSettingsStore } from "@/stores/settings";

/**
 * Composable that registers global keyboard shortcuts.
 */
export function useKeybindings() {
  const fileOps = useFileOps();
  const compiler = useCompiler();
  const editorStore = useEditorStore();
  const settingsStore = useSettingsStore();

  async function handleKeydown(event: KeyboardEvent): Promise<void> {
    const ctrl = event.ctrlKey || event.metaKey;
    const shift = event.shiftKey;

    if (!ctrl) return;

    switch (event.key) {
      case "s":
        event.preventDefault();
        await fileOps.saveActiveFile();
        // Trigger compile on save if in on_save mode
        if (settingsStore.settings.previewMode === "on_save") {
          await compiler.triggerCompile();
        }
        break;

      case "n":
        event.preventDefault();
        // Emit event for NewFile dialog; handled by App.vue
        window.dispatchEvent(new CustomEvent("zenypst:new-file"));
        break;

      case "o":
        event.preventDefault();
        if (shift) {
          await fileOps.openFolderDialog();
        } else {
          await fileOps.openFileDialog();
        }
        break;

      case "+":
      case "=":
        event.preventDefault();
        if (settingsStore.settings.fontSize < 32) {
          await settingsStore.update("fontSize", settingsStore.settings.fontSize + 1);
        }
        break;

      case "-":
        event.preventDefault();
        if (settingsStore.settings.fontSize > 8) {
          await settingsStore.update("fontSize", settingsStore.settings.fontSize - 1);
        }
        break;

      case "0":
        event.preventDefault();
        await settingsStore.update("fontSize", 14);
        break;

      case "w":
        event.preventDefault();
        if (editorStore.activeTabId) {
          editorStore.closeTab(editorStore.activeTabId);
        }
        break;
    }
  }

  onMounted(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onUnmounted(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
}
