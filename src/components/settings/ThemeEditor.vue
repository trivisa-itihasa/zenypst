<script setup lang="ts">
import { ref } from "vue";
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
      <q-btn dense outline no-caps icon="mdi-plus" label="New Theme" @click="openNewTheme" />
    </div>

    <q-list dense class="mb-4">
      <q-item
        v-for="theme in themes"
        :key="theme.id"
        clickable
        :active="theme.id === settingsStore.settings.activeThemeId"
        active-class="zen-theme-active"
        class="rounded"
        @click="setTheme(theme.id)"
      >
        <q-item-section avatar>
          <q-icon
            :name="theme.id === settingsStore.settings.activeThemeId ? 'mdi-radiobox-marked' : 'mdi-radiobox-blank'"
          />
        </q-item-section>
        <q-item-section>
          <q-item-label>{{ theme.name }}</q-item-label>
          <q-item-label v-if="theme.builtIn" caption>Built-in</q-item-label>
        </q-item-section>
        <q-item-section side>
          <div class="row no-wrap">
            <q-btn dense flat round size="sm" icon="mdi-content-copy" @click.stop="openDuplicate(theme)" />
            <q-btn
              v-if="!theme.builtIn"
              dense flat round size="sm"
              icon="mdi-pencil"
              @click.stop="openEdit(theme)"
            />
            <q-btn
              v-if="!theme.builtIn"
              dense flat round size="sm"
              color="negative"
              icon="mdi-delete"
              @click.stop="deleteConfirmId = theme.id"
            />
          </div>
        </q-item-section>
      </q-item>
    </q-list>

    <!-- Theme editor dialog -->
    <q-dialog v-model="editDialog">
      <q-card class="zen-card" style="width: 560px; max-width: 95vw;">
        <q-card-section>
          <div class="text-subtitle-2">{{ editingTheme.id ? "Edit Theme" : "New Theme" }}</div>
        </q-card-section>
        <q-card-section style="max-height: 70vh; overflow-y: auto;">
          <q-input
            v-model="editingTheme.name"
            label="Theme Name"
            outlined
            dense
            class="mb-4"
          />
          <div class="row q-col-gutter-x-sm q-col-gutter-y-xs">
            <div
              v-for="field in COLOR_FIELDS"
              :key="field.key"
              class="col-6"
            >
              <div class="d-flex align-center mb-2">
                <div
                  class="color-swatch mr-2"
                  :style="{ backgroundColor: editingTheme.colors[field.key] }"
                >
                  <q-menu>
                    <q-color
                      :model-value="editingTheme.colors[field.key]"
                      no-header
                      no-footer
                      default-view="palette"
                      format-model="hex"
                      @update:model-value="(v: string | null) => editingTheme.colors[field.key] = v ?? ''"
                    />
                  </q-menu>
                </div>
                <q-input
                  v-model="editingTheme.colors[field.key]"
                  :label="field.label"
                  borderless
                  dense
                  class="color-input flex-grow-1"
                />
              </div>
            </div>
          </div>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" @click="editDialog = false" />
          <q-btn flat color="primary" label="Save" @click="saveEditingTheme" />
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- Delete confirm dialog -->
    <q-dialog
      :model-value="!!deleteConfirmId"
      @update:model-value="deleteConfirmId = null"
    >
      <q-card class="zen-card" style="width: 360px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">Delete Theme?</div></q-card-section>
        <q-card-section>This action cannot be undone.</q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" @click="deleteConfirmId = null" />
          <q-btn
            flat
            color="negative"
            label="Delete"
            @click="() => deleteConfirmId && confirmDelete(deleteConfirmId)"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<style scoped>
.color-swatch {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid var(--zen-border);
  cursor: pointer;
  flex-shrink: 0;
  position: relative;
}

.color-input :deep(.q-field__native) {
  font-family: monospace;
  font-size: 12px;
  padding: 0;
  min-height: unset;
}

.zen-theme-active {
  background: rgba(var(--zen-primary-rgb), 0.12);
  color: var(--zen-primary);
}
</style>
