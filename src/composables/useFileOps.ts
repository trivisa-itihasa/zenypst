import { invoke } from "@tauri-apps/api/core";
import { useEditorStore } from "@/stores/editor";
import { useFileTreeStore } from "@/stores/fileTree";
import { useSettingsStore } from "@/stores/settings";
import { useTemplateStore } from "@/stores/template";
import type { FileFilter } from "@/types";

/**
 * Composable for file operations (open, save, create, delete).
 */
export function useFileOps() {
  const editorStore = useEditorStore();
  const fileTreeStore = useFileTreeStore();
  const settingsStore = useSettingsStore();
  const templateStore = useTemplateStore();

  /** Open a file from disk by path. */
  async function openFile(path: string): Promise<void> {
    try {
      const content = await invoke<string>("read_file", { path });
      editorStore.openTab(path, content);
      // Add to recent paths
      const recent = settingsStore.settings.recentPaths.filter((p) => p !== path);
      recent.unshift(path);
      settingsStore.settings.recentPaths = recent.slice(0, 10);
      settingsStore.settings.lastOpenedPath = path;
      await settingsStore.save();
    } catch (err) {
      throw new Error(`Failed to open file: ${err}`);
    }
  }

  /** Show a file picker dialog and open the selected file. */
  async function openFileDialog(): Promise<void> {
    const filters: FileFilter[] = [
      { name: "Typst Files", extensions: ["typ"] },
      { name: "All Files", extensions: ["*"] },
    ];
    const path = await invoke<string | null>("pick_file", { filters });
    if (path) {
      await openFile(path);
    }
  }

  /** Show a folder picker dialog and load the selected folder. */
  async function openFolderDialog(): Promise<void> {
    const path = await invoke<string | null>("pick_folder");
    if (path) {
      await fileTreeStore.loadDirectory(path);
      settingsStore.settings.lastOpenedPath = path;
      await settingsStore.save();
    }
  }

  /** Save the active tab's content. Returns true if saved. */
  async function saveActiveFile(): Promise<boolean> {
    const tab = editorStore.activeTab;
    if (!tab) return false;

    if (tab.path) {
      await invoke("write_file", { path: tab.path, content: tab.content });
      editorStore.markSaved(tab.id);
      return true;
    } else {
      return await saveAsActiveFile();
    }
  }

  /** Show save dialog and save the active tab. Returns true if saved. */
  async function saveAsActiveFile(): Promise<boolean> {
    const tab = editorStore.activeTab;
    if (!tab) return false;

    const path = await invoke<string | null>("save_file_dialog", {
      defaultName: tab.name,
    });
    if (!path) return false;

    await invoke("write_file", { path, content: tab.content });
    editorStore.markSaved(tab.id, path);
    return true;
  }

  /** Create a new file with optional template content. */
  async function newFile(content = "", fileName = "untitled.typ"): Promise<void> {
    editorStore.newUntitledTab(content, fileName);
  }

  /** Create a file on disk and open it. */
  async function createFileOnDisk(path: string, content = ""): Promise<void> {
    await invoke("create_file", { path, content });
    await openFile(path);
    await fileTreeStore.refresh();
  }

  /** Delete a file or directory from disk. */
  async function deletePath(path: string): Promise<void> {
    await invoke("delete_path", { path });
    // Close any open tab for this file
    const tab = editorStore.tabs.find((t) => t.path === path);
    if (tab) {
      editorStore.closeTab(tab.id);
    }
    await fileTreeStore.refresh();
  }

  /** Rename a path on disk. */
  async function renamePath(oldPath: string, newPath: string): Promise<void> {
    await invoke("rename_path", { old: oldPath, new: newPath });
    // Update any open tab path
    const tab = editorStore.tabs.find((t) => t.path === oldPath);
    if (tab) {
      tab.path = newPath;
      tab.name = newPath.split("/").pop() ?? newPath;
    }
    await fileTreeStore.refresh();
  }

  return {
    openFile,
    openFileDialog,
    openFolderDialog,
    saveActiveFile,
    saveAsActiveFile,
    newFile,
    createFileOnDisk,
    deletePath,
    renamePath,
  };
}
