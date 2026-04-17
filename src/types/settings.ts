/** Application settings. */
export interface AppSettings {
  // Editor
  fontFamily: string;
  fontFamilyFallback: string;
  fontSize: number;
  showLineNumbers: boolean;
  wordWrap: boolean;

  // Theme
  activeThemeId: string;

  // Preview
  previewMode: "realtime" | "on_save" | "manual";
  realtimeDebounceMs: number;

  // Layout
  fileTreeWidth: number;
  previewWidth: number;
  fileTreeVisible: boolean;
  previewVisible: boolean;

  // App
  uiFontSize: number;
  lastOpenedPath: string | null;
  recentPaths: string[];

}

export const DEFAULT_SETTINGS: AppSettings = {
  fontFamily: "Fira Code",
  fontFamilyFallback: "monospace",
  fontSize: 14,
  showLineNumbers: true,
  wordWrap: false,
  activeThemeId: "zenypst-dark",
  previewMode: "realtime",
  realtimeDebounceMs: 150,
  fileTreeWidth: 250,
  previewWidth: 400,
  fileTreeVisible: true,
  previewVisible: true,
  uiFontSize: 13,
  lastOpenedPath: null,
  recentPaths: [],
};

/** Typst compilation result from the Rust backend. */
export interface CompileError {
  severity: string;
  message: string;
  file?: string;
  line?: number;
  column?: number;
  endLine?: number;
  endColumn?: number;
  sourceLine?: string;
  hints?: string[];
}

export interface CompileResult {
  success: boolean;
  pdfPath?: string;
  errors: CompileError[];
  warnings: CompileError[];
}
