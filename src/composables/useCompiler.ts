import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useEditorStore } from "@/stores/editor";
import { usePreviewStore } from "@/stores/preview";
import { useSettingsStore } from "@/stores/settings";
import type { CompileError } from "@/types";
import { getDirectory } from "@/utils/path";

let debounceTimer: ReturnType<typeof setTimeout> | null = null;
let nativeUnlisten: UnlistenFn | null = null;
let nativeListening = false;

interface NativeRenderResult {
  success: boolean;
  pdf: string | null;
  errors: CompileError[];
  warnings: CompileError[];
}

interface PdfExportResult {
  success: boolean;
  errors: CompileError[];
  warnings: CompileError[];
}

export function useCompiler() {
  const editorStore = useEditorStore();
  const previewStore = usePreviewStore();
  const settingsStore = useSettingsStore();

  /** Ensure we're listening for native compile results. */
  async function ensureListening(): Promise<void> {
    if (nativeListening) return;
    nativeListening = true;
    nativeUnlisten = await listen<NativeRenderResult>("typst-native-result", (event) => {
      const result = event.payload;
      if (result.success && result.pdf) {
        previewStore.setSuccess(result.pdf);
      } else {
        previewStore.setError(result.errors ?? [], result.warnings ?? []);
      }
    });
  }

  /** Run native in-process compilation. */
  async function compileNative(content: string, root?: string): Promise<void> {
    await ensureListening();
    previewStore.setCompiling();
    try {
      await invoke("compile_native", { content, root: root ?? null });
    } catch (err) {
      previewStore.setError([{ severity: "error", message: String(err) }], []);
    }
  }

  /** Export the current document to a PDF file. */
  async function exportPdf(content: string, root: string | undefined, outputPath: string): Promise<PdfExportResult> {
    return await invoke<PdfExportResult>("export_pdf", {
      content,
      root: root ?? null,
      outputPath,
    });
  }

  /** Schedule a compile with debounce (realtime mode). */
  function scheduleCompile(path: string | null, content: string): void {
    const mode = settingsStore.settings.previewMode;
    if (mode !== "realtime") return;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      const root = path ? getDirectory(path) : undefined;
      compileNative(content, root).catch(console.error);
    }, settingsStore.settings.realtimeDebounceMs);
  }

  /** Trigger compile immediately (for on_save / manual modes). */
  async function triggerCompile(): Promise<void> {
    const tab = editorStore.activeTab;
    if (!tab) return;
    const root = tab.path ? getDirectory(tab.path) : undefined;
    await compileNative(tab.content, root);
  }

  /** Stop the native listener (called on unmount). */
  async function stopWatcher(): Promise<void> {
    nativeUnlisten?.();
    nativeUnlisten = null;
    nativeListening = false;
    if (debounceTimer) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
  }

  return {
    scheduleCompile,
    triggerCompile,
    exportPdf,
    stopWatcher,
    startWatcher: async () => {},
    compile: (path: string, content: string) => compileNative(content, path ? getDirectory(path) : undefined),
  };
}
