import { defineStore } from "pinia";
import { ref } from "vue";
import type { CompileError } from "@/types";

export type CompileStatus = "idle" | "compiling" | "success" | "error";

export const usePreviewStore = defineStore("preview", () => {
  const status = ref<CompileStatus>("idle");
  const pdfPath = ref<string | null>(null);
  const errors = ref<CompileError[]>([]);
  const warnings = ref<CompileError[]>([]);
  const lastSuccessfulPdfPath = ref<string | null>(null);
  const pdfRevision = ref(0); // Increment to force PDF reload

  function setCompiling(): void {
    status.value = "compiling";
    errors.value = [];
    warnings.value = [];
  }

  function setSuccess(path: string): void {
    status.value = "success";
    pdfPath.value = path;
    lastSuccessfulPdfPath.value = path;
    pdfRevision.value++;
    errors.value = [];
  }

  function setError(errs: CompileError[], warns: CompileError[]): void {
    status.value = "error";
    errors.value = errs;
    warnings.value = warns;
    // Keep the last successful PDF path visible
  }

  function setIdle(): void {
    status.value = "idle";
  }

  return {
    status,
    pdfPath,
    errors,
    warnings,
    lastSuccessfulPdfPath,
    pdfRevision,
    setCompiling,
    setSuccess,
    setError,
    setIdle,
  };
});
