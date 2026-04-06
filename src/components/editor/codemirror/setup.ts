import { EditorState } from "@codemirror/state";
import {
  EditorView,
  keymap,
  lineNumbers,
  highlightActiveLine,
  highlightActiveLineGutter,
  drawSelection,
  dropCursor,
  rectangularSelection,
  crosshairCursor,
  ViewUpdate,
} from "@codemirror/view";
import {
  defaultKeymap,
  history,
  historyKeymap,
  indentWithTab,
} from "@codemirror/commands";
import {
  searchKeymap,
  highlightSelectionMatches,
} from "@codemirror/search";
import {
  bracketMatching,
  indentOnInput,
  foldGutter,
  foldKeymap,
} from "@codemirror/language";
import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
import { linter, lintGutter, setDiagnostics, type Diagnostic } from "@codemirror/lint";
import type { Extension } from "@codemirror/state";
import { typstLanguage } from "./typst-language";
import { buildThemeExtension } from "./theme";
import { fontCompletion } from "./font-completion";
import type { ThemeColors, CompileError } from "@/types";

export interface EditorConfig {
  doc: string;
  themeColors: ThemeColors;
  fontFamily: string;
  fontFamilyFallback: string;
  fontSize: number;
  showLineNumbers: boolean;
  wordWrap: boolean;
  onChange: (content: string) => void;
  /** Returns the list of available font names for in-editor font-name completion. */
  getFonts?: () => string[];
}

/**
 * Create a fully configured CodeMirror 6 EditorState.
 */
export function createEditorState(config: EditorConfig): EditorState {
  const extensions: Extension[] = [
    // Theme
    buildThemeExtension(config.themeColors),
    // Language
    typstLanguage(),
    // Lint (no-op linter installs the lint state; diagnostics pushed externally)
    linter(() => []),
    lintGutter(),
    // Editor features
    history(),
    drawSelection(),
    dropCursor(),
    indentOnInput(),
    bracketMatching(),
    closeBrackets(),
    rectangularSelection(),
    crosshairCursor(),
    highlightSelectionMatches(),
    highlightActiveLine(),
    highlightActiveLineGutter(),
    // Keymaps
    keymap.of([
      ...closeBracketsKeymap,
      ...defaultKeymap,
      ...historyKeymap,
      ...searchKeymap,
      ...foldKeymap,
      indentWithTab,
    ]),
    // Font styling via CSS
    EditorView.theme({
      ".cm-content": {
        fontFamily: `"${config.fontFamily}", "${config.fontFamilyFallback}", monospace`,
        fontSize: `${config.fontSize}px`,
      },
      ".cm-gutters": {
        fontFamily: `"${config.fontFamily}", "${config.fontFamilyFallback}", monospace`,
        fontSize: `${config.fontSize}px`,
      },
    }),
    // Font name completion (triggers inside `font: "..."` strings)
    ...(config.getFonts ? [fontCompletion(config.getFonts)] : []),
    // Change listener
    EditorView.updateListener.of((update: ViewUpdate) => {
      if (update.docChanged) {
        config.onChange(update.state.doc.toString());
      }
    }),
  ];

  if (config.showLineNumbers) {
    extensions.push(lineNumbers(), foldGutter());
  }

  if (config.wordWrap) {
    extensions.push(EditorView.lineWrapping);
  }

  return EditorState.create({
    doc: config.doc,
    extensions,
  });
}

/**
 * Convert a CompileError to a CodeMirror Diagnostic, or null if not applicable.
 * Only errors for the main file ("main.typ") are shown inline in the editor.
 */
function errorToDiagnostic(err: CompileError, view: EditorView): Diagnostic | null {
  if (!err.line) return null;
  // Show only errors belonging to the main document (no file = main.typ)
  if (err.file && err.file !== "main.typ") return null;

  const doc = view.state.doc;
  const lineNum = err.line;
  if (lineNum < 1 || lineNum > doc.lines) return null;

  const line = doc.line(lineNum);
  const col = Math.max(0, (err.column ?? 1) - 1);
  const from = Math.min(line.from + col, line.to);

  let to: number;
  if (err.endLine && err.endColumn) {
    const endLineNum = err.endLine;
    if (endLineNum >= 1 && endLineNum <= doc.lines) {
      const endLine = doc.line(endLineNum);
      to = Math.min(endLine.from + err.endColumn - 1, endLine.to);
    } else {
      to = line.to;
    }
  } else {
    to = line.to;
  }

  // Ensure the range is at least 1 character wide so CM renders the underline
  if (to <= from) to = Math.min(from + 1, doc.length);

  return {
    from,
    to,
    severity: err.severity === "error" ? "error" : "warning",
    message: err.message,
  };
}

/**
 * Push compile diagnostics (errors + warnings) into a CodeMirror view.
 * Call this whenever previewStore.errors / .warnings change.
 */
export function applyDiagnostics(
  view: EditorView,
  errors: CompileError[],
  warnings: CompileError[]
): void {
  const diagnostics: Diagnostic[] = [];
  for (const err of errors) {
    const d = errorToDiagnostic(err, view);
    if (d) diagnostics.push(d);
  }
  for (const warn of warnings) {
    const d = errorToDiagnostic(warn, view);
    if (d) diagnostics.push(d);
  }
  view.dispatch(setDiagnostics(view.state, diagnostics));
}

/**
 * Build a set of reconfigurable extensions for theme/font updates.
 */
export function buildDynamicExtensions(
  themeColors: ThemeColors,
  fontFamily: string,
  fontFamilyFallback: string,
  fontSize: number
): Extension[] {
  return [
    buildThemeExtension(themeColors),
    EditorView.theme({
      ".cm-content": {
        fontFamily: `"${fontFamily}", "${fontFamilyFallback}", monospace`,
        fontSize: `${fontSize}px`,
      },
      ".cm-gutters": {
        fontFamily: `"${fontFamily}", "${fontFamilyFallback}", monospace`,
        fontSize: `${fontSize}px`,
      },
    }),
  ];
}
