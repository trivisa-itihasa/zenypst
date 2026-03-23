import { EditorView } from "@codemirror/view";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";
import type { Extension } from "@codemirror/state";
import type { ThemeColors } from "@/types";

/**
 * Build a CodeMirror 6 Extension from a ThemeColors object.
 * Applies both the base theme (background, gutter, selection…) and token highlighting.
 */
export function buildThemeExtension(colors: ThemeColors): Extension {
  const baseTheme = EditorView.theme(
    {
      "&": {
        backgroundColor: colors.background,
        color: colors.foreground,
        height: "100%",
      },
      ".cm-content": {
        caretColor: colors.caret,
        fontFamily: "inherit",
        fontSize: "inherit",
      },
      ".cm-cursor, .cm-dropCursor": {
        borderLeftColor: colors.caret,
      },
      "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection": {
        backgroundColor: colors.selection,
      },
      ".cm-activeLine": {
        backgroundColor: colors.lineHighlight,
      },
      ".cm-gutters": {
        backgroundColor: colors.gutterBackground,
        color: colors.gutterForeground,
        border: "none",
      },
      ".cm-activeLineGutter": {
        backgroundColor: colors.lineHighlight,
      },
      ".cm-lineNumbers .cm-gutterElement": {
        color: colors.gutterForeground,
      },
      ".cm-matchingBracket, .cm-nonmatchingBracket": {
        backgroundColor: colors.selection,
        outline: `1px solid ${colors.bracket}`,
      },
      ".cm-searchMatch": {
        backgroundColor: colors.selection,
        outline: `1px solid ${colors.operator}`,
      },
      ".cm-selectionMatch": {
        backgroundColor: colors.selection,
      },
      ".cm-tooltip": {
        backgroundColor: colors.gutterBackground,
        border: `1px solid ${colors.bracket}`,
      },
      ".cm-tooltip.cm-tooltip-autocomplete > ul > li[aria-selected]": {
        backgroundColor: colors.lineHighlight,
        color: colors.foreground,
      },
      ".cm-panels": {
        backgroundColor: colors.gutterBackground,
        color: colors.foreground,
      },
      ".cm-panels input": {
        backgroundColor: colors.background,
        color: colors.foreground,
        border: `1px solid ${colors.bracket}`,
      },
      ".cm-button": {
        backgroundImage: "none",
        backgroundColor: colors.lineHighlight,
        color: colors.foreground,
        border: `1px solid ${colors.bracket}`,
      },
    },
    { dark: isDark(colors.background) }
  );

  const highlightStyle = HighlightStyle.define([
    { tag: t.heading, color: colors.heading, fontWeight: "bold" },
    { tag: t.emphasis, color: colors.emphasis, fontStyle: "italic" },
    { tag: t.strong, color: colors.strong, fontWeight: "bold" },
    { tag: t.keyword, color: colors.keyword },
    { tag: t.variableName, color: colors.function },
    { tag: t.string, color: colors.string },
    { tag: t.number, color: colors.number },
    { tag: t.lineComment, color: colors.comment, fontStyle: "italic" },
    { tag: t.blockComment, color: colors.comment, fontStyle: "italic" },
    { tag: t.meta, color: colors.rawBlock },
    { tag: t.link, color: colors.label },
    { tag: t.operator, color: colors.operator },
    { tag: t.bracket, color: colors.bracket },
    { tag: t.punctuation, color: colors.bracket },
    { tag: t.typeName, color: colors.function },
    { tag: t.className, color: colors.heading },
    { tag: t.definition(t.variableName), color: colors.function },
    { tag: t.special(t.string), color: colors.math },
    { tag: t.atom, color: colors.keyword },
    { tag: t.bool, color: colors.keyword },
    { tag: t.null, color: colors.keyword },
    { tag: t.invalid, color: "#ff0000" },
  ]);

  return [baseTheme, syntaxHighlighting(highlightStyle)];
}

/** Determine if a hex color is dark (for EditorView.theme dark parameter). */
function isDark(hex: string): boolean {
  const color = hex.replace("#", "");
  const r = parseInt(color.substring(0, 2), 16);
  const g = parseInt(color.substring(2, 4), 16);
  const b = parseInt(color.substring(4, 6), 16);
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  return luminance < 0.5;
}
