/** Color tokens used by both the app UI and the editor. */
export interface ThemeColors {
  // ─── App UI ───
  appBackground: string;
  surface: string;
  surfaceVariant: string;
  border: string;
  uiText: string;
  uiTextMuted: string;
  primary: string;
  statusBar: string;
  statusBarText: string;
  error: string;
  warning: string;
  info: string;
  success: string;

  // ─── Editor pane ───
  background: string;
  foreground: string;
  caret: string;
  selection: string;
  lineHighlight: string;
  gutterBackground: string;
  gutterForeground: string;

  // ─── Syntax tokens ───
  heading: string;
  emphasis: string;
  strong: string;
  keyword: string;
  function: string;
  string: string;
  number: string;
  comment: string;
  math: string;
  label: string;
  rawBlock: string;
  operator: string;
  bracket: string;
}

/** A full theme definition. */
export interface Theme {
  id: string;
  name: string;
  builtIn: boolean;
  isDark: boolean;
  colors: ThemeColors;
}


/** Default dark theme colors. */
export const DEFAULT_DARK_COLORS: ThemeColors = {
  // App UI
  appBackground: "#1e1e2e",
  surface: "#181825",
  surfaceVariant: "#313244",
  border: "rgba(205, 214, 244, 0.12)",
  uiText: "#cdd6f4",
  uiTextMuted: "rgba(205, 214, 244, 0.6)",
  primary: "#9e9e9e",
  statusBar: "#239dad",
  statusBarText: "#ffffff",
  error: "#ef5350",
  warning: "#fab387",
  info: "#89dceb",
  success: "#a6e3a1",

  // Editor pane
  background: "#1e1e2e",
  foreground: "#cdd6f4",
  caret: "#f5e0dc",
  selection: "#45475a",
  lineHighlight: "#313244",
  gutterBackground: "#1e1e2e",
  gutterForeground: "#6c7086",

  // Syntax tokens
  heading: "#b8b8b8",
  emphasis: "#cacaca",
  strong: "#fab387",
  keyword: "#a8a8a8",
  function: "#89b4fa",
  string: "#a6e3a1",
  number: "#fab387",
  comment: "#6c7086",
  math: "#f9e2af",
  label: "#74c7ec",
  rawBlock: "#a6adc8",
  operator: "#89dceb",
  bracket: "#9399b2",
};
