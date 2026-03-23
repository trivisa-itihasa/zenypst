import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Theme } from "@/types";
import { DEFAULT_DARK_COLORS } from "@/types";
import { useSettingsStore } from "@/stores/settings";

const themes = ref<Theme[]>([]);
const isLoaded = ref(false);

/**
 * Composable for loading, applying, and saving editor themes.
 */
export function useTheme() {
  const settingsStore = useSettingsStore();

  const activeTheme = computed<Theme>(() => {
    const id = settingsStore.settings.activeThemeId;
    return (
      themes.value.find((t) => t.id === id) ?? {
        id: "zenypst-dark",
        name: "Zenypst Dark",
        builtIn: true,
        colors: DEFAULT_DARK_COLORS,
      }
    );
  });

  /** Load all themes from the backend. */
  async function loadThemes(): Promise<void> {
    try {
      const result = await invoke<Theme[]>("list_themes");
      themes.value = result;
      isLoaded.value = true;
    } catch (err) {
      console.error("Failed to load themes:", err);
    }
  }

  /** Set the active theme by ID. */
  async function setTheme(id: string): Promise<void> {
    await settingsStore.update("activeThemeId", id);
  }

  /** Save a custom theme. */
  async function saveTheme(theme: Theme): Promise<void> {
    await invoke("save_theme", { theme });
    await loadThemes();
  }

  /** Delete a custom theme. */
  async function deleteTheme(id: string): Promise<void> {
    await invoke("delete_theme", { id });
    themes.value = themes.value.filter((t) => t.id !== id);
    // Fall back to default if deleted theme was active
    if (settingsStore.settings.activeThemeId === id) {
      await setTheme("zenypst-dark");
    }
  }

  return {
    themes,
    isLoaded,
    activeTheme,
    loadThemes,
    setTheme,
    saveTheme,
    deleteTheme,
  };
}
