# CLAUDE.md — Zenypst

Full product specification is in [SPEC.md](./SPEC.md).

**SPEC.md の扱い**: ユーザーの指示が SPEC.md の内容と異なる場合は、指示に従って実装したうえで SPEC.md を書き直すこと。SPEC.md は現時点の仕様の記録であり、指示が優先される。

---

## Development Environment

`dev.sh` supports both macOS and WSL2/Linux and detects the OS automatically.

### macOS
- **Node.js**: via nvm (`~/.nvm/nvm.sh`) or system install
- **Rust/cargo**: install via `rustup` — `dev.sh` adds `~/.cargo/bin` to PATH automatically
- No GTK/WebKit libraries needed — Tauri uses the system WKWebView

### WSL2 (Ubuntu 24.04) on Windows 11
- **Node.js**: managed via nvm — always source before running npm commands:
  ```bash
  source ~/.nvm/nvm.sh  # or open a new shell (nvm is in ~/.zshrc)
  ```
- **Rust/cargo**: must be installed inside WSL (`rustup`), not the Windows version
- **No `sudo` access** in this environment — install tools to `~/.local/bin`
- **GUI (WSLg)**: set these env vars to avoid slow WebKit startup due to Mesa GPU fallback:
  ```bash
  export WEBKIT_DISABLE_DMABUF_RENDERER=1
  export WEBKIT_DISABLE_COMPOSITING_MODE=1
  export LIBGL_ALWAYS_SOFTWARE=1
  ```
  `dev.sh` sets these automatically on Linux.

## Running the App

```bash
./dev.sh          # Tauri + Vite (recommended)
npm run dev       # Vite only (no Tauri IPC, for UI work from browser)
```

## Git & GitHub

- Remote: `https://github.com/trivisa-itihasa/zenypst.git`
- Default branch: `main`
- Push using token stored in `~/.gitconfig_local` (URL rewrite):
  ```bash
  git -c include.path=/home/gecko/.gitconfig_local push origin main
  ```
- Multi-platform CI builds via GitHub Actions (`.github/workflows/build.yml`) — Windows/macOS/Ubuntu artifacts published as draft Releases

---

## Non-Obvious Gotchas

### Tauri does not inherit shell PATH
`typst` won't be found via `Command::new("typst")` because Tauri spawns without `~/.zshrc`. `typst_command_with_path()` in `commands/typst.rs` explicitly searches `~/.cargo/bin`, `~/.local/bin`, and extends `PATH` before invoking the process.

### Rust → TypeScript serialization must be camelCase
All Tauri command return structs need `#[serde(rename_all = "camelCase")]`. Without it, snake_case fields (e.g. `pdf_path`) arrive as-is in JS and TypeScript's camelCase access (e.g. `pdfPath`) returns `undefined`.

### Browse button disabled outside Tauri
`TypstSettings.vue` checks `window.__TAURI_INTERNALS__` to detect if running inside the desktop app. The Browse button and Check version button are hidden/disabled in the browser (e.g. when using `npm run dev`).

### ICO file must use embedded PNG format
`src-tauri/icons/icon.ico` is a Vista-style ICO (PNG-in-ICO). If regenerating, use Python's struct to write the ICO header manually — do not use tools that produce 8-bit/RGB PNG inside ICO, as Tauri's `generate_context!()` macro rejects non-RGBA PNG data.

---

## Coding Conventions

- Vue: `<script setup lang="ts">`, Composition API only, no Options API
- Naming: PascalCase components, camelCase composables/functions, UPPER_SNAKE_CASE constants
- Stores: Pinia setup-store syntax (`defineStore` with arrow function)
- Tauri calls: always `try/catch`; show errors via `v-snackbar`
- Async: `async/await` only, no `.then()` chains
- CSS: Quasar utility classes + scoped `<style>`; avoid inline styles
- Rust: `cargo fmt`; keep `#[tauri::command]` functions thin, delegate to helper modules
