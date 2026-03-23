import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { FileTab } from "@/types";
import { v4 as uuidv4 } from "uuid";

export const useEditorStore = defineStore("editor", () => {
  const tabs = ref<FileTab[]>([]);
  const activeTabId = ref<string | null>(null);

  const activeTab = computed(() =>
    tabs.value.find((t) => t.id === activeTabId.value) ?? null
  );

  const hasDirtyTabs = computed(() =>
    tabs.value.some((t) => t.isDirty)
  );

  /** Open a file in a new tab or switch to it if already open. */
  function openTab(path: string, content: string, name?: string): string {
    const existing = tabs.value.find((t) => t.path === path);
    if (existing) {
      activeTabId.value = existing.id;
      return existing.id;
    }

    const tab: FileTab = {
      id: uuidv4(),
      path,
      name: name ?? path.split("/").pop() ?? path,
      content,
      isDirty: false,
      isUntitled: false,
    };
    tabs.value.push(tab);
    activeTabId.value = tab.id;
    return tab.id;
  }

  /** Create a new untitled tab. */
  function newUntitledTab(content = "", name = "untitled.typ"): string {
    const tab: FileTab = {
      id: uuidv4(),
      path: null,
      name,
      content,
      isDirty: content.length > 0,
      isUntitled: true,
    };
    tabs.value.push(tab);
    activeTabId.value = tab.id;
    return tab.id;
  }

  /** Close a tab by ID. Returns the ID of the newly active tab, or null. */
  function closeTab(id: string): string | null {
    const idx = tabs.value.findIndex((t) => t.id === id);
    if (idx === -1) return activeTabId.value;

    tabs.value.splice(idx, 1);

    if (activeTabId.value === id) {
      if (tabs.value.length === 0) {
        activeTabId.value = null;
      } else {
        const newIdx = Math.min(idx, tabs.value.length - 1);
        activeTabId.value = tabs.value[newIdx].id;
      }
    }

    return activeTabId.value;
  }

  /** Update the content of the active tab. */
  function updateContent(id: string, content: string): void {
    const tab = tabs.value.find((t) => t.id === id);
    if (tab) {
      tab.content = content;
      tab.isDirty = true;
    }
  }

  /** Mark a tab as saved (clears dirty flag). */
  function markSaved(id: string, path?: string): void {
    const tab = tabs.value.find((t) => t.id === id);
    if (tab) {
      tab.isDirty = false;
      if (path) {
        tab.path = path;
        tab.name = path.split("/").pop() ?? path;
        tab.isUntitled = false;
      }
    }
  }

  function setActiveTab(id: string): void {
    if (tabs.value.some((t) => t.id === id)) {
      activeTabId.value = id;
    }
  }

  return {
    tabs,
    activeTabId,
    activeTab,
    hasDirtyTabs,
    openTab,
    newUntitledTab,
    closeTab,
    updateContent,
    markSaved,
    setActiveTab,
  };
});
