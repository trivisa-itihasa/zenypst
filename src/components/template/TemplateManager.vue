<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useTemplateStore } from "@/stores/template";
import type { Template } from "@/types";
import TemplateEditorDialog from "./TemplateEditorDialog.vue";

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
      <p class="text-subtitle-2 flex-grow-1">Template Manager</p>
      <v-btn size="small" variant="outlined" prepend-icon="mdi-plus" @click="newTemplate">
        New Template
      </v-btn>
    </div>

    <v-progress-linear v-if="templateStore.isLoading" indeterminate color="primary" class="mb-2" />

    <v-list density="compact">
      <v-list-item
        v-for="template in templateStore.templates"
        :key="template.id"
        :subtitle="template.description"
      >
        <template #prepend>
          <v-icon>{{ template.builtIn ? 'mdi-file-document-outline' : 'mdi-file-document-edit-outline' }}</v-icon>
        </template>
        <v-list-item-title>{{ template.name }}</v-list-item-title>
        <template #append>
          <v-btn icon size="x-small" variant="text" title="Duplicate / Edit" @click="editTemplate(template)">
            <v-icon>{{ template.builtIn ? 'mdi-content-copy' : 'mdi-pencil' }}</v-icon>
          </v-btn>
          <v-btn
            v-if="!template.builtIn"
            icon size="x-small" variant="text" color="error"
            @click="deleteConfirmId = template.id"
          >
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </template>
      </v-list-item>
    </v-list>

    <TemplateEditorDialog
      v-model="editorDialog"
      :template="editingTemplate"
      @saved="templateStore.loadTemplates"
    />

    <v-dialog :model-value="!!deleteConfirmId" max-width="360" @update:model-value="deleteConfirmId = null">
      <v-card>
        <v-card-title>Delete Template?</v-card-title>
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
