<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useTemplateStore } from "@/stores/template";
import type { Template, UniverseTemplate } from "@/types";

const { t } = useI18n();

withDefaults(defineProps<{ modelValue: boolean; allowUniverse?: boolean }>(), {
  allowUniverse: true,
});
const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "selected", template: Template): void;
  (e: "selected-universe", template: UniverseTemplate): void;
}>();

const templateStore = useTemplateStore();
const search = ref("");
const tab = ref<"local" | "universe">("local");

const filteredTemplates = computed(() =>
  templateStore.templates.filter(
    (t) =>
      t.name.toLowerCase().includes(search.value.toLowerCase()) ||
      t.description.toLowerCase().includes(search.value.toLowerCase())
  )
);

const filteredUniverseTemplates = computed(() =>
  templateStore.universeTemplates.filter(
    (t) =>
      t.name.toLowerCase().includes(search.value.toLowerCase()) ||
      (t.description?.toLowerCase().includes(search.value.toLowerCase()) ?? false)
  )
);

onMounted(async () => {
  if (templateStore.templates.length === 0) {
    await templateStore.loadTemplates();
  }
  if (templateStore.universeTemplates.length === 0) {
    await templateStore.fetchUniverseTemplates();
  }
});

function selectLocal(template: Template): void {
  emit("selected", template);
  emit("update:modelValue", false);
}

function selectUniverse(template: UniverseTemplate): void {
  emit("selected-universe", template);
  emit("update:modelValue", false);
}
</script>

<template>
  <q-dialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <q-card class="zen-card" style="width: 600px; max-width: 95vw;">
      <q-card-section><div class="text-subtitle-2">{{ t('templatePicker.title') }}</div></q-card-section>
      <q-card-section>
        <q-input
          v-model="search"
          :placeholder="t('templatePicker.searchPlaceholder')"
          outlined
          dense
          class="mb-3"
        >
          <template #prepend>
            <q-icon name="mdi-magnify" size="18px" />
          </template>
          <template v-if="search" #append>
            <q-icon
              name="mdi-close-circle-outline"
              size="18px"
              class="cursor-pointer"
              @click="search = ''"
            />
          </template>
        </q-input>

        <q-tabs
          v-if="allowUniverse"
          v-model="tab"
          dense
          class="text-caption mb-2"
          indicator-color="primary"
          align="left"
        >
          <q-tab name="local" :label="t('templatePicker.localTab')" />
          <q-tab name="universe" :label="t('templatePicker.universeTab')" />
        </q-tabs>

        <q-linear-progress
          v-if="templateStore.isLoading || templateStore.isLoadingUniverse"
          indeterminate
          color="primary"
          class="mb-2"
        />

        <q-tab-panels v-model="tab" animated class="bg-transparent" style="min-height: 200px;">
          <q-tab-panel name="local" class="q-pa-none">
            <q-list dense>
              <q-item
                v-for="template in filteredTemplates"
                :key="template.id"
                clickable
                @click="selectLocal(template)"
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
                <q-item-section>{{ t('templatePicker.noTemplatesFound') }}</q-item-section>
              </q-item>
            </q-list>
          </q-tab-panel>

          <q-tab-panel name="universe" class="q-pa-none">
            <q-list dense>
              <q-item
                v-for="template in filteredUniverseTemplates"
                :key="template.name + '-' + template.version"
                clickable
                @click="selectUniverse(template)"
              >
                <q-item-section avatar>
                  <q-icon name="mdi-earth" />
                </q-item-section>
                <q-item-section>
                  <q-item-label>{{ template.name }} <q-badge dense color="secondary" class="text-caption">{{ template.version }}</q-badge></q-item-label>
                  <q-item-label caption>
                    {{ template.description || t('templatePicker.noDescription') }}
                  </q-item-label>
                  <q-item-label caption class="text-italic">
                    {{ template.categories.join(', ') }}
                  </q-item-label>
                </q-item-section>
              </q-item>

              <q-item
                v-if="filteredUniverseTemplates.length === 0 && !templateStore.isLoadingUniverse"
                class="text-medium-emphasis"
              >
                <q-item-section>{{ t('templatePicker.noTemplatesFound') }}</q-item-section>
              </q-item>
            </q-list>
          </q-tab-panel>
        </q-tab-panels>
      </q-card-section>
      <q-card-actions align="right">
        <q-btn flat :label="t('common.cancel')" @click="emit('update:modelValue', false)" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>
