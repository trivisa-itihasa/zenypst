import { defineStore } from "pinia";
import { ref } from "vue";
import type { Template, UniverseTemplate } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useTemplateStore = defineStore("template", () => {
  const templates = ref<Template[]>([]);
  const isLoading = ref(false);

  const universeTemplates = ref<UniverseTemplate[]>([]);
  const isLoadingUniverse = ref(false);

  /** Load all templates from disk. */
  async function loadTemplates(): Promise<void> {
    isLoading.value = true;
    try {
      const result = await invoke<Template[]>("list_templates");
      templates.value = result;
    } catch (err) {
      console.error("Failed to load templates:", err);
    } finally {
      isLoading.value = false;
    }
  }

  /** Save a template (create or update). */
  async function saveTemplate(template: Template): Promise<void> {
    await invoke("save_template", { template });
    await loadTemplates();
  }

  /** Delete a custom template by ID. */
  async function deleteTemplate(id: string): Promise<void> {
    await invoke("delete_template", { id });
    templates.value = templates.value.filter((t) => t.id !== id);
  }

  /** Fetch templates from Typst Universe. */
  async function fetchUniverseTemplates(): Promise<void> {
    isLoadingUniverse.value = true;
    try {
      const result = await invoke<UniverseTemplate[]>("fetch_universe_templates");
      universeTemplates.value = result;
    } catch (err) {
      console.error("Failed to fetch universe templates:", err);
    } finally {
      isLoadingUniverse.value = false;
    }
  }

  /** Initialize a project from a Typst Universe template. */
  async function initUniverseTemplate(
    templateName: string,
    version: string,
    targetDir: string
  ): Promise<void> {
    await invoke("init_universe_template", {
      templateName,
      version,
      targetDir,
    });
  }

  return {
    templates,
    isLoading,
    loadTemplates,
    saveTemplate,
    deleteTemplate,
    universeTemplates,
    isLoadingUniverse,
    fetchUniverseTemplates,
    initUniverseTemplate,
  };
});
