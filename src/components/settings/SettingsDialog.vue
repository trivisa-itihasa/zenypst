<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import FontSettings from "./FontSettings.vue";
import ThemeEditor from "./ThemeEditor.vue";
import PreviewSettings from "./PreviewSettings.vue";
import TypstSettings from "./TypstSettings.vue";

defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{ (e: "update:modelValue", v: boolean): void }>();

const { t } = useI18n();
const tab = ref("editor");

interface NavItem {
  value: string;
  icon: string;
  label: string;
}

const NAV_ITEMS: NavItem[] = [
  { value: "editor", icon: "mdi-format-font", label: t("settings.editor") },
  { value: "theme", icon: "mdi-palette", label: t("settings.theme") },
  { value: "preview", icon: "mdi-eye", label: t("settings.preview") },
  { value: "typst", icon: "mdi-typewriter", label: t("settings.typst") },
];
</script>

<template>
  <q-dialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <q-card class="zen-card" style="width: 720px; max-width: 95vw;">
      <q-card-section class="row items-center q-pa-md">
        <div class="text-subtitle-2">{{ t('settings.title') }}</div>
        <q-space />
        <q-btn flat dense round icon="mdi-close" @click="emit('update:modelValue', false)" />
      </q-card-section>

      <q-separator />

      <q-card-section style="min-height: 500px;">
        <div class="row no-wrap">
          <div class="col-auto" style="width: 180px;">
            <q-list dense>
              <q-item
                v-for="item in NAV_ITEMS"
                :key="item.value"
                clickable
                :active="tab === item.value"
                active-class="zen-nav-active"
                class="rounded"
                @click="tab = item.value"
              >
                <q-item-section avatar><q-icon :name="item.icon" size="18px" /></q-item-section>
                <q-item-section>{{ item.label }}</q-item-section>
              </q-item>
            </q-list>
          </div>

          <div class="col q-pl-md">
            <div v-show="tab === 'editor'">
              <FontSettings />
            </div>
            <div v-show="tab === 'theme'">
              <ThemeEditor />
            </div>
            <div v-show="tab === 'preview'">
              <PreviewSettings />
            </div>
            <div v-show="tab === 'typst'">
              <TypstSettings />
            </div>
          </div>
        </div>
      </q-card-section>
    </q-card>
  </q-dialog>
</template>

<style scoped>
.zen-nav-active {
  background: rgba(var(--zen-primary-rgb), 0.12);
  color: var(--zen-primary);
}
</style>
