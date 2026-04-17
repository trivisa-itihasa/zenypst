<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useTemplateStore } from "@/stores/template";
import type { Template } from "@/types";
import TemplateEditorDialog from "./TemplateEditorDialog.vue";

const { t } = useI18n();

const templateStore = useTemplateStore();

const editorDialog = ref(false);
const editingTemplate = ref<Template | null>(null);
const deleteConfirmId = ref<string | null>(null);

onMounted(async () => {
  await templateStore.loadTemplates();
});

function newTemplate(): void {
  editingTemplate.value = null;
  editorDialog.value = true;
}

function editTemplate(template: Template): void {
  editingTemplate.value = template;
  editorDialog.value = true;
}

async function confirmDelete(id: string): Promise<void> {
  await templateStore.deleteTemplate(id);
  deleteConfirmId.value = null;
}
</script>

<template>
  <div class="template-manager">
    <div class="d-flex align-center mb-4">
      <p class="text-subtitle-2 flex-grow-1">{{ t('templateManager.title') }}</p>
      <q-btn dense outline no-caps icon="mdi-plus" :label="t('templateManager.newTemplate')" @click="newTemplate" />
    </div>

    <q-linear-progress v-if="templateStore.isLoading" indeterminate color="primary" class="mb-2" />

    <q-list dense>
      <q-item
        v-for="template in templateStore.templates"
        :key="template.id"
      >
        <q-item-section avatar>
          <q-icon :name="template.builtIn ? 'mdi-file-document-outline' : 'mdi-file-document-edit-outline'" />
        </q-item-section>
        <q-item-section>
          <q-item-label>{{ template.name }}</q-item-label>
          <q-item-label caption>{{ template.description }}</q-item-label>
        </q-item-section>
        <q-item-section side>
          <div class="row no-wrap">
            <q-btn
              dense flat round size="sm"
              :icon="template.builtIn ? 'mdi-content-copy' : 'mdi-pencil'"
              :title="template.builtIn ? t('templateManager.duplicate') : t('templateManager.edit')"
              @click="editTemplate(template)"
            />
            <q-btn
              v-if="!template.builtIn"
              dense flat round size="sm"
              color="negative"
              icon="mdi-delete"
              @click="deleteConfirmId = template.id"
            />
          </div>
        </q-item-section>
      </q-item>
    </q-list>

    <TemplateEditorDialog
      v-model="editorDialog"
      :template="editingTemplate"
      @saved="templateStore.loadTemplates"
    />

    <q-dialog
      :model-value="!!deleteConfirmId"
      @update:model-value="deleteConfirmId = null"
    >
      <q-card class="zen-card" style="width: 360px; max-width: 90vw;">
        <q-card-section><div class="text-subtitle-2">{{ t('templateManager.deleteTemplate') }}</div></q-card-section>
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
