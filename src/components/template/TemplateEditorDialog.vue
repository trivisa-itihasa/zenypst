<script setup lang="ts">
import { ref, watch } from "vue";
import { useTemplateStore } from "@/stores/template";
import type { Template } from "@/types";
import { v4 as uuidv4 } from "uuid";

const props = defineProps<{
  modelValue: boolean;
  template?: Template | null;
}>();
const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "saved"): void;
}>();

const templateStore = useTemplateStore();

const form = ref<Template>({
  id: "",
  name: "",
  description: "",
  content: "",
  builtIn: false,
});

watch(
  () => props.template,
  (t) => {
    if (t) {
      form.value = { ...t, builtIn: false, id: t.builtIn ? "" : t.id };
    } else {
      form.value = { id: "", name: "", description: "", content: "", builtIn: false };
    }
  },
  { immediate: true }
);

async function save(): Promise<void> {
  if (!form.value.id) {
    form.value.id = uuidv4();
  }
  try {
    await templateStore.saveTemplate(form.value);
    emit("saved");
    emit("update:modelValue", false);
  } catch (err) {
    console.error("Failed to save template:", err);
  }
}
</script>

<template>
  <v-dialog
    :model-value="modelValue"
    max-width="700"
    scrollable
    @update:model-value="emit('update:modelValue', $event)"
  >
    <v-card>
      <v-card-title>
        {{ template?.builtIn ? "Duplicate Template" : template?.id ? "Edit Template" : "New Template" }}
      </v-card-title>
      <v-card-text>
        <v-text-field
          v-model="form.name"
          label="Template Name"
          density="compact"
          variant="outlined"
          class="mb-3"
        />
        <v-text-field
          v-model="form.description"
          label="Description"
          density="compact"
          variant="outlined"
          class="mb-3"
        />
        <v-label class="mb-1 text-caption">Content</v-label>
        <v-textarea
          v-model="form.content"
          variant="outlined"
          rows="16"
          auto-grow
          no-resize
          class="template-content-editor"
          placeholder="#set page(paper: &quot;a4&quot;)&#10;&#10;= Title&#10;"
        />
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn @click="emit('update:modelValue', false)">Cancel</v-btn>
        <v-btn color="primary" @click="save">Save Template</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.template-content-editor :deep(textarea) {
  font-family: "Fira Code", "Consolas", monospace;
  font-size: 13px;
}
</style>
