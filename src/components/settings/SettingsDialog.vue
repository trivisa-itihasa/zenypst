<script setup lang="ts">
import { ref } from "vue";
import { useSettingsStore } from "@/stores/settings";
import FontSettings from "./FontSettings.vue";
import ThemeEditor from "./ThemeEditor.vue";
import PreviewSettings from "./PreviewSettings.vue";
import TypstSettings from "./TypstSettings.vue";

defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{ (e: "update:modelValue", v: boolean): void }>();

const settingsStore = useSettingsStore();
const tab = ref("editor");

async function toggleColorScheme(): Promise<void> {
  const next = settingsStore.settings.colorScheme === "dark" ? "light" : "dark";
  await settingsStore.update("colorScheme", next);
}
</script>

<template>
  <v-dialog
    :model-value="modelValue"
    max-width="720"
    scrollable
    @update:model-value="emit('update:modelValue', $event)"
  >
    <v-card>
      <v-card-title class="d-flex align-center">
        Settings
        <v-spacer />
        <v-btn
          icon
          variant="text"
          @click="emit('update:modelValue', false)"
        >
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-card-title>

      <v-divider />

      <v-card-text style="min-height: 500px;">
        <v-row>
          <v-col cols="3">
            <v-list density="compact" nav>
              <v-list-item
                prepend-icon="mdi-format-font"
                title="Editor"
                value="editor"
                :active="tab === 'editor'"
                active-color="primary"
                @click="tab = 'editor'"
              />
              <v-list-item
                prepend-icon="mdi-palette"
                title="Theme"
                value="theme"
                :active="tab === 'theme'"
                active-color="primary"
                @click="tab = 'theme'"
              />
              <v-list-item
                prepend-icon="mdi-eye"
                title="Preview"
                value="preview"
                :active="tab === 'preview'"
                active-color="primary"
                @click="tab = 'preview'"
              />
              <v-list-item
                prepend-icon="mdi-monitor"
                title="Appearance"
                value="appearance"
                :active="tab === 'appearance'"
                active-color="primary"
                @click="tab = 'appearance'"
              />
              <v-list-item
                prepend-icon="mdi-console"
                title="Typst CLI"
                value="typst"
                :active="tab === 'typst'"
                active-color="primary"
                @click="tab = 'typst'"
              />
            </v-list>
          </v-col>

          <v-col cols="9" class="pl-4">
            <div v-show="tab === 'editor'">
              <FontSettings />
            </div>
            <div v-show="tab === 'theme'">
              <ThemeEditor />
            </div>
            <div v-show="tab === 'preview'">
              <PreviewSettings />
            </div>
            <div v-show="tab === 'appearance'">
              <p class="text-subtitle-2 mb-4">Appearance</p>
              <v-switch
                :model-value="settingsStore.settings.colorScheme === 'dark'"
                label="Dark Mode"
                density="compact"
                hide-details
                class="mb-4"
                @update:model-value="toggleColorScheme"
              />
              <v-slider
                :model-value="settingsStore.settings.uiFontSize"
                label="UI Font Size"
                :min="11"
                :max="18"
                :step="1"
                density="compact"
                thumb-label
                hide-details
                @update:model-value="settingsStore.update('uiFontSize', $event)"
              >
                <template #append>
                  <span class="text-caption" style="width: 36px;">{{ settingsStore.settings.uiFontSize }}px</span>
                </template>
              </v-slider>
            </div>
            <div v-show="tab === 'typst'">
              <TypstSettings />
            </div>
          </v-col>
        </v-row>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>
