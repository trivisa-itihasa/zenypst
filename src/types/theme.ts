/** Color tokens used by the editor theme. */
export interface ThemeColors {
  background: string;
  foreground: string;
  caret: string;
  selection: string;
  lineHighlight: string;
  gutterBackground: string;
  gutterForeground: string;
  // Typst-specific tokens
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
  colors: ThemeColors;
}

/** Default dark theme colors. */
export const DEFAULT_DARK_COLORS: ThemeColors = {
  background: "#1e1e2e",
  foreground: "#cdd6f4",
  caret: "#f5e0dc",
  selection: "#45475a",
  lineHighlight: "#313244",
  gutterBackground: "#1e1e2e",
  gutterForeground: "#6c7086",
  heading: "#f38ba8",
  emphasis: "#f5c2e7",
  strong: "#fab387",
  keyword: "#cba6f7",
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
