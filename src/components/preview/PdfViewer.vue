<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { usePreviewStore } from "@/stores/preview";
import { convertFileSrc } from "@tauri-apps/api/core";
import CompileStatus from "./CompileStatus.vue";

const previewStore = usePreviewStore();
const pdfContainer = ref<HTMLElement | null>(null);
const isLoading = ref(false);
const scale = ref(1.5);

let pdfjsLib: typeof import("pdfjs-dist") | null = null;

async function loadPdfjs(): Promise<void> {
  if (pdfjsLib) return;
  const pdfjs = await import("pdfjs-dist");
  // Configure worker
  const workerSrc = new URL(
    "pdfjs-dist/build/pdf.worker.mjs",
    import.meta.url
  ).toString();
  pdfjs.GlobalWorkerOptions.workerSrc = workerSrc;
  pdfjsLib = pdfjs;
}

async function renderPdf(filePath: string): Promise<void> {
  if (!pdfContainer.value || !filePath) return;

  isLoading.value = true;
  try {
    await loadPdfjs();
    if (!pdfjsLib) return;

    // Convert the file path to a tauri asset URL
    const url = convertFileSrc(filePath) + `?rev=${previewStore.pdfRevision}`;

    const loadingTask = pdfjsLib.getDocument(url);
    const pdf = await loadingTask.promise;

    // Clear previous pages
    pdfContainer.value.innerHTML = "";

    for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
      const page = await pdf.getPage(pageNum);
      const viewport = page.getViewport({ scale: scale.value });

      const canvas = document.createElement("canvas");
      canvas.width = viewport.width;
      canvas.height = viewport.height;
      canvas.style.display = "block";
      canvas.style.margin = "8px auto";
      canvas.style.boxShadow = "0 2px 8px rgba(0,0,0,0.3)";

      const ctx = canvas.getContext("2d");
      if (!ctx) continue;

      await page.render({ canvasContext: ctx, viewport }).promise;
      pdfContainer.value.appendChild(canvas);
    }
  } catch (err) {
    console.error("Failed to render PDF:", err);
  } finally {
    isLoading.value = false;
  }
}

// Re-render when a new PDF is available
watch(
  () => previewStore.pdfRevision,
  async () => {
    const path = previewStore.lastSuccessfulPdfPath;
    if (path) {
      await renderPdf(path);
    }
  }
);

onMounted(async () => {
  const path = previewStore.lastSuccessfulPdfPath;
  if (path) {
    await renderPdf(path);
  }
});
</script>

<template>
  <div class="pdf-viewer d-flex flex-column fill-height" style="position: relative;">
    <!-- Toolbar -->
    <div class="pdf-toolbar d-flex align-center px-2 py-1" style="min-height: 36px;">
      <v-btn icon size="x-small" variant="text" @click="scale = Math.max(0.5, scale - 0.25)">
        <v-icon>mdi-magnify-minus-outline</v-icon>
      </v-btn>
      <span class="text-caption mx-2">{{ Math.round(scale * 100) }}%</span>
      <v-btn icon size="x-small" variant="text" @click="scale = Math.min(4, scale + 0.25)">
        <v-icon>mdi-magnify-plus-outline</v-icon>
      </v-btn>
      <v-spacer />
      <v-progress-circular
        v-if="isLoading"
        indeterminate
        size="16"
        width="2"
        color="primary"
      />
    </div>

    <!-- PDF Pages -->
    <div
      ref="pdfContainer"
      class="pdf-pages flex-grow-1 overflow-y-auto pa-2"
    />

    <!-- Empty state -->
    <div
      v-if="!previewStore.lastSuccessfulPdfPath && previewStore.status !== 'compiling'"
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
  background: rgb(var(--v-theme-surface-variant));
  overflow: hidden;
}

.pdf-toolbar {
  background: rgb(var(--v-theme-surface));
  border-bottom: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  flex-shrink: 0;
}

.pdf-pages {
  background: rgb(var(--v-theme-surface-variant));
}
</style>
