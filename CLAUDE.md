# CLAUDE.md — Zenypst

## Project Overview

**Zenypst** is a desktop Typst editor application built with Tauri v2, Vue 3, and Vuetify. It provides a VSCode-like editing experience specifically tailored for Typst document authoring, with live PDF preview, customizable syntax highlighting themes, and a template management system.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop runtime | Tauri v2 (Rust backend) |
| Frontend framework | Vue 3 (Composition API + `<script setup>`) |
| UI library | Vuetify 3 |
| Code editor | CodeMirror 6 |
| Typst compilation | Typst CLI (invoked from Tauri/Rust side) |
| PDF rendering | pdf.js (embedded in right panel) |
| State management | Pinia |
| Build tool | Vite |
| Language | TypeScript throughout (frontend), Rust (Tauri backend) |

---

## Architecture

### Directory Structure

```
zenypst/
├── CLAUDE.md
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── file.rs          # File I/O commands (read, write, list directory)
│   │   │   ├── typst.rs         # Typst CLI invocation & compilation
│   │   │   ├── template.rs      # Template CRUD operations
│   │   │   └── settings.rs      # App settings persistence
│   │   └── utils/
│   │       ├── mod.rs
│   │       └── typst_cli.rs     # Typst CLI detection, path resolution, error handling
│   └── icons/
├── src/
│   ├── main.ts
│   ├── App.vue
│   ├── plugins/
│   │   └── vuetify.ts
│   ├── router/
│   │   └── index.ts
│   ├── stores/
│   │   ├── editor.ts            # Active file, buffer content, dirty state
│   │   ├── fileTree.ts          # Directory tree state, open/close nodes
│   │   ├── settings.ts          # App settings (font, preview mode, theme)
│   │   ├── template.ts          # Template list and management
│   │   └── preview.ts           # PDF preview state, compilation status
│   ├── components/
│   │   ├── layout/
│   │   │   ├── AppShell.vue         # Top-level layout: toolbar + 3-panel body
│   │   │   ├── Toolbar.vue          # Top menu bar (File, Edit, View, Settings, Help)
│   │   │   └── StatusBar.vue        # Bottom status bar (line/col, compile status, etc.)
│   │   ├── sidebar/
│   │   │   ├── FileTree.vue         # Recursive file/folder tree (left panel)
│   │   │   └── FileTreeItem.vue     # Single tree node (file or folder)
│   │   ├── editor/
│   │   │   ├── EditorPanel.vue      # CodeMirror 6 wrapper + tab bar
│   │   │   ├── TabBar.vue           # Open file tabs
│   │   │   └── codemirror/
│   │   │       ├── setup.ts         # CodeMirror extensions & config
│   │   │       ├── typst-language.ts  # Typst language support (Lezer grammar)
│   │   │       └── theme.ts         # Dynamic theme generation from user settings
│   │   ├── preview/
│   │   │   ├── PdfViewer.vue        # pdf.js-based PDF rendering (right panel)
│   │   │   └── CompileStatus.vue    # Compile progress / error display overlay
│   │   ├── settings/
│   │   │   ├── SettingsDialog.vue   # Main settings modal
│   │   │   ├── ThemeEditor.vue      # Syntax highlight theme editor (color pickers)
│   │   │   ├── FontSettings.vue     # Font family & size selector
│   │   │   └── PreviewSettings.vue  # Preview update mode toggle
│   │   └── template/
│   │       ├── TemplateManager.vue      # Template list, create/edit/delete UI
│   │       ├── TemplatePickerDialog.vue # Dialog shown on "New File" to select template
│   │       └── TemplateEditorDialog.vue # Create/edit a template's name & content
│   ├── composables/
│   │   ├── useCompiler.ts       # Typst compilation logic (debounce, trigger modes)
│   │   ├── useFileOps.ts        # File open/save/create wrappers over Tauri commands
│   │   ├── useTheme.ts          # Load/apply/save CodeMirror themes
│   │   └── useKeybindings.ts    # Global keyboard shortcuts
│   ├── types/
│   │   ├── index.ts
│   │   ├── file.ts              # FileNode, FileTab, etc.
│   │   ├── theme.ts             # ThemeDefinition (colors, token styles)
│   │   ├── template.ts          # Template type definition
│   │   └── settings.ts          # AppSettings type
│   └── assets/
│       └── styles/
│           └── global.scss
├── package.json
├── tsconfig.json
├── vite.config.ts
└── index.html
```

### Data Flow

```
User edits in CodeMirror 6
        │
        ▼
  Pinia editor store (buffer updated)
        │
        ▼
  useCompiler composable (respects preview mode setting)
        │
        ├─ "realtime" → debounce 500ms → trigger compile
        ├─ "on_save"  → trigger on Ctrl+S / manual save
        └─ "manual"   → trigger on button click only
        │
        ▼
  Tauri command: compile_typst
        │
        ▼
  Rust backend: invoke `typst compile <input> <output.pdf>`
        │
        ▼
  Return PDF path (or compilation errors) → frontend
        │
        ▼
  PdfViewer loads/refreshes the PDF via pdf.js
```

---

## Core Features & Specifications

### 1. Editor (CodeMirror 6)

- **Language support**: Typst syntax highlighting via a custom Lezer grammar definition or community grammar (`codemirror-lang-typst` if available; otherwise implement a basic Lezer grammar covering Typst's markup, math mode, code blocks, headings, labels, references, and comments).
- **Multiple tabs**: Support opening multiple files in tabs. Each tab tracks its own undo history and dirty state.
- **Standard keybindings**: Ctrl+S (save), Ctrl+Z/Y (undo/redo), Ctrl+F (find), Ctrl+Shift+F (find & replace), Ctrl+N (new file), Ctrl+O (open file/folder).
- **Line numbers**: Enabled by default, togglable.

### 2. Syntax Highlighting Themes

- **Theme structure**: A theme defines colors for the following token categories:
  - `background` — Editor background color
  - `foreground` — Default text color
  - `caret` — Cursor color
  - `selection` — Selection highlight color
  - `lineHighlight` — Current line highlight color
  - `gutterBackground` — Line number gutter background
  - `gutterForeground` — Line number text color
  - Token-specific colors:
    - `heading` — `= Heading` markup
    - `emphasis` — `_italic_` markup
    - `strong` — `*bold*` markup
    - `keyword` — `#set`, `#let`, `#show`, `#import`, `#if`, `#for`, etc.
    - `function` — Function names after `#`
    - `string` — `"string literals"`
    - `number` — Numeric literals
    - `comment` — `// comments`
    - `math` — `$ math mode $`
    - `label` — `<label>` and `@reference`
    - `rawBlock` — `` `raw` `` and ` ```code blocks``` `
    - `operator` — `+`, `-`, `*`, `/`, `=`, etc.
    - `bracket` — `()`, `[]`, `{}`
- **Built-in themes**: Ship with at least 3 presets:
  - "Zenypst Light" (light background, dark text)
  - "Zenypst Dark" (dark background, light text — default)
  - "Solarized Dark"
- **Custom themes**: Users can create, edit, duplicate, rename, and delete custom themes via the Theme Editor UI (color pickers for each token category).
- **Persistence**: Themes are saved as JSON files in the app data directory:
  - Linux: `~/.local/share/zenypst/themes/`
  - macOS: `~/Library/Application Support/zenypst/themes/`
  - Windows: `%APPDATA%/zenypst/themes/`
- **Theme file format**:
  ```json
  {
    "name": "My Custom Theme",
    "builtIn": false,
    "colors": {
      "background": "#1e1e2e",
      "foreground": "#cdd6f4",
      "caret": "#f5e0dc",
      "selection": "#45475a",
      "lineHighlight": "#313244",
      "gutterBackground": "#1e1e2e",
      "gutterForeground": "#6c7086",
      "heading": "#f38ba8",
      "emphasis": "#f5c2e7",
      "strong": "#fab387",
      "keyword": "#cba6f7",
      "function": "#89b4fa",
      "string": "#a6e3a1",
      "number": "#fab387",
      "comment": "#6c7086",
      "math": "#f9e2af",
      "label": "#74c7ec",
      "rawBlock": "#a6adc8",
      "operator": "#89dceb",
      "bracket": "#9399b2"
    }
  }
  ```
- **Application**: Generate a CodeMirror 6 `Extension` (via `EditorView.theme()` and `syntaxHighlighting()`) dynamically from the theme JSON. Theme changes apply instantly without reloading.

### 3. Font Settings

- **Font family**: User can select a font family from a text input. The input should provide suggestions from common monospace fonts.
- **Fallback chain**: Always append fallback fonts: `→ "Fira Code" → "Consolas" → "Liberation Mono" → monospace`.
  - Example: If user selects "JetBrains Mono", the applied CSS is `font-family: "JetBrains Mono", "Fira Code", "Consolas", "Liberation Mono", monospace;`
- **Font size**: Adjustable via number input or Ctrl+`+`/Ctrl+`-`/Ctrl+`0` (zoom). Range: 8–32px, default 14px.
- **Persistence**: Font settings saved in the app settings JSON file.

### 4. PDF Preview

- **Renderer**: Use `pdfjs-dist` to render PDF pages into `<canvas>` elements. Show all pages vertically scrollable.
- **Update modes** (user-configurable in settings):
  - `realtime` — Compile on every keystroke, debounced at 500ms. Default mode.
  - `on_save` — Compile automatically when the file is saved.
  - `manual` — Compile only when the user clicks the compile button in the toolbar.
- **Error handling**: If Typst compilation fails, display the error message as an overlay on the preview panel (do not replace the last successful PDF; keep it visible behind the error).
- **Scroll sync**: (Nice-to-have, not required for v1) Sync scroll position between editor and PDF.

### 5. Typst CLI Integration (Rust Backend)

- **CLI detection**: On app startup, check if `typst` is available in `$PATH`. If not found, show a prominent notification bar at the top of the window with a message: "Typst CLI not found. Please install Typst (https://typst.app) and ensure it's in your PATH." with a link.
- **Compilation command**: `typst compile <input.typ> <output.pdf>`
  - Write a temporary input file if compiling from an unsaved buffer.
  - Output PDF to a temp directory managed by the app.
- **Error parsing**: Parse Typst CLI stderr output to extract error messages, line numbers, and display them in the CompileStatus overlay and StatusBar.
- **Cancellation**: If a new compilation is triggered while one is in progress, kill the previous process before starting a new one.

### 6. File Tree (Left Panel)

- **Folder mode**: User opens a folder → display full recursive directory tree. Files are filterable (show `.typ` files prominently, but don't hide others).
- **Single file mode**: User opens a single `.typ` file → hide the file tree panel entirely, giving the editor more space.
- **Interactions**:
  - Click file → open in editor tab
  - Right-click → context menu: New File, New Folder, Rename, Delete, Reveal in File Manager
  - Drag-and-drop reordering is NOT required.
- **Icons**: Use Vuetify's `mdi` icons to differentiate file types (`.typ`, `.pdf`, `.png`, `.jpg`, folders, etc.).

### 7. Template System

- **Storage**: Templates are stored in the app data directory:
  - `~/.local/share/zenypst/templates/` (Linux)
  - `~/Library/Application Support/zenypst/templates/` (macOS)
  - `%APPDATA%/zenypst/templates/` (Windows)
- **Template format**: Each template is a JSON file:
  ```json
  {
    "name": "Academic Paper",
    "description": "A basic template for academic papers with title, author, and sections.",
    "content": "#set page(paper: \"a4\")\n#set text(font: \"New Computer Modern\", size: 11pt)\n\n= Title\n\n== Introduction\n\n== Methods\n\n== Results\n\n== Conclusion\n"
  }
  ```
- **Built-in templates**: Ship with at least 3:
  - "Blank" (empty file)
  - "Article" (basic article structure)
  - "Letter" (letter format)
- **Template Manager UI** (accessible from Settings or menu):
  - View all templates (built-in + custom) in a list/grid.
  - Create new template: Opens a dialog with name, description, and a CodeMirror editor for the template content.
  - Edit custom templates. Built-in templates can be duplicated but not edited.
  - Delete custom templates (with confirmation dialog).
- **New File flow**: When user creates a new file (Ctrl+N or File → New), a dialog appears:
  1. Select a template (or "Blank").
  2. Choose save location (file dialog) or create as an untitled buffer.
  3. File is created with the template content pre-filled.

### 8. Layout & UI

- **Overall structure**: VSCode-like 3-panel layout. No terminal panel.
  ```
  ┌─────────────────────────────────────────────────────────┐
  │  Toolbar (menu bar)                                     │
  ├────────┬─────────────────────────────┬──────────────────┤
  │        │  Tab Bar                    │                  │
  │  File  │  ┌─────────────────────┐    │  PDF Viewer      │
  │  Tree  │  │                     │    │                  │
  │        │  │  CodeMirror Editor  │    │  (pdf.js canvas) │
  │        │  │                     │    │                  │
  │        │  └─────────────────────┘    │                  │
  ├────────┴─────────────────────────────┴──────────────────┤
  │  Status Bar                                             │
  └─────────────────────────────────────────────────────────┘
  ```
- **Panel resizing**: All three panels (file tree, editor, PDF viewer) are separated by draggable splitters. Use a Vue splitter library (e.g., `splitpanes`) or implement custom drag handles.
- **Default widths**: File tree: 250px, Editor: flexible (fill), PDF viewer: 40% of remaining space.
- **Minimum widths**: File tree: 150px, Editor: 300px, PDF viewer: 200px.
- **Panel collapsing**: Double-click a splitter or use View menu to collapse/expand panels.
- **Color scheme**: Follow Vuetify's dark theme by default (consistent with the editor's dark theme). Support light mode toggle.
- **Toolbar**: Flat menu bar with items: File (New, Open File, Open Folder, Save, Save As), Edit (Undo, Redo, Find, Replace), View (Toggle File Tree, Toggle Preview, Zoom In/Out), Settings (open settings dialog), Help (About, Keyboard Shortcuts).
- **Status bar** (bottom): Show current file path, line:column, Typst compile status (idle / compiling / error count), selected preview mode indicator.

---

## Tauri Commands (Rust ↔ Frontend IPC)

Define the following `#[tauri::command]` functions:

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `read_file` | `path: String` | `Result<String, String>` | Read text file contents |
| `write_file` | `path: String, content: String` | `Result<(), String>` | Write text file |
| `list_directory` | `path: String` | `Result<Vec<FileNode>, String>` | List directory recursively |
| `create_file` | `path: String, content: String` | `Result<(), String>` | Create new file |
| `create_directory` | `path: String` | `Result<(), String>` | Create new directory |
| `delete_path` | `path: String` | `Result<(), String>` | Delete file or directory |
| `rename_path` | `old: String, new: String` | `Result<(), String>` | Rename file/directory |
| `compile_typst` | `input: String, output: String` | `Result<CompileResult, String>` | Run `typst compile` |
| `check_typst_installed` | — | `Result<bool, String>` | Check if `typst` is in PATH |
| `get_typst_version` | — | `Result<String, String>` | Return `typst --version` output |
| `list_templates` | — | `Result<Vec<Template>, String>` | List all templates from app data dir |
| `save_template` | `template: Template` | `Result<(), String>` | Create/update template |
| `delete_template` | `id: String` | `Result<(), String>` | Delete a custom template |
| `list_themes` | — | `Result<Vec<Theme>, String>` | List all themes from app data dir |
| `save_theme` | `theme: Theme` | `Result<(), String>` | Create/update theme |
| `delete_theme` | `id: String` | `Result<(), String>` | Delete a custom theme |
| `load_settings` | — | `Result<AppSettings, String>` | Load settings from app data dir |
| `save_settings` | `settings: AppSettings` | `Result<(), String>` | Persist settings |
| `open_in_file_manager` | `path: String` | `Result<(), String>` | Open path in OS file manager |
| `pick_folder` | — | `Result<Option<String>, String>` | Open native folder picker dialog |
| `pick_file` | `filters: Vec<FileFilter>` | `Result<Option<String>, String>` | Open native file picker dialog |
| `save_file_dialog` | `default_name: String` | `Result<Option<String>, String>` | Open native save dialog |

---

## Settings Persistence

All settings are stored in a single JSON file at the app data directory:
- `~/.local/share/zenypst/settings.json` (Linux)
- `~/Library/Application Support/zenypst/settings.json` (macOS)
- `%APPDATA%/zenypst/settings.json` (Windows)

```typescript
interface AppSettings {
  // Editor
  fontFamily: string;        // default: "Fira Code"
  fontSize: number;          // default: 14
  showLineNumbers: boolean;  // default: true
  wordWrap: boolean;         // default: false

  // Theme
  activeThemeId: string;     // default: "zenypst-dark"

  // Preview
  previewMode: "realtime" | "on_save" | "manual"; // default: "realtime"
  realtimeDebounceMs: number; // default: 500

  // Layout
  fileTreeWidth: number;     // default: 250
  previewWidth: number;      // default: 400 (or percentage)
  fileTreeVisible: boolean;  // default: true
  previewVisible: boolean;   // default: true

  // App
  colorScheme: "dark" | "light"; // default: "dark"
  lastOpenedPath: string | null; // Restore last session
  recentPaths: string[];     // Recent files/folders list
}
```

---

## Implementation Notes

### CodeMirror 6 Setup

```typescript
// src/components/editor/codemirror/setup.ts
import { EditorState } from "@codemirror/state";
import { EditorView, keymap, lineNumbers, highlightActiveLine } from "@codemirror/view";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
import { bracketMatching } from "@codemirror/language";
import { typstLanguage } from "./typst-language";
import { buildThemeExtension } from "./theme";

export function createEditorState(doc: string, themeColors: ThemeColors): EditorState {
  return EditorState.create({
    doc,
    extensions: [
      lineNumbers(),
      highlightActiveLine(),
      history(),
      bracketMatching(),
      highlightSelectionMatches(),
      keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap]),
      typstLanguage(),
      buildThemeExtension(themeColors),
      // ... additional extensions
    ],
  });
}
```

### Typst Language Grammar

If no suitable community grammar exists, define a basic Lezer grammar:

```
// Minimum token coverage:
@top File { (content)* }

content {
  Heading | Emphasis | Strong | MathInline | MathBlock |
  RawInline | RawBlock | Comment | HashExpr | Label | Reference |
  Text
}

Heading { "=" "="* " " line }
Emphasis { "_" content "_" }
Strong { "*" content "*" }
Comment { "//" line }
MathInline { "$" mathContent "$" }
MathBlock { "$ " mathContent " $" }
HashExpr { "#" (Keyword | FunctionCall | Identifier) }
Keyword { "set" | "let" | "show" | "import" | "include" | "if" | "else" | "for" | "while" | "return" }
Label { "<" labelName ">" }
Reference { "@" refName }
```

This is a simplified starting point. Refine incrementally — perfect grammar coverage is not required for v1.

### Build-in Template Seeding

On first launch, if the templates directory is empty, seed it with the built-in templates. Mark them with `"builtIn": true` so the UI disables editing (but allows duplication).

### Typst CLI Error Parsing

Typst CLI outputs errors to stderr in a format like:
```
error: expected expression
  ┌─ input.typ:3:5
  │
3 │     #let x =
  │              ^ expected expression
```

Parse this to extract:
- Severity (`error` / `warning`)
- Message text
- File path, line, and column
- Display in CompileStatus component and optionally annotate the editor gutter.

---

## Coding Conventions

- **Vue components**: Use `<script setup lang="ts">` with Composition API. No Options API.
- **Naming**: PascalCase for components, camelCase for composables and functions, UPPER_SNAKE_CASE for constants.
- **Stores**: Pinia with setup store syntax (`defineStore` with function).
- **Error handling**: All Tauri command invocations must be wrapped in try/catch. Display user-friendly error messages via Vuetify's `v-snackbar`.
- **Async operations**: Use `async/await`, never raw `.then()` chains.
- **Comments**: Add JSDoc comments to exported functions and complex logic. Inline comments for non-obvious behavior.
- **CSS**: Use Vuetify's utility classes and theme system where possible. Scoped `<style>` for component-specific styles. Avoid inline styles.
- **Rust**: Follow standard Rust formatting (`cargo fmt`). Use `thiserror` for error types. Keep Tauri commands thin — delegate to helper modules.

---

## Development Setup

```bash
# Prerequisites
# - Node.js >= 18
# - Rust (stable)
# - Typst CLI (https://github.com/typst/typst)

# Install dependencies
npm install

# Run in development mode (Tauri + Vite dev server)
npm run tauri dev

# Build for production
npm run tauri build
```

---

## Key Dependencies (package.json)

```json
{
  "dependencies": {
    "vue": "^3.5",
    "vuetify": "^3.7",
    "pinia": "^2.2",
    "@codemirror/state": "^6.x",
    "@codemirror/view": "^6.x",
    "@codemirror/language": "^6.x",
    "@codemirror/commands": "^6.x",
    "@codemirror/search": "^6.x",
    "@codemirror/lang-markdown": "^6.x",
    "@lezer/lr": "^1.x",
    "pdfjs-dist": "^4.x",
    "splitpanes": "^3.x",
    "@tauri-apps/api": "^2.x",
    "@tauri-apps/plugin-dialog": "^2.x",
    "@tauri-apps/plugin-fs": "^2.x",
    "@tauri-apps/plugin-shell": "^2.x",
    "@mdi/font": "^7.x"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.x",
    "typescript": "^5.x",
    "vite": "^6.x",
    "@vitejs/plugin-vue": "^5.x",
    "sass": "^1.x"
  }
}
```

---

## Non-Goals (v1)

These are explicitly out of scope for the initial version:

- Typst LSP integration (autocomplete, hover docs, go-to-definition)
- Git integration
- Multi-file project compilation (Typst resolves imports itself, but no project-level build system)
- Plugin/extension system
- Collaborative editing
- Export to formats other than PDF
- Terminal panel
- Scroll sync between editor and PDF
- i18n / multi-language UI (English only for now)
