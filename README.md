# Zenypst

Zenypst is a lightweight and simple Typst editor with a built-in Typst compiler. You can start writing and previewing Typst documents right away without installing any additional compiler.

## Features

- **Built-in Typst compiler**  
  Typst is embedded, so no separate CLI installation or PATH configuration is required. The document is compiled automatically on save or in real time, and the PDF preview is updated instantly.

- **Live PDF preview**  
  Changes in the editor are reflected in the PDF immediately. Clicking on text in the preview panel jumps to the corresponding source location in the editor.

- **Typst-focused editor**  
  Powered by CodeMirror 6 with Typst syntax highlighting. It provides a comfortable writing environment with multiple tabs, line numbers, font settings, and keyboard shortcuts.

- **File tree**  
  Browse the folder structure in the left panel while you work. You can create, rename, delete files and folders, and reveal them in the system file manager.

- **Template system**  
  In addition to built-in templates, you can create and manage your own. You can also search templates from [Typst Universe](https://typst.app/universe) and start a new project with one click.

- **Customizable themes**  
  Switch the colors of both the editor and the UI with a unified theme. Built-in themes are included, and you can create and edit your own.

- **PDF export**  
  Export your document as a PDF to any location.

- **Internationalization**  
  The UI uses an i18n framework, supporting multiple languages.

## Installation

Download the binary for your platform (Windows, macOS, or Linux) from the [Releases](https://github.com/trivisa-itihasa/zenypst/releases) page.

## Development

### Prerequisites

- Node.js
- Rust (stable)

### Setup

```bash
npm install
```

### Run the development server

```bash
./dev.sh
```

or

```bash
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Vite, Quasar, CodeMirror 6, pdf.js
- **Desktop runtime**: Tauri v2 (Rust)
- **Built-in compiler**: typst (Rust crate)

## License

Zenypst is distributed under the MIT License.  
See [LICENSE](LICENSE) for details.

This application bundles third-party open-source software.  
Their respective licenses are included in the distributed binaries or can be found in their source repositories.
