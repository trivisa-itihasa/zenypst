#!/usr/bin/env bash
# dev.sh — Zenypst 開発サーバー起動スクリプト
# フロントエンド (Vite) と Tauri バックエンドを一括起動します。

set -euo pipefail

# ── 色定義 ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

log()  { echo -e "${CYAN}[zenypst]${RESET} $*"; }
ok()   { echo -e "${GREEN}[zenypst]${RESET} $*"; }
warn() { echo -e "${YELLOW}[zenypst]${RESET} $*"; }
err()  { echo -e "${RED}[zenypst]${RESET} $*" >&2; }

# ── スクリプトのディレクトリに移動 ──────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── WSL/Mesa GPU フォールバック回避 ─────────────────────────────────────────
# WSL2 では GPU アクセスに失敗して WebKit の起動が遅くなるため、
# 最初からソフトウェアレンダリングを使うよう指定する。
export WEBKIT_DISABLE_DMABUF_RENDERER=1
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export LIBGL_ALWAYS_SOFTWARE=1

echo ""
echo -e "${BOLD}  Zenypst 開発サーバー${RESET}"
echo "  ─────────────────────────────────────"
echo ""

# ── nvm のロード ─────────────────────────────────────────────────────────────
if [ -f "$HOME/.nvm/nvm.sh" ]; then
  # shellcheck source=/dev/null
  . "$HOME/.nvm/nvm.sh" 2>/dev/null || true
fi

# ── Node.js の確認 ───────────────────────────────────────────────────────────
if ! command -v node &>/dev/null; then
  err "Node.js が見つかりません。"
  err "インストール: curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash"
  exit 1
fi
log "Node.js $(node --version)"

# ── npm install (node_modules がなければ自動実行) ──────────────────────────
if [ ! -d "node_modules" ]; then
  log "node_modules が見つかりません。npm install を実行します…"
  npm install
fi

# ── Tauri フルスタック起動の可否チェック ────────────────────────────────────
TAURI_READY=true
MISSING=()

# cargo の確認
if ! command -v cargo &>/dev/null; then
  export PATH="$HOME/.cargo/bin:$PATH"
  if ! command -v cargo &>/dev/null; then
    TAURI_READY=false
    MISSING+=("Rust/cargo (https://rustup.rs)")
  fi
fi

# pkg-config の確認
if ! command -v pkg-config &>/dev/null; then
  TAURI_READY=false
  MISSING+=("pkg-config (sudo apt install pkg-config)")
fi

# GTK/WebKit 開発ライブラリの確認
if command -v pkg-config &>/dev/null; then
  for lib in "gtk+-3.0" "webkit2gtk-4.1"; do
    if ! pkg-config --exists "$lib" 2>/dev/null; then
      TAURI_READY=false
      case "$lib" in
        "gtk+-3.0")    MISSING+=("libgtk-3-dev (sudo apt install libgtk-3-dev)") ;;
        "webkit2gtk-4.1") MISSING+=("libwebkit2gtk-4.1-dev (sudo apt install libwebkit2gtk-4.1-dev)") ;;
      esac
    fi
  done
fi

# ── 使用中ポートの解放 ──────────────────────────────────────────────────────
DEV_PORT=1420
if lsof -ti :"$DEV_PORT" &>/dev/null; then
  warn "ポート $DEV_PORT が使用中です。既存プロセスを終了します…"
  lsof -ti :"$DEV_PORT" | xargs kill -9 2>/dev/null || true
  sleep 1
  ok "ポート $DEV_PORT を解放しました。"
fi

# ── 子プロセス管理 ──────────────────────────────────────────────────────────
PIDS=()

cleanup() {
  echo ""
  log "シャットダウン中…"
  for pid in "${PIDS[@]}"; do
    if kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
    fi
  done
  ok "終了しました。"
}
trap cleanup EXIT INT TERM

# ── 起動 ────────────────────────────────────────────────────────────────────
if $TAURI_READY; then
  ok "全依存関係が揃っています。Tauri フルスタックで起動します。"
  echo ""
  npm run tauri:dev &
  PIDS+=($!)
  log "Tauri dev PID: ${PIDS[-1]}"
else
  warn "以下の依存関係が不足しているため、フロントエンドのみ起動します:"
  for m in "${MISSING[@]}"; do
    warn "  • $m"
  done
  echo ""

  # まとめて apt install コマンドを提示
  if [[ " ${MISSING[*]} " == *"apt install"* ]]; then
    echo -e "  ${BOLD}システムライブラリを一括インストールするには:${RESET}"
    echo -e "  ${CYAN}sudo apt install -y pkg-config libgtk-3-dev libwebkit2gtk-4.1-dev \\"
    echo -e "    libssl-dev libayatana-appindicator3-dev librsvg2-dev \\"
    echo -e "    libglib2.0-dev libcairo2-dev libpango1.0-dev${RESET}"
    echo ""
  fi

  ok "フロントエンド起動中: http://localhost:1420"
  echo ""
  npm run dev &
  PIDS+=($!)
  log "Vite dev PID: ${PIDS[-1]}"
fi

echo ""
log "停止するには Ctrl+C を押してください。"
echo ""

wait "${PIDS[@]}"
