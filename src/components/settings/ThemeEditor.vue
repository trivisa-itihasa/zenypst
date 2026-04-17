<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useTheme } from "@/composables/useTheme";
import { useSettingsStore } from "@/stores/settings";
import type { Theme, ThemeColors } from "@/types";
import { DEFAULT_DARK_COLORS } from "@/types";
import { v4 as uuidv4 } from "uuid";

const { t } = useI18n();

const { themes, setTheme, saveTheme, deleteTheme } = useTheme();
const settingsStore = useSettingsStore();

const editDialog = ref(false);
const deleteConfirmId = ref<string | null>(null);
const editingTheme = ref<Theme>({
  id: "",
  name: "",
  builtIn: false,
  isDark: true,
  colors: { ...DEFAULT_DARK_COLORS },
});

interface ColorField {
  key: keyof ThemeColors;
  i18nKey: string;
}

interface ColorSection {
  i18nKey: string;
  fields: ColorField[];
}

const COLOR_SECTIONS: ColorSection[] = [
  {
    i18nKey: "themeEditor.sectionUi",
    fields: [
      { key: "appBackground", i18nKey: "themeEditor.appBackground" },
      { key: "surface", i18nKey: "themeEditor.surface" },
      { key: "surfaceVariant", i18nKey: "themeEditor.surfaceVariant" },
      { key: "border", i18nKey: "themeEditor.border" },
      { key: "uiText", i18nKey: "themeEditor.uiText" },
      { key: "uiTextMuted", i18nKey: "themeEditor.uiTextMuted" },
      { key: "primary", i18nKey: "themeEditor.primary" },
      { key: "statusBar", i18nKey: "themeEditor.statusBar" },
      { key: "statusBarText", i18nKey: "themeEditor.statusBarText" },
      { key: "error", i18nKey: "themeEditor.error" },
      { key: "warning", i18nKey: "themeEditor.warning" },
      { key: "info", i18nKey: "themeEditor.info" },
      { key: "success", i18nKey: "themeEditor.success" },
    ],
  },
  {
    i18nKey: "themeEditor.sectionEditor",
    fields: [
      { key: "background", i18nKey: "themeEditor.background" },
      { key: "foreground", i18nKey: "themeEditor.text" },
      { key: "caret", i18nKey: "themeEditor.cursor" },
      { key: "selection", i18nKey: "themeEditor.selection" },
      { key: "lineHighlight", i18nKey: "themeEditor.lineHighlight" },
      { key: "gutterBackground", i18nKey: "themeEditor.gutterBackground" },
      { key: "gutterForeground", i18nKey: "themeEditor.gutterText" },
    ],
  },
  {
    i18nKey: "themeEditor.sectionSyntax",
    fields: [
      { key: "heading", i18nKey: "themeEditor.heading" },
      { key: "emphasis", i18nKey: "themeEditor.italic" },
      { key: "strong", i18nKey: "themeEditor.bold" },
      { key: "keyword", i18nKey: "themeEditor.keyword" },
      { key: "function", i18nKey: "themeEditor.function" },
      { key: "string", i18nKey: "themeEditor.string" },
      { key: "number", i18nKey: "themeEditor.number" },
      { key: "comment", i18nKey: "themeEditor.comment" },
      { key: "math", i18nKey: "themeEditor.math" },
      { key: "label", i18nKey: "themeEditor.labelReference" },
      { key: "rawBlock", i18nKey: "themeEditor.rawCode" },
      { key: "operator", i18nKey: "themeEditor.operator" },
      { key: "bracket", i18nKey: "themeEditor.bracket" },
    ],
  },
];

function openNewTheme(): void {
  editingTheme.value = {
    id: "",
    name: "New Theme",
    builtIn: false,
    isDark: true,
    colors: { ...DEFAULT_DARK_COLORS },
  };
  editDialog.value = true;
}

function openDuplicate(theme: Theme): void {
  editingTheme.value = {
    id: "",
    name: `${theme.name} (copy)`,
    builtIn: false,
    isDark: theme.isDark,
    colors: { ...theme.colors },
  };
  editDialog.value = true;
}

function openEdit(theme: Theme): void {
  editingTheme.value = {
    id: theme.id,
    name: theme.name,
    builtIn: false,
    isDark: theme.isDark,
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
      <p class="text-subtitle-2 flex-grow-1">{{ t('themeEditor.theme') }}</p>
      <q-btn dense outline no-caps icon="mdi-plus" :label="t('themeEditor.newTheme')" @click="openNewTheme" />
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
          <q-item-label v-if="theme.builtIn" caption>{{ t('themeEditor.builtIn') }}</q-item-label>
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
          <div class="text-subtitle-2">{{ editingTheme.id ? t('themeEditor.editTheme') : t('themeEditor.newTheme') }}</div>
        </q-card-section>
        <q-card-section style="max-height: 70vh; overflow-y: auto;">
          <q-input
            v-model="editingTheme.name"
            :label="t('themeEditor.themeName')"
            outlined
            dense
            class="mb-4"
          />
          <q-toggle
            v-model="editingTheme.isDark"
            :label="t('themeEditor.isDark')"
            dense
            class="mb-4"
          />
          <template v-for="section in COLOR_SECTIONS" :key="section.i18nKey">
            <p class="text-caption text-medium-emphasis text-uppercase tracking-widest mt-4 mb-2">
              {{ t(section.i18nKey) }}
            </p>
            <div class="row q-col-gutter-x-sm q-col-gutter-y-xs">
              <div
                v-for="field in section.fields"
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
                    :label="t(field.i18nKey)"
                    borderless
                    dense
                    class="color-input flex-grow-1"
                  />
                </div>
              </div>
            </div>
          </template>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat :label="t('common.cancel')" @click="editDialog = false" />
          <q-btn flat color="primary" :label="t('common.save')" @click="saveEditingTheme" />
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- Delete confirm dialog -->
    <q-dialog
      :model-value="!!deleteConfirmId"
      @update:model-value="deleteConfirmId = null"
    >
      <q-card class="zen-card" style="width: 360px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">{{ t('themeEditor.deleteTheme') }}</div></q-card-section>
        <q-card-section>{{ t('common.thisActionCannotBeUndone') }}</q-card-section>
        <q-card-actions align="right">
          <q-btn flat :label="t('common.cancel')" @click="deleteConfirmId = null" />
          <q-btn
            flat
            color="negative"
            :label="t('common.delete')"
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
