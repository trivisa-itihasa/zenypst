<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useTemplateStore } from "@/stores/template";
import type { Template } from "@/types";

defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "selected", template: Template): void;
}>();

const templateStore = useTemplateStore();
const search = ref("");

const filteredTemplates = computed(() =>
  templateStore.templates.filter(
    (t) =>
      t.name.toLowerCase().includes(search.value.toLowerCase()) ||
      t.description.toLowerCase().includes(search.value.toLowerCase())
  )
);

onMounted(async () => {
  if (templateStore.templates.length === 0) {
    await templateStore.loadTemplates();
  }
});

function select(template: Template): void {
  emit("selected", template);
  emit("update:modelValue", false);
}
</script>

<template>
  <q-dialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <q-card class="zen-card" style="width: 560px; max-width: 95vw;">
      <q-card-section><div class="text-subtitle-2">New File — Choose Template</div></q-card-section>
      <q-card-section>
        <q-input
          v-model="search"
          placeholder="Search templates…"
          outlined
          dense
          clearable
          class="mb-3"
        >
          <template #prepend>
            <q-icon name="mdi-magnify" size="18px" />
          </template>
        </q-input>

        <q-linear-progress
          v-if="templateStore.isLoading"
          indeterminate
          color="primary"
          class="mb-2"
        />

        <q-list dense>
          <q-item
            v-for="template in filteredTemplates"
            :key="template.id"
            clickable
            @click="select(template)"
          >
            <q-item-section avatar>
              <q-icon :name="template.builtIn ? 'mdi-file-document-outline' : 'mdi-file-document-edit-outline'" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ template.name }}</q-item-label>
              <q-item-label caption>{{ template.description }}</q-item-label>
            </q-item-section>
          </q-item>

          <q-item
            v-if="filteredTemplates.length === 0 && !templateStore.isLoading"
            class="text-medium-emphasis"
          >
            <q-item-section>No templates found</q-item-section>
          </q-item>
        </q-list>
      </q-card-section>
      <q-card-actions align="right">
        <q-btn flat label="Cancel" @click="emit('update:modelValue', false)" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>
