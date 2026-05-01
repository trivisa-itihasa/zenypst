---
description: dev → main マージ + バージョンタグ push でリリースビルドを起動
argument-hint: <vX.Y.Z>
---

# /release — リリースフロー（dev → main → tag push）

リリースバージョン: `$1`
（必須。`v` プレフィックス + セマンティックバージョン形式。例: `v0.1.0`、`v1.2.3`）

## 事前バリデーション

`$1` が以下を満たすか確認。満たさなければ即座に中止してユーザーに指摘:
- `v` で始まる
- `vMAJOR.MINOR.PATCH`（例: `v0.1.0`）の形式
- 空でない

タグ衝突チェック:
```bash
git tag -l "$1"
```
既に存在すればユーザーに上書きするか別バージョンか確認。

## 手順

1. **現在の状態を確認** — 並列で:
   - `git status`（ワーキングツリーが clean か）
   - `git branch --show-current`
   - `git log --oneline origin/main..dev`（main に対して dev が何コミット先行しているか）

2. **未コミットの変更があれば中止** — リリース前にすべて `dev` に push 済みであることを要求

3. **dev をリモートと同期**:
   ```bash
   git checkout dev
   git pull origin dev
   ```

4. **バージョンを更新** — `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` の3ファイルを `$1` に合わせる:
   ```bash
   VERSION="${1#v}"
   npm version --no-git-tag-version "$VERSION"
   python3 -c "
   import json
   with open('src-tauri/tauri.conf.json', 'r') as f:
       data = json.load(f)
   data['version'] = '$VERSION'
   with open('src-tauri/tauri.conf.json', 'w') as f:
       json.dump(data, f, indent=2)
       f.write('\n')
   "
   python3 -c "
   import re
   with open('src-tauri/Cargo.toml', 'r') as f:
       content = f.read()
   content = re.sub(r'^(version\s*=\s*)\".*?\"', r'\1\"$VERSION\"', content, count=1, flags=re.MULTILINE)
   with open('src-tauri/Cargo.toml', 'w') as f:
       f.write(content)
   "
   git add package.json package-lock.json src-tauri/tauri.conf.json src-tauri/Cargo.toml
   git commit -m "Bump version to $1"
   git -c include.path=/home/gecko/.gitconfig_local push origin dev
   ```

5. **main に切り替えて最新化**:
   ```bash
   git checkout main
   git pull origin main
   ```

6. **dev をマージ（履歴保持のため --no-ff）**:
   ```bash
   git merge --no-ff dev -m "Release $1"
   ```
   コンフリクトしたらユーザーに報告して中止（自動解決しない）

7. **タグを作成**:
   ```bash
   git tag -a "$1" -m "Release $1"
   ```

8. **main とタグを同時 push**:
   ```bash
   git -c include.path=/home/gecko/.gitconfig_local push --follow-tags origin main
   ```

9. **dev を main に同期**（マージコミットを取り込み、次の開発で divergence を起こさない）:
   ```bash
   git checkout dev
   git merge main
   git -c include.path=/home/gecko/.gitconfig_local push origin dev
   ```

10. **完了報告**:
    - リリースタグ名
    - 「タグ push をトリガに GitHub Actions のビルドが起動した。`https://github.com/trivisa-itihasa/zenypst/actions` で進捗を確認できる」
    - 「ビルド完了後、GitHub が自動生成したリリースノートを含む Release が公開される。`https://github.com/trivisa-itihasa/zenypst/releases` で内容を確認できる」

## 注意

- 各 step が失敗したら、それ以降の step は実行しない（特に push 系）
- `git push --force` は絶対に使わない
- マージコンフリクトを勝手に解決しない — ユーザーに渡す
