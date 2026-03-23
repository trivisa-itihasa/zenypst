import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useEditorStore } from "@/stores/editor";
import { usePreviewStore } from "@/stores/preview";
import { useSettingsStore } from "@/stores/settings";
import type { CompileResult } from "@/types";

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

/**
 * Composable that manages Typst compilation based on preview mode.
 */
export function useCompiler() {
  const editorStore = useEditorStore();
  const previewStore = usePreviewStore();
  const settingsStore = useSettingsStore();

  const isTypstInstalled = ref<boolean | null>(null);

  /** Check if typst CLI is available. */
  async function checkTypstInstalled(): Promise<boolean> {
    try {
      const typstPath = settingsStore.settings.typstPath || undefined;
      const installed = await invoke<boolean>("check_typst_installed", { typstPath });
      isTypstInstalled.value = installed;
      return installed;
    } catch {
      isTypstInstalled.value = false;
      return false;
    }
  }

  /** Trigger a Typst compilation for the given file path and content. */
  async function compile(inputPath: string, content: string): Promise<void> {
    if (isTypstInstalled.value === null) {
      await checkTypstInstalled();
    }
    if (!isTypstInstalled.value) return;

    previewStore.setCompiling();

    try {
      // Write content to a temp input file via Rust backend
      const tempInput = await invoke<string>("get_temp_input_path");
      await invoke("write_file", { path: tempInput, content });

      const tempOutput = await invoke<string>("get_temp_output_path");

      const typstPath = settingsStore.settings.typstPath || undefined;
      const result = await invoke<CompileResult>("compile_typst", {
        input: tempInput,
        output: tempOutput,
        typstPath,
      });

      if (result.success && result.pdfPath) {
        previewStore.setSuccess(result.pdfPath);
      } else {
        previewStore.setError(result.errors, result.warnings);
      }
    } catch (err) {
      previewStore.setError(
        [{ severity: "error", message: String(err) }],
        []
      );
    }
  }

  /** Schedule a compile with debounce. */
  function scheduleCompile(path: string | null, content: string): void {
    const mode = settingsStore.settings.previewMode;
    if (mode !== "realtime") return;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      await compile(path ?? "untitled.typ", content);
    }, settingsStore.settings.realtimeDebounceMs);
  }

  /** Trigger compile immediately (for on_save / manual modes). */
  async function triggerCompile(): Promise<void> {
    const tab = editorStore.activeTab;
    if (!tab) return;
    await compile(tab.path ?? "untitled.typ", tab.content);
  }

  return {
    isTypstInstalled,
    checkTypstInstalled,
    compile,
    scheduleCompile,
    triggerCompile,
  };
}
