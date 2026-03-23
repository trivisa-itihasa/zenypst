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
  <v-dialog
    :model-value="modelValue"
    max-width="560"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <v-card>
      <v-card-title>New File — Choose Template</v-card-title>
      <v-card-text>
        <v-text-field
          v-model="search"
          placeholder="Search templates…"
          density="compact"
          variant="outlined"
          prepend-inner-icon="mdi-magnify"
          hide-details
          class="mb-3"
          clearable
        />

        <v-progress-linear v-if="templateStore.isLoading" indeterminate color="primary" class="mb-2" />

        <v-list density="compact">
          <v-list-item
            v-for="template in filteredTemplates"
            :key="template.id"
            :subtitle="template.description"
            @click="select(template)"
          >
            <template #prepend>
              <v-icon>{{ template.builtIn ? 'mdi-file-document-outline' : 'mdi-file-document-edit-outline' }}</v-icon>
            </template>
            <v-list-item-title>{{ template.name }}</v-list-item-title>
          </v-list-item>

          <v-list-item
            v-if="filteredTemplates.length === 0 && !templateStore.isLoading"
            class="text-medium-emphasis"
          >
            No templates found
          </v-list-item>
        </v-list>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn @click="emit('update:modelValue', false)">Cancel</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
