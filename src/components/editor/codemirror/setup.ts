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
import type { Extension } from "@codemirror/state";
import { typstLanguage } from "./typst-language";
import { buildThemeExtension } from "./theme";
import type { ThemeColors } from "@/types";

export interface EditorConfig {
  doc: string;
  themeColors: ThemeColors;
  fontFamily: string;
  fontSize: number;
  showLineNumbers: boolean;
  wordWrap: boolean;
  onChange: (content: string) => void;
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
        fontFamily: `"${config.fontFamily}", "Fira Code", "Consolas", "Liberation Mono", monospace`,
        fontSize: `${config.fontSize}px`,
      },
      ".cm-gutters": {
        fontFamily: `"${config.fontFamily}", "Fira Code", "Consolas", "Liberation Mono", monospace`,
        fontSize: `${config.fontSize}px`,
      },
    }),
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
 * Build a set of reconfigurable extensions for theme/font updates.
 */
export function buildDynamicExtensions(
  themeColors: ThemeColors,
  fontFamily: string,
  fontSize: number
): Extension[] {
  return [
    buildThemeExtension(themeColors),
    EditorView.theme({
      ".cm-content": {
        fontFamily: `"${fontFamily}", "Fira Code", "Consolas", "Liberation Mono", monospace`,
        fontSize: `${fontSize}px`,
      },
      ".cm-gutters": {
        fontFamily: `"${fontFamily}", "Fira Code", "Consolas", "Liberation Mono", monospace`,
        fontSize: `${fontSize}px`,
      },
    }),
  ];
}
