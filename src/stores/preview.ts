import { defineStore } from "pinia";
import { ref } from "vue";
import type { CompileError } from "@/types";

export type CompileStatus = "idle" | "compiling" | "success" | "error";

export const usePreviewStore = defineStore("preview", () => {
  const status = ref<CompileStatus>("idle");
  const pages = ref<string[]>([]); // base64 PNG pages
  const errors = ref<CompileError[]>([]);
  const warnings = ref<CompileError[]>([]);

  function setCompiling(): void {
    status.value = "compiling";
    errors.value = [];
    warnings.value = [];
  }

  function setSuccess(newPages: string[]): void {
    status.value = "success";
    pages.value = newPages;
    errors.value = [];
    warnings.value = [];
  }

  function setError(errs: CompileError[], warns: CompileError[]): void {
    status.value = "error";
    errors.value = errs;
    warnings.value = warns;
    // Keep last successful pages visible
  }

  function setIdle(): void {
    status.value = "idle";
  }

  return {
    status,
    pages,
    errors,
    warnings,
    setCompiling,
    setSuccess,
    setError,
    setIdle,
  };
});
