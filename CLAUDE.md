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
- Branch strategy:
  - `main` — release branch. Always represents the latest released build.
  - `dev` — development branch. **デフォルトのpush先はここ。** ユーザーから明示的に `main` への push を指示されない限り、コミット・push はすべて `dev` に対して行う。
- Push 認証: トークンは `~/.gitconfig_local` の URL 書き換えルールで保持。push 時は `-c include.path=...` を付与する。
- CI: GitHub Actions (`.github/workflows/build.yml`) は `v*` タグの push もしくは `workflow_dispatch` でのみ起動。Windows/macOS/Ubuntu 用成果物が draft Release として公開される。通常の `dev`/`main` への push ではビルドは走らない。

### Claude Code カスタムコマンド

以下の手順は `.claude/commands/` にスラッシュコマンドとして登録済み。Claude Code セッション内では下記コマンドで実行できる:

- `/push-dev [commit message]` — 通常開発フロー（下記）を一括実行
- `/release vX.Y.Z` — リリースフロー（下記）を一括実行（`vX.Y.Z` 必須）

### 通常開発: dev への push

```bash
# 作業前: dev に切り替えて最新化
git checkout dev
git pull origin dev

# 変更をコミット
git add <files>
git commit -m "..."

# push（dev へ）
git -c include.path=/home/gecko/.gitconfig_local push origin dev
```

### リリース: dev → main マージしてタグ push

```bash
# main を最新化
git checkout main
git pull origin main

# dev をマージ（履歴を残すため --no-ff 推奨）
git merge --no-ff dev

# バージョンタグを打つ（vX.Y.Z 形式必須 — CI のトリガ条件）
git tag -a v0.1.0 -m "Release v0.1.0"

# main とタグを同時に push（--follow-tags でタグも一緒に送る）
git -c include.path=/home/gecko/.gitconfig_local push --follow-tags origin main

# 戻って dev で作業継続
git checkout dev
git merge main   # main にあるマージコミットを dev に同期
```

タグ push がトリガとなり、CI が走り、3 プラットフォーム分の成果物が GitHub の draft Release として作られる。リリースノートを書いて publish すれば配布完了。

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
