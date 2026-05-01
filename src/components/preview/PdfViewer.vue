<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import * as pdfjsLib from "pdfjs-dist";
import { TextLayer } from "pdfjs-dist";
import type { PDFDocumentProxy, PDFPageProxy } from "pdfjs-dist";
import workerSrc from "pdfjs-dist/build/pdf.worker.min.mjs?url";
import { invoke } from "@tauri-apps/api/core";
import { readFile } from "@tauri-apps/plugin-fs";
import { useI18n } from "vue-i18n";
import { usePreviewStore } from "@/stores/preview";
import { useEditorStore } from "@/stores/editor";
import CompileStatus from "./CompileStatus.vue";

const { t } = useI18n();

pdfjsLib.GlobalWorkerOptions.workerSrc = workerSrc;

const previewStore = usePreviewStore();
const editorStore = useEditorStore();
const pagesContainer = ref<HTMLElement | null>(null);
const scale = ref(1.0);

let currentPdf: PDFDocumentProxy | null = null;
let currentBlobUrl: string | null = null;
let renderToken = 0;
let zoomTimer: ReturnType<typeof setTimeout> | null = null;
let observer: IntersectionObserver | null = null;

let pageObjects: PDFPageProxy[] = [];
let pageWrappers: (HTMLElement | null)[] = [];
let renderedPages = new Set<number>();
let renderingPages = new Set<number>();

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
  observer?.disconnect();
  currentPdf?.destroy();
  currentPdf = null;
  if (currentBlobUrl) {
    URL.revokeObjectURL(currentBlobUrl);
    currentBlobUrl = null;
  }
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

/** Limit canvas resolution to avoid GPU/Software rendering issues on WSLg */
function getRenderScale(displayScale: number): number {
  // Cap at 1.5x DPR on WSLg/WebKit to keep canvas size reasonable
  const dpr = typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1;
  return displayScale * Math.min(dpr, 1.5);
}

async function renderPage(pageIndex: number, token: number): Promise<void> {
  if (token !== renderToken) return;
  if (renderedPages.has(pageIndex) || renderingPages.has(pageIndex)) return;

  const page = pageObjects[pageIndex];
  if (!page) return;

  renderingPages.add(pageIndex);
  try {
    const wrapper = pageWrappers[pageIndex];
    if (!wrapper) return;

    const s = scale.value;
    const displayViewport = page.getViewport({ scale: s });
    const renderViewport = page.getViewport({ scale: getRenderScale(s) });

    wrapper.innerHTML = "";
    wrapper.style.background = "#fff";

    const canvas = document.createElement("canvas");
    canvas.width = renderViewport.width;
    canvas.height = renderViewport.height;
    canvas.style.cssText = "display:block;width:100%;height:100%;";

    const ctx = canvas.getContext("2d", { alpha: false });
    if (!ctx) {
      console.error(`[PDF] No 2D context for page ${pageIndex}`);
      return;
    }

    console.log(`[PDF] Rendering page ${pageIndex + 1} at ${canvas.width}x${canvas.height}`);
    const renderTask = page.render({ canvasContext: ctx, viewport: renderViewport });
    await renderTask.promise;
    console.log(`[PDF] Rendered page ${pageIndex + 1}`);

    if (token !== renderToken) return;

    wrapper.appendChild(canvas);
    console.log(`[PDF] Appended canvas ${canvas.width}x${canvas.height} to wrapper ${wrapper.clientWidth}x${wrapper.clientHeight}`);

    // TEMP: Skip TextLayer to isolate whether canvas itself is visible.
    // If removing this makes text appear, the issue is with TextLayer CSS/overlay.
    /*
    const textLayerDiv = document.createElement("div");
    textLayerDiv.className = "textLayer";
    textLayerDiv.style.cssText = "position:absolute;inset:0;";
    wrapper.style.setProperty("--scale-factor", String(s));

    const textLayer = new TextLayer({
      textContentSource: page.streamTextContent(),
      container: textLayerDiv,
      viewport: displayViewport,
    });
    await textLayer.render();
    if (token !== renderToken) return;

    const userScale = s;
    let mouseDownX = 0;
    let mouseDownY = 0;
    textLayerDiv.addEventListener("mousedown", (e) => {
      mouseDownX = e.clientX;
      mouseDownY = e.clientY;
    });
    textLayerDiv.addEventListener("click", (e) => {
      if (e.detail !== 1) return;
      const dx = e.clientX - mouseDownX;
      const dy = e.clientY - mouseDownY;
      if (dx * dx + dy * dy > 16) return;
      handleTextLayerClick(e, pageIndex, userScale);
    });

    wrapper.appendChild(textLayerDiv);
    */

    renderedPages.add(pageIndex);
  } catch (err) {
    console.error(`Failed to render page ${pageIndex}:`, err);
  } finally {
    renderingPages.delete(pageIndex);
  }
}

function clearAllRendered(): void {
  for (const wrapper of pageWrappers) {
    if (wrapper) wrapper.innerHTML = "";
  }
  renderedPages.clear();
  renderingPages.clear();
}

async function loadPdf(pdfPath: string): Promise<void> {
  const token = ++renderToken;

  // Disconnect old observer but keep content visible until the new one is ready
  if (observer) {
    observer.disconnect();
    observer = null;
  }

  const prev = currentPdf;
  currentPdf = null;
  prev?.destroy();

  if (currentBlobUrl) {
    URL.revokeObjectURL(currentBlobUrl);
    currentBlobUrl = null;
  }

  if (!pagesContainer.value) return;

  const container = pagesContainer.value;
  const oldScrollTop = container.scrollTop;

  try {
    const bytes = await readFile(pdfPath);
    console.log(`[PDF] readFile returned ${bytes.length} bytes from ${pdfPath}`);

    // Use a Blob URL instead of passing raw bytes directly;
    // some WebKit builds on WSLg handle {url} better than {data}.
    const blob = new Blob([bytes], { type: "application/pdf" });
    const blobUrl = URL.createObjectURL(blob);
    currentBlobUrl = blobUrl;

    const pdf = await pdfjsLib.getDocument({ url: blobUrl }).promise;
    if (token !== renderToken) {
      pdf.destroy();
      return;
    }

    currentPdf = pdf;

    // Preload all page objects to determine exact wrapper dimensions
    const pages: PDFPageProxy[] = [];
    for (let i = 1; i <= pdf.numPages; i++) {
      pages.push(await pdf.getPage(i));
      if (token !== renderToken) return;
    }

    pageObjects = pages;

    const fragment = document.createDocumentFragment();
    const wrappers: (HTMLElement | null)[] = [];

    for (let i = 0; i < pages.length; i++) {
      const page = pages[i];
      const viewport = page.getViewport({ scale: scale.value });

      const wrapper = document.createElement("div");
      wrapper.style.cssText = [
        "position: relative",
        `width: ${viewport.width}px`,
        `height: ${viewport.height}px`,
        "margin: 8px auto",
        "box-shadow: 0 2px 8px rgba(0,0,0,0.3)",
        "overflow: hidden",
        "background: #fff",
      ].join(";");
      wrapper.dataset.pageIndex = String(i);

      fragment.appendChild(wrapper);
      wrappers.push(wrapper);
    }

    // Update state so renderPage can find the new wrappers
    pageWrappers = wrappers;
    renderedPages.clear();
    renderingPages.clear();

    // Pre-render the pages that will be visible after swap (based on old scroll)
    const pageHeightPx = pages[0]?.getViewport({ scale: scale.value }).height ?? 800;
    const margin = 16;
    const startPage = Math.max(0, Math.floor(oldScrollTop / (pageHeightPx + margin)));
    const visibleCount = Math.ceil(container.clientHeight / (pageHeightPx + margin)) + 1;
    const endPage = Math.min(pages.length, startPage + visibleCount + 1);

    for (let i = startPage; i < endPage; i++) {
      await renderPage(i, token);
      if (token !== renderToken) return;
    }

    // Atomically swap old content for new – this avoids the blank flash
    container.innerHTML = "";
    container.appendChild(fragment);
    container.scrollTop = oldScrollTop;

    observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const idx = Number((entry.target as HTMLElement).dataset.pageIndex);
            if (!Number.isNaN(idx) && !renderedPages.has(idx)) {
              renderPage(idx, renderToken);
            }
          }
        }
      },
      { root: container, rootMargin: "200px" }
    );

    for (const wrapper of wrappers) {
      if (wrapper) observer.observe(wrapper);
    }
  } catch (err) {
    console.error("Failed to load PDF:", err);
  }
}

watch(
  () => previewStore.compileCount,
  () => {
    if (previewStore.pdfPath) loadPdf(previewStore.pdfPath);
  },
  { immediate: true }
);

watch(scale, (s) => {
  if (!currentPdf) return;
  if (zoomTimer) clearTimeout(zoomTimer);
  const token = ++renderToken;
  zoomTimer = setTimeout(() => {
    if (!currentPdf) return;

    // Update wrapper sizes and clear rendered content
    for (let i = 0; i < pageObjects.length; i++) {
      const page = pageObjects[i];
      const wrapper = pageWrappers[i];
      if (!page || !wrapper) continue;
      const viewport = page.getViewport({ scale: s });
      wrapper.style.width = `${viewport.width}px`;
      wrapper.style.height = `${viewport.height}px`;
      wrapper.innerHTML = "";
    }

    renderedPages.clear();
    renderingPages.clear();

    // Manually trigger render for pages currently visible in the viewport
    const container = pagesContainer.value;
    if (!container) return;
    const containerRect = container.getBoundingClientRect();

    for (let i = 0; i < pageWrappers.length; i++) {
      const wrapper = pageWrappers[i];
      if (!wrapper) continue;
      const rect = wrapper.getBoundingClientRect();
      if (rect.bottom >= containerRect.top - 200 && rect.top <= containerRect.bottom + 200) {
        renderPage(i, token);
      }
    }
  }, 150);
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
      v-if="!previewStore.pdfPath && previewStore.status !== 'compiling'"
      class="pdf-empty"
    >
      <div class="pdf-empty__top" />
      <q-icon name="mdi-file-pdf-box" size="64px" color="grey-6" />
      <p class="text-medium-emphasis mt-4 empty-message">{{ t('pdfViewer.emptyState') }}</p>
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

.empty-message {
  height: 1lh;
  overflow: visible;
  text-align: center;
  padding: 0 16px;
}
</style>

<!-- Text selection highlight color (global, targets dynamically created elements) -->
<style>
.pdf-pages .textLayer ::-moz-selection,
.pdf-pages .textLayer ::selection {
  background: rgba(100, 150, 255, 0.35);
  color: transparent;
}
</style>
