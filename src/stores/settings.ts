import { defineStore } from "pinia";
import { ref } from "vue";
import type { AppSettings } from "@/types";
import { DEFAULT_SETTINGS } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS });
  const isLoaded = ref(false);

  /** Load settings from disk via Tauri command. */
  async function load(): Promise<void> {
    try {
      const loaded = await invoke<AppSettings>("load_settings");
      settings.value = { ...DEFAULT_SETTINGS, ...loaded };
    } catch (err) {
      console.error("Failed to load settings:", err);
      settings.value = { ...DEFAULT_SETTINGS };
    } finally {
      isLoaded.value = true;
    }
  }

  /** Persist the current settings to disk. */
  async function save(): Promise<void> {
    try {
      await invoke("save_settings", { settings: settings.value });
    } catch (err) {
      console.error("Failed to save settings:", err);
      throw err;
    }
  }

  /** Update a setting field and persist. */
  async function update<K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ): Promise<void> {
    settings.value[key] = value;
    await save();
  }

  return { settings, isLoaded, load, save, update };
});
