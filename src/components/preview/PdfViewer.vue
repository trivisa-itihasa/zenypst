<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import * as pdfjsLib from "pdfjs-dist";
import { TextLayer } from "pdfjs-dist";
import type { PDFDocumentProxy } from "pdfjs-dist";
import workerSrc from "pdfjs-dist/build/pdf.worker.min.mjs?url";
import { invoke } from "@tauri-apps/api/core";
import { usePreviewStore } from "@/stores/preview";
import { useEditorStore } from "@/stores/editor";
import CompileStatus from "./CompileStatus.vue";

pdfjsLib.GlobalWorkerOptions.workerSrc = workerSrc;

const previewStore = usePreviewStore();
const editorStore = useEditorStore();
const pagesContainer = ref<HTMLElement | null>(null);
const scale = ref(1.0);

let currentPdf: PDFDocumentProxy | null = null;
let renderToken = 0;
let zoomTimer: ReturnType<typeof setTimeout> | null = null;

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
  currentPdf?.destroy();
  currentPdf = null;
});

interface SourceLocation {
  line: number;
  col: number;
}

async function handleTextLayerClick(e: MouseEvent, pageIndex: number, userScale: number): Promise<void> {
  const target = e.currentTarget as HTMLElement;
  const rect = target.getBoundingClientRect();
  // Convert CSS pixel offset to typst pt (1 CSS px = 1/userScale pt)
  const xPt = (e.clientX - rect.left) / userScale;
  const yPt = (e.clientY - rect.top) / userScale;

  try {
    const loc = await invoke<SourceLocation | null>("locate_source", {
      pageIndex,
      xPt,
      yPt,
    });
    if (loc) {
      editorStore.requestJump(loc.line, loc.col);
    }
  } catch (err) {
    console.error("locate_source failed:", err);
  }
}

async function renderPages(pdf: PDFDocumentProxy, s: number, token: number): Promise<void> {
  if (!pagesContainer.value) return;

  const fragment = document.createDocumentFragment();

  for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
    if (token !== renderToken) return;

    const page = await pdf.getPage(pageNum);

    // Use separate viewports: high-res for canvas, display-res for text layer & CSS size
    const displayViewport = page.getViewport({ scale: s });
    const renderViewport = page.getViewport({ scale: s * 2 });

    // Page wrapper (relative-positioned so text layer can overlay)
    const wrapper = document.createElement("div");
    wrapper.style.cssText = [
      "position: relative",
      `width: ${displayViewport.width}px`,
      `height: ${displayViewport.height}px`,
      "margin: 8px auto",
      "box-shadow: 0 2px 8px rgba(0,0,0,0.3)",
      "overflow: hidden",
    ].join(";");
    // pdfjs-dist 4.x TextLayer uses var(--scale-factor) to position text spans
    wrapper.style.setProperty("--scale-factor", String(s));

    // Canvas (rendered at 2x for sharpness)
    const canvas = document.createElement("canvas");
    canvas.width = renderViewport.width;
    canvas.height = renderViewport.height;
    canvas.style.cssText = `display:block;width:100%;height:100%;`;

    const ctx = canvas.getContext("2d");
    if (!ctx) continue;
    await page.render({ canvasContext: ctx, viewport: renderViewport }).promise;

    wrapper.appendChild(canvas);

    // Text layer (overlay for selection and double-click sync)
    const textLayerDiv = document.createElement("div");
    textLayerDiv.className = "textLayer";
    textLayerDiv.style.cssText = "position:absolute;inset:0;";

    const textLayer = new TextLayer({
      textContentSource: page.streamTextContent(),
      container: textLayerDiv,
      viewport: displayViewport,
    });
    await textLayer.render();

    // Single click → jump editor cursor to source location
    // Drag and double-click are left to the browser for text selection
    const pageIndex = pageNum - 1;
    const userScale = s;
    let mouseDownX = 0;
    let mouseDownY = 0;
    textLayerDiv.addEventListener("mousedown", (e) => {
      mouseDownX = e.clientX;
      mouseDownY = e.clientY;
    });
    textLayerDiv.addEventListener("click", (e) => {
      if (e.detail !== 1) return; // ignore double-click
      const dx = e.clientX - mouseDownX;
      const dy = e.clientY - mouseDownY;
      if (dx * dx + dy * dy > 16) return; // ignore drag (>4px)
      handleTextLayerClick(e, pageIndex, userScale);
    });

    wrapper.appendChild(textLayerDiv);
    fragment.appendChild(wrapper);
  }

  if (token !== renderToken) return;
  pagesContainer.value.innerHTML = "";
  pagesContainer.value.appendChild(fragment);
}

async function loadPdf(pdfBase64: string): Promise<void> {
  const token = ++renderToken;

  const prev = currentPdf;
  currentPdf = null;
  prev?.destroy();

  const binary = atob(pdfBase64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);

  const pdf = await pdfjsLib.getDocument({ data: bytes }).promise;
  if (token !== renderToken) {
    pdf.destroy();
    return;
  }

  currentPdf = pdf;
  await renderPages(pdf, scale.value, token);
}

watch(
  () => previewStore.pdf,
  (pdf) => {
    if (pdf) loadPdf(pdf);
  },
  { immediate: true }
);

watch(scale, (s) => {
  if (!currentPdf) return;
  if (zoomTimer) clearTimeout(zoomTimer);
  const token = ++renderToken;
  zoomTimer = setTimeout(() => {
    if (currentPdf) renderPages(currentPdf, s, token);
  }, 80);
});
</script>

<template>
  <div class="pdf-viewer">
    <!-- Toolbar -->
    <div class="pdf-toolbar d-flex align-center px-2">
      <q-btn
        dense
        flat
        round
        size="xs"
        icon="mdi-magnify-minus-outline"
        @click="scale = Math.max(0.25, Math.round((scale - 0.25) * 10) / 10)"
      />
      <span class="text-caption mx-2">{{ Math.round(scale * 100) }}%</span>
      <q-btn
        dense
        flat
        round
        size="xs"
        icon="mdi-magnify-plus-outline"
        @click="scale = Math.min(4, Math.round((scale + 0.25) * 10) / 10)"
      />
      <q-space />
      <q-spinner
        v-if="previewStore.status === 'compiling'"
        size="16px"
        :thickness="2"
        color="primary"
        class="mr-1"
      />
    </div>

    <!-- Pages -->
    <div ref="pagesContainer" class="pdf-pages pa-2" />

    <!-- Empty state -->
    <div
      v-if="!previewStore.pdf && previewStore.status !== 'compiling'"
      class="pdf-empty"
    >
      <div class="pdf-empty__top" />
      <q-icon name="mdi-file-pdf-box" size="64px" color="grey-6" />
      <p class="text-medium-emphasis mt-4">PDF preview will appear here</p>
      <div class="pdf-empty__bottom" />
    </div>

    <!-- Compile Status Overlay -->
    <CompileStatus />
  </div>
</template>

<style scoped>
.pdf-viewer {
  position: absolute;
  inset: 0;
  background: var(--zen-surface-variant);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.pdf-toolbar {
  height: var(--panel-header-height);
  min-height: var(--panel-header-height);
  background: var(--zen-surface);
  border-bottom: 1px solid var(--zen-border);
  flex-shrink: 0;
}

.pdf-pages {
  flex: 1 1 0;
  overflow-y: auto;
  background: var(--zen-surface-variant);
  scrollbar-width: thin;
  scrollbar-color: var(--zen-border) transparent;
}

.pdf-pages::-webkit-scrollbar {
  width: var(--scrollbar-size);
}
.pdf-pages::-webkit-scrollbar-thumb {
  background: var(--zen-border);
  border-radius: var(--scrollbar-radius);
}
.pdf-pages::-webkit-scrollbar-track {
  background: transparent;
}

.pdf-pages--hidden {
  display: none;
}

.pdf-empty {
  position: absolute;
  inset: var(--panel-header-height) 0 0 0;
  pointer-events: none;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.pdf-empty__top { flex: 2; }
.pdf-empty__bottom { flex: 3; }
</style>

<!-- Text selection highlight color (global, targets dynamically created elements) -->
<style>
.pdf-pages .textLayer ::-moz-selection,
.pdf-pages .textLayer ::selection {
  background: rgba(100, 150, 255, 0.35);
  color: transparent;
}
</style>
