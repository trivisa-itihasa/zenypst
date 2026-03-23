<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "@/stores/settings";

// Browse is only available inside the Tauri desktop app.
const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

const settingsStore = useSettingsStore();

const versionText = ref<string | null>(null);
const checking = ref(false);

async function checkVersion(): Promise<void> {
  checking.value = true;
  versionText.value = null;
  try {
    const typstPath = settingsStore.settings.typstPath || undefined;
    const version = await invoke<string>("get_typst_version", { typstPath });
    versionText.value = version;
  } catch {
    versionText.value = "Not found";
  } finally {
    checking.value = false;
  }
}

async function browsePath(): Promise<void> {
  const { open } = await import("@tauri-apps/plugin-dialog");
  const selected = await open({ multiple: false, directory: false });
  if (selected && typeof selected === "string") {
    await settingsStore.update("typstPath", selected);
    await checkVersion();
  }
}

onMounted(() => { if (isTauri) checkVersion(); });
</script>

<template>
  <p class="text-subtitle-2 mb-4">Typst CLI</p>

  <v-text-field
    :model-value="settingsStore.settings.typstPath"
    label="Typst executable path"
    placeholder="Leave blank to auto-detect"
    density="compact"
    hide-details="auto"
    class="mb-2"
    @update:model-value="settingsStore.update('typstPath', $event)"
  >
    <template v-if="isTauri" #append>
      <v-btn size="small" variant="tonal" @click="browsePath">Browse</v-btn>
    </template>
  </v-text-field>

  <p class="text-caption text-medium-emphasis mb-4">
    Leave blank to auto-detect from PATH and common locations (e.g.
    <code>~/.cargo/bin/typst</code>).
  </p>

  <div class="d-flex align-center gap-3">
    <v-btn
      size="small"
      variant="tonal"
      :loading="checking"
      :disabled="!isTauri"
      prepend-icon="mdi-check-circle-outline"
      @click="checkVersion"
    >
      Check version
    </v-btn>
    <span v-if="versionText" class="text-caption">
      <v-icon
        size="14"
        :color="versionText === 'Not found' ? 'error' : 'success'"
        class="mr-1"
      >
        {{ versionText === "Not found" ? "mdi-close-circle" : "mdi-check-circle" }}
      </v-icon>
      {{ versionText }}
    </span>
  </div>
</template>
