---
description: dev ブランチへ変更をコミット & push する通常開発フロー
argument-hint: [commit message]
---

# /push-dev — 通常開発フロー（dev ブランチへ push）

引数で渡されたコミットメッセージ: `$ARGUMENTS`
（空の場合は変更内容から適切なメッセージを生成すること）

## 手順

1. **現在の状態を確認** — 並列で以下を実行:
   - `git status`
   - `git diff`
   - `git branch --show-current`
   - `git log --oneline -5`

2. **dev ブランチに切り替え（必要なら）**:
   - 現ブランチが `dev` でなければ `git checkout dev` を実行
   - 切り替え前に未コミットの変更がある場合はユーザーに確認（stash するか、現ブランチでコミットするか）

3. **dev を最新化**:
   ```bash
   git pull origin dev
   ```
   （リモートに `dev` がまだ無い場合は失敗してよい — その旨ユーザーに報告）

4. **変更をステージしてコミット**:
   - `git add` は機密ファイル（`.env` 等）を含めないよう個別パスで指定
   - コミットメッセージは引数優先、無ければ diff から生成（"why" を 1 行）
   - HEREDOC で Co-Authored-By を含める

5. **dev へ push**:
   ```bash
   git -c include.path=/home/gecko/.gitconfig_local push origin dev
   ```

6. **完了報告** — push 後の `git status` と最新コミット 1 行を表示

## 注意

- `main` には絶対に push しない（リリース時のみ `/release` を使う）
- CI ビルドは走らない（`v*` タグ push 時のみ起動）。これは想定動作なのでユーザーに伝える必要はない
- pre-commit hook が落ちたら `--amend` ではなく新規コミットで再試行
