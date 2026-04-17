<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";

const { t } = useI18n();

const version = ref<string | null>(null);

onMounted(async () => {
  try {
    version.value = await invoke<string>("get_typst_version");
  } catch {
    version.value = null;
  }
});
</script>

<template>
  <p class="text-subtitle-2 mb-4">{{ t('typstSettings.title') }}</p>
  <p class="text-body-2 mb-2">
    {{ t('typstSettings.bundledMessage') }}
  </p>
  <div v-if="version" class="d-flex align-center gap-2 mt-4">
    <q-icon name="mdi-check-circle" size="16px" color="positive" />
    <span class="text-caption">{{ version }}</span>
  </div>
</template>
