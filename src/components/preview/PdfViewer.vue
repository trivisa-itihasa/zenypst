<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { usePreviewStore } from "@/stores/preview";
import { readFile } from "@tauri-apps/plugin-fs";
import type { PDFDocumentProxy } from "pdfjs-dist";
import CompileStatus from "./CompileStatus.vue";

const previewStore = usePreviewStore();
const pdfContainer = ref<HTMLElement | null>(null);
const isLoading = ref(false);
const scale = ref(1.5);

let pdfjsLib: typeof import("pdfjs-dist") | null = null;
let currentBlobUrl: string | null = null;
let cachedPdf: PDFDocumentProxy | null = null;

async function loadPdfjs(): Promise<void> {
  if (pdfjsLib) return;
  const pdfjs = await import("pdfjs-dist");
  const workerSrc = new URL("pdfjs-dist/build/pdf.worker.mjs", import.meta.url).toString();
  pdfjs.GlobalWorkerOptions.workerSrc = workerSrc;
  pdfjsLib = pdfjs;
}

/** Re-draw all pages using cachedPdf and current scale. */
async function renderPages(): Promise<void> {
  if (!pdfContainer.value || !cachedPdf) return;

  // スクロール位置を保持する
  const savedScrollTop = pdfContainer.value.scrollTop;

  isLoading.value = true;
  try {
    const numPages = cachedPdf.numPages;

    // 先に全 canvas を DOM に追加して順序を確定する
    pdfContainer.value.innerHTML = "";
    const canvases: HTMLCanvasElement[] = Array.from({ length: numPages }, () => {
      const canvas = document.createElement("canvas");
      canvas.style.display = "block";
      canvas.style.margin = "8px auto";
      canvas.style.boxShadow = "0 2px 8px rgba(0,0,0,0.3)";
      pdfContainer.value!.appendChild(canvas);
      return canvas;
    });

    // 全ページを並列描画
    await Promise.all(
      canvases.map(async (canvas, i) => {
        const page = await cachedPdf!.getPage(i + 1);
        const viewport = page.getViewport({ scale: scale.value });
        canvas.width = viewport.width;
        canvas.height = viewport.height;
        const ctx = canvas.getContext("2d");
        if (!ctx) return;
        await page.render({ canvasContext: ctx, viewport }).promise;
      })
    );

    // スクロール位置を復元
    pdfContainer.value.scrollTop = savedScrollTop;
  } catch (err) {
    console.error("Failed to render pages:", err);
  } finally {
    isLoading.value = false;
  }
}

/** Load a new PDF file from disk, then render. */
async function loadPdf(filePath: string): Promise<void> {
  if (!filePath) return;
  await loadPdfjs();
  if (!pdfjsLib) return;

  const bytes = await readFile(filePath);
  if (currentBlobUrl) URL.revokeObjectURL(currentBlobUrl);
  currentBlobUrl = URL.createObjectURL(new Blob([bytes], { type: "application/pdf" }));

  cachedPdf = await pdfjsLib.getDocument(currentBlobUrl).promise;
  await renderPages();
}

// New PDF compiled → reload file and re-render
watch(() => previewStore.pdfRevision, async () => {
  const path = previewStore.lastSuccessfulPdfPath;
  if (path) await loadPdf(path);
});

// Scale changed → re-draw with cached PDF (no disk I/O)
watch(scale, async () => {
  await renderPages();
});

// Ctrl+wheel zoom on the PDF container
function onWheel(e: WheelEvent): void {
  if (!e.ctrlKey) return;
  e.preventDefault();
  const delta = e.deltaY > 0 ? -0.1 : 0.1;
  scale.value = Math.min(4, Math.max(0.5, Math.round((scale.value + delta) * 10) / 10));
}

onMounted(async () => {
  pdfContainer.value?.addEventListener("wheel", onWheel, { passive: false });
  const path = previewStore.lastSuccessfulPdfPath;
  if (path) await loadPdf(path);
});

onUnmounted(() => {
  pdfContainer.value?.removeEventListener("wheel", onWheel);
  if (currentBlobUrl) URL.revokeObjectURL(currentBlobUrl);
});
</script>

<template>
  <div class="pdf-viewer">
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
      class="pdf-pages pa-2"
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
</style>
