<script setup lang="ts">
import { ref, computed } from "vue";
import { useTheme } from "@/composables/useTheme";
import { useSettingsStore } from "@/stores/settings";
import type { Theme, ThemeColors } from "@/types";
import { DEFAULT_DARK_COLORS } from "@/types";
import { v4 as uuidv4 } from "uuid";

const { themes, activeTheme, loadThemes, setTheme, saveTheme, deleteTheme } = useTheme();
const settingsStore = useSettingsStore();

const editDialog = ref(false);
const deleteConfirmId = ref<string | null>(null);
const editingTheme = ref<Theme>({
  id: "",
  name: "",
  builtIn: false,
  colors: { ...DEFAULT_DARK_COLORS },
});

const COLOR_FIELDS: { key: keyof ThemeColors; label: string }[] = [
  { key: "background", label: "Background" },
  { key: "foreground", label: "Text" },
  { key: "caret", label: "Cursor" },
  { key: "selection", label: "Selection" },
  { key: "lineHighlight", label: "Line Highlight" },
  { key: "gutterBackground", label: "Gutter Background" },
  { key: "gutterForeground", label: "Gutter Text" },
  { key: "heading", label: "Heading" },
  { key: "emphasis", label: "Italic" },
  { key: "strong", label: "Bold" },
  { key: "keyword", label: "Keyword" },
  { key: "function", label: "Function" },
  { key: "string", label: "String" },
  { key: "number", label: "Number" },
  { key: "comment", label: "Comment" },
  { key: "math", label: "Math" },
  { key: "label", label: "Label / Reference" },
  { key: "rawBlock", label: "Raw / Code" },
  { key: "operator", label: "Operator" },
  { key: "bracket", label: "Bracket" },
];

function openNewTheme(): void {
  editingTheme.value = {
    id: "",
    name: "New Theme",
    builtIn: false,
    colors: { ...DEFAULT_DARK_COLORS },
  };
  editDialog.value = true;
}

function openDuplicate(theme: Theme): void {
  editingTheme.value = {
    id: "",
    name: `${theme.name} (copy)`,
    builtIn: false,
    colors: { ...theme.colors },
  };
  editDialog.value = true;
}

function openEdit(theme: Theme): void {
  editingTheme.value = {
    id: theme.id,
    name: theme.name,
    builtIn: false,
    colors: { ...theme.colors },
  };
  editDialog.value = true;
}

async function saveEditingTheme(): Promise<void> {
  if (!editingTheme.value.id) {
    editingTheme.value.id = uuidv4();
  }
  await saveTheme(editingTheme.value);
  editDialog.value = false;
}

async function confirmDelete(id: string): Promise<void> {
  await deleteTheme(id);
  deleteConfirmId.value = null;
}
</script>

<template>
  <div class="theme-editor">
    <div class="d-flex align-center mb-4">
      <p class="text-subtitle-2 flex-grow-1">Syntax Theme</p>
      <v-btn size="small" variant="outlined" prepend-icon="mdi-plus" @click="openNewTheme">
        New Theme
      </v-btn>
    </div>

    <v-list density="compact" class="mb-4">
      <v-list-item
        v-for="theme in themes"
        :key="theme.id"
        :active="theme.id === settingsStore.settings.activeThemeId"
        active-color="primary"
        class="rounded"
        @click="setTheme(theme.id)"
      >
        <template #prepend>
          <v-icon :icon="theme.id === settingsStore.settings.activeThemeId ? 'mdi-radiobox-marked' : 'mdi-radiobox-blank'" />
        </template>
        <v-list-item-title>{{ theme.name }}</v-list-item-title>
        <v-list-item-subtitle v-if="theme.builtIn">Built-in</v-list-item-subtitle>
        <template #append>
          <v-btn icon size="x-small" variant="text" @click.stop="openDuplicate(theme)">
            <v-icon>mdi-content-copy</v-icon>
          </v-btn>
          <v-btn
            v-if="!theme.builtIn"
            icon size="x-small" variant="text"
            @click.stop="openEdit(theme)"
          >
            <v-icon>mdi-pencil</v-icon>
          </v-btn>
          <v-btn
            v-if="!theme.builtIn"
            icon size="x-small" variant="text" color="error"
            @click.stop="deleteConfirmId = theme.id"
          >
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </template>
      </v-list-item>
    </v-list>

    <!-- Theme editor dialog -->
    <v-dialog v-model="editDialog" max-width="560" scrollable>
      <v-card>
        <v-card-title>
          {{ editingTheme.id ? "Edit Theme" : "New Theme" }}
        </v-card-title>
        <v-card-text>
          <v-text-field
            v-model="editingTheme.name"
            label="Theme Name"
            density="compact"
            variant="outlined"
            class="mb-4"
          />
          <v-row dense>
            <v-col
              v-for="field in COLOR_FIELDS"
              :key="field.key"
              cols="6"
            >
              <div class="d-flex align-center mb-2">
                <v-menu>
                  <template #activator="{ props }">
                    <div
                      class="color-swatch mr-2"
                      :style="{ backgroundColor: editingTheme.colors[field.key] }"
                      v-bind="props"
                    />
                  </template>
                  <v-color-picker
                    :model-value="editingTheme.colors[field.key]"
                    mode="hex"
                    hide-inputs
                    @update:model-value="(v: string) => editingTheme.colors[field.key] = v"
                  />
                </v-menu>
                <div class="flex-grow-1">
                  <v-text-field
                    v-model="editingTheme.colors[field.key]"
                    :label="field.label"
                    density="compact"
                    variant="plain"
                    hide-details
                    class="color-input"
                  />
                </div>
              </div>
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="editDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="saveEditingTheme">Save</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete confirm dialog -->
    <v-dialog :model-value="!!deleteConfirmId" max-width="360" @update:model-value="deleteConfirmId = null">
      <v-card>
        <v-card-title>Delete Theme?</v-card-title>
        <v-card-text>This action cannot be undone.</v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="deleteConfirmId = null">Cancel</v-btn>
          <v-btn color="error" @click="() => deleteConfirmId && confirmDelete(deleteConfirmId)">
            Delete
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped>
.color-swatch {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid rgba(var(--v-border-color), 0.3);
  cursor: pointer;
  flex-shrink: 0;
}

.color-input :deep(.v-field__input) {
  font-family: monospace;
  font-size: 12px;
  padding: 0;
  min-height: unset;
}
</style>
