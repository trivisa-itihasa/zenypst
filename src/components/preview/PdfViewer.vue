<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, computed } from "vue";
import { usePreviewStore } from "@/stores/preview";
import CompileStatus from "./CompileStatus.vue";

const previewStore = usePreviewStore();
const pagesContainer = ref<HTMLElement | null>(null);
const scale = ref(1.0);

// Ctrl+wheel zoom
function onWheel(e: WheelEvent): void {
  if (!e.ctrlKey) return;
  e.preventDefault();
  const delta = e.deltaY > 0 ? -0.1 : 0.1;
  scale.value = Math.min(4, Math.max(0.25, Math.round((scale.value + delta) * 10) / 10));
}

onMounted(() => {
  pagesContainer.value?.addEventListener("wheel", onWheel, { passive: false });
});

onUnmounted(() => {
  pagesContainer.value?.removeEventListener("wheel", onWheel);
});

const pageUrls = computed(() =>
  previewStore.pages.map((b64) => `data:image/png;base64,${b64}`)
);
</script>

<template>
  <div class="pdf-viewer">
    <!-- Toolbar -->
    <div class="pdf-toolbar d-flex align-center px-2 py-1" style="min-height: 36px;">
      <v-btn icon size="x-small" variant="text" @click="scale = Math.max(0.25, scale - 0.25)">
        <v-icon>mdi-magnify-minus-outline</v-icon>
      </v-btn>
      <span class="text-caption mx-2">{{ Math.round(scale * 100) }}%</span>
      <v-btn icon size="x-small" variant="text" @click="scale = Math.min(4, scale + 0.25)">
        <v-icon>mdi-magnify-plus-outline</v-icon>
      </v-btn>
      <v-spacer />
      <v-progress-circular
        v-if="previewStore.status === 'compiling'"
        indeterminate
        size="16"
        width="2"
        color="primary"
      />
    </div>

    <!-- Pages -->
    <div ref="pagesContainer" class="pdf-pages pa-2">
      <img
        v-for="(url, i) in pageUrls"
        :key="i"
        :src="url"
        :style="{ width: `${scale * 100}%`, maxWidth: 'none' }"
        class="page-img"
        draggable="false"
      />
    </div>

    <!-- Empty state -->
    <div
      v-if="previewStore.pages.length === 0 && previewStore.status !== 'compiling'"
      class="pdf-empty d-flex flex-column align-center justify-center fill-height"
      style="position: absolute; inset: 36px 0 0 0; pointer-events: none;"
    >
      <v-icon size="64" color="medium-emphasis">mdi-file-pdf-box</v-icon>
      <p class="text-medium-emphasis mt-4 text-caption">PDF preview will appear here</p>
    </div>

    <!-- Compile Status Overlay -->
    <CompileStatus />
  </div>
</template>

<style scoped>
.pdf-viewer {
  position: absolute;
  inset: 0;
  background: rgb(var(--v-theme-surface-variant));
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.pdf-toolbar {
  background: rgb(var(--v-theme-surface));
  border-bottom: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  flex-shrink: 0;
}

.pdf-pages {
  flex: 1 1 0;
  overflow-y: auto;
  background: rgb(var(--v-theme-surface-variant));
  scrollbar-width: thin;
  scrollbar-color: rgba(var(--v-border-color), 0.4) transparent;
}

.pdf-pages::-webkit-scrollbar {
  width: 6px;
}
.pdf-pages::-webkit-scrollbar-thumb {
  background: rgba(var(--v-border-color), 0.4);
  border-radius: 3px;
}
.pdf-pages::-webkit-scrollbar-track {
  background: transparent;
}

.page-img {
  display: block;
  margin: 8px auto;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}
</style>
