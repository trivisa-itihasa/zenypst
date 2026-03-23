import { defineStore } from "pinia";
import { ref } from "vue";
import type { FileNode } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useFileTreeStore = defineStore("fileTree", () => {
  const rootPath = ref<string | null>(null);
  const nodes = ref<FileNode[]>([]);
  const expandedPaths = ref<Set<string>>(new Set());
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  /** Load a directory as the root of the file tree. */
  async function loadDirectory(path: string): Promise<void> {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await invoke<FileNode[]>("list_directory", { path });
      rootPath.value = path;
      nodes.value = result;
      expandedPaths.value = new Set([path]);
    } catch (err) {
      error.value = String(err);
    } finally {
      isLoading.value = false;
    }
  }

  /** Refresh the file tree from disk. */
  async function refresh(): Promise<void> {
    if (rootPath.value) {
      await loadDirectory(rootPath.value);
    }
  }

  /** Toggle expanded state for a folder path. */
  function toggleExpanded(path: string): void {
    if (expandedPaths.value.has(path)) {
      expandedPaths.value.delete(path);
    } else {
      expandedPaths.value.add(path);
    }
  }

  function isExpanded(path: string): boolean {
    return expandedPaths.value.has(path);
  }

  function clearTree(): void {
    rootPath.value = null;
    nodes.value = [];
    expandedPaths.value.clear();
  }

  return {
    rootPath,
    nodes,
    expandedPaths,
    isLoading,
    error,
    loadDirectory,
    refresh,
    toggleExpanded,
    isExpanded,
    clearTree,
  };
});
