import { defineStore } from "pinia";
import { ref } from "vue";
import type { CompileError } from "@/types";

export type CompileStatus = "idle" | "compiling" | "success" | "error";

export const usePreviewStore = defineStore("preview", () => {
  const status = ref<CompileStatus>("idle");
  const pdfPath = ref<string | null>(null); // absolute path to the generated PDF
  const errors = ref<CompileError[]>([]);
  const warnings = ref<CompileError[]>([]);
  const compileCount = ref(0); // increments on every successful compile so watchers fire

  function setCompiling(): void {
    status.value = "compiling";
    errors.value = [];
    warnings.value = [];
  }

  function setSuccess(newPdfPath: string): void {
    status.value = "success";
    pdfPath.value = newPdfPath;
    compileCount.value += 1;
    errors.value = [];
    warnings.value = [];
  }

  function setError(errs: CompileError[], warns: CompileError[]): void {
    status.value = "error";
    errors.value = errs;
    warnings.value = warns;
    // Keep last successful PDF visible
  }

  function setIdle(): void {
    status.value = "idle";
  }

  return {
    status,
    pdfPath,
    compileCount,
    errors,
    warnings,
    setCompiling,
    setSuccess,
    setError,
    setIdle,
  };
});
