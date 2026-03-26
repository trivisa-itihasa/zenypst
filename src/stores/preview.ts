import { defineStore } from "pinia";
import { ref } from "vue";
import type { CompileError } from "@/types";

export type CompileStatus = "idle" | "compiling" | "success" | "error";

export const usePreviewStore = defineStore("preview", () => {
  const status = ref<CompileStatus>("idle");
  const pdf = ref<string | null>(null); // base64-encoded PDF
  const errors = ref<CompileError[]>([]);
  const warnings = ref<CompileError[]>([]);

  function setCompiling(): void {
    status.value = "compiling";
    errors.value = [];
    warnings.value = [];
  }

  function setSuccess(newPdf: string): void {
    status.value = "success";
    pdf.value = newPdf;
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
    pdf,
    errors,
    warnings,
    setCompiling,
    setSuccess,
    setError,
    setIdle,
  };
});
