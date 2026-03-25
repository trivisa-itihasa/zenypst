import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useEditorStore } from "@/stores/editor";
import { usePreviewStore } from "@/stores/preview";
import { useSettingsStore } from "@/stores/settings";
import type { CompileResult, CompileError } from "@/types";

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

// Module-level watcher state (singleton across all useCompiler() calls)
let watcherUnlisten: UnlistenFn | null = null;
let watcherReadyUnlisten: UnlistenFn | null = null;
let watcherDiedUnlisten: UnlistenFn | null = null;
let watcherActive = false;
let watcherStarting = false; // 重複起動防止

interface WatchEvent {
  success: boolean;
  pdfPath?: string;
  errors: CompileError[];
  warnings: CompileError[];
}

/**
 * Composable that manages Typst compilation based on preview mode.
 * - realtime / on_save: uses `typst watch` (lazy-started on first compile request)
 * - manual: uses `typst compile` directly
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

  /**
   * Start `typst watch` (internal).
   * Called lazily the first time a compile is needed in realtime/on_save mode.
   */
  async function startWatcher(): Promise<void> {
    if (watcherActive || watcherStarting) return;
    watcherStarting = true;

    // Clean up any stale listeners
    watcherUnlisten?.();
    watcherUnlisten = null;
    watcherReadyUnlisten?.();
    watcherReadyUnlisten = null;
    watcherDiedUnlisten?.();
    watcherDiedUnlisten = null;

    try {
      const tempInput = await invoke<string>("get_temp_input_path");
      const tempOutput = await invoke<string>("get_temp_output_path");
      const typstPath = settingsStore.settings.typstPath || undefined;

      watcherUnlisten = await listen<WatchEvent>("typst-watch-result", (event) => {
        const result = event.payload;
        if (result.success && result.pdfPath) {
          previewStore.setSuccess(result.pdfPath);
        } else {
          previewStore.setError(result.errors ?? [], result.warnings ?? []);
        }
      });

      watcherReadyUnlisten = await listen("typst-watch-ready", () => {
        watcherActive = true;
        watcherStarting = false;
      });

      watcherDiedUnlisten = await listen("typst-watch-died", () => {
        watcherActive = false;
        watcherStarting = false;
        previewStore.setIdle();
      });

      await invoke("start_typst_watch", { input: tempInput, output: tempOutput, typstPath });
    } catch (err) {
      console.error("Failed to start typst watcher:", err);
      watcherActive = false;
      watcherStarting = false;
    }
  }

  /** Stop the `typst watch` process (called on app unmount or mode change to manual). */
  async function stopWatcher(): Promise<void> {
    watcherUnlisten?.();
    watcherUnlisten = null;
    watcherReadyUnlisten?.();
    watcherReadyUnlisten = null;
    watcherDiedUnlisten?.();
    watcherDiedUnlisten = null;
    watcherActive = false;
    watcherStarting = false;
    try {
      await invoke("stop_typst_watch");
    } catch {
      // ignore
    }
  }

  /**
   * Write content to the temp input file to trigger watcher recompilation.
   * Caller must ensure watcherActive === true before calling.
   */
  async function writeForWatcher(content: string): Promise<void> {
    try {
      const tempInput = await invoke<string>("get_temp_input_path");
      previewStore.setCompiling();
      await invoke("write_file", { path: tempInput, content });
    } catch (err) {
      previewStore.setError([{ severity: "error", message: String(err) }], []);
    }
  }

  /** Run a full compile via `typst compile` (manual mode or watcher fallback). */
  async function compile(inputPath: string, content: string): Promise<void> {
    if (isTypstInstalled.value === null) {
      await checkTypstInstalled();
    }
    if (!isTypstInstalled.value) return;

    previewStore.setCompiling();

    try {
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
      previewStore.setError([{ severity: "error", message: String(err) }], []);
    }
  }

  /** Schedule a compile with debounce (realtime mode). */
  function scheduleCompile(path: string | null, content: string): void {
    const mode = settingsStore.settings.previewMode;
    if (mode !== "realtime") return;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      if (mode === "realtime") {
        if (watcherActive) {
          await writeForWatcher(content);
        } else {
          // watcher 未起動の間は直接コンパイル
          await compile(path ?? "untitled.typ", content);
          // バックグラウンドで watcher を起動しておく
          startWatcher().catch(console.error);
        }
      }
    }, settingsStore.settings.realtimeDebounceMs);
  }

  /** Trigger compile immediately (for on_save / manual modes). */
  async function triggerCompile(): Promise<void> {
    const tab = editorStore.activeTab;
    if (!tab) return;

    const mode = settingsStore.settings.previewMode;
    if (mode === "manual") {
      await compile(tab.path ?? "untitled.typ", tab.content);
    } else if (watcherActive) {
      await writeForWatcher(tab.content);
    } else {
      await compile(tab.path ?? "untitled.typ", tab.content);
      startWatcher().catch(console.error);
    }
  }

  return {
    isTypstInstalled,
    checkTypstInstalled,
    compile,
    scheduleCompile,
    triggerCompile,
    startWatcher,
    stopWatcher,
  };
}
