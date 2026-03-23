import { defineStore } from "pinia";
import { ref } from "vue";
import type { Template } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useTemplateStore = defineStore("template", () => {
  const templates = ref<Template[]>([]);
  const isLoading = ref(false);

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

  return { templates, isLoading, loadTemplates, saveTemplate, deleteTemplate };
});
