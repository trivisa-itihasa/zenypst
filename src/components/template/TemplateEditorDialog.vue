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
  <q-dialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <q-card class="zen-card" style="width: 700px; max-width: 95vw;">
      <q-card-section>
        <div class="text-subtitle-2">
          {{ template?.builtIn ? "Duplicate Template" : template?.id ? "Edit Template" : "New Template" }}
        </div>
      </q-card-section>
      <q-card-section style="max-height: 70vh; overflow-y: auto;">
        <q-input
          v-model="form.name"
          label="Template Name"
          outlined
          dense
          class="mb-3"
        />
        <q-input
          v-model="form.description"
          label="Description"
          outlined
          dense
          class="mb-3"
        />
        <label class="text-caption mb-1">Content</label>
        <q-input
          v-model="form.content"
          type="textarea"
          outlined
          autogrow
          input-class="template-content-input"
          placeholder="#set page(paper: &quot;a4&quot;)&#10;&#10;= Title&#10;"
        />
      </q-card-section>
      <q-card-actions align="right">
        <q-btn flat label="Cancel" @click="emit('update:modelValue', false)" />
        <q-btn flat color="primary" label="Save Template" @click="save" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<style scoped>
:deep(.template-content-input) {
  font-family: "Fira Code", "Consolas", monospace;
  font-size: 13px;
  min-height: 320px;
}
</style>
