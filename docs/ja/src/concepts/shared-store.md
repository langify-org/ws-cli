# 共有ストア

## 概要

共有ストア（shared store）は、worktree 間で gitignored ファイルを一元管理する仕組みです。`.envrc`, `.mcp.json`, `.env.local` など git 管理外のファイルを store に登録しておくことで、新しい worktree 作成時に自動的に配布されます。

## store の構造

store は `<git-common-dir>/worktree-store/` に作成されます。bare 構成の場合は `.bare/worktree-store/` です。

```
.bare/worktree-store/
├── manifest         # "strategy:filepath" の行形式
├── .mcp.json        # マスターコピー
├── .envrc           # マスターコピー
└── .env.local       # マスターコピー
```

### manifest

manifest はテキストファイルで、追跡するファイルとその strategy を1行ずつ記録します。

```
symlink:.envrc
symlink:.mcp.json
copy:.env.local
```

## strategy

共有ストアは2つの strategy（配布戦略）をサポートしています。

### symlink

store 内のファイルへのシンボリックリンクを worktree に作成します。

```bash
ws store track -s symlink .envrc
```

- **全 worktree で同じ内容を共有** — store のファイルを編集すると全 worktree に反映される
- `track` 実行時に既存ファイルは store に移動され、シンボリックリンクに置き換えられる

**用途:** `.envrc`, `.mcp.json` など、全 worktree で共通の設定ファイル

### copy

store からファイルを worktree にコピーします。

```bash
ws store track -s copy .env.local
```

- **worktree ごとにカスタマイズ可能** — コピー後は各 worktree で独立して編集できる
- `ws store push` で変更を store に書き戻し、`ws store pull` で store から取得

**用途:** `.env.local` など、worktree ごとに異なる値が必要なファイル

### strategy の比較

| | symlink | copy |
|---|---------|------|
| 配布方法 | シンボリックリンク | ファイルコピー |
| 内容の共有 | 全 worktree で同一 | worktree ごとに独立 |
| 更新の反映 | 即座（リンク先が同じ） | `push` / `pull` が必要 |
| 用途 | 共通の設定ファイル | 環境ごとに異なるファイル |

## ワークフロー

### 初回セットアップ

worktree 内で追跡したいファイルを登録します。

```bash
ws store track -s symlink .envrc
ws store track -s symlink .mcp.json
ws store track -s copy .env.local
```

初回の `ws store track` 実行時に store が自動的に初期化されます。

### 新しい worktree の作成

`ws new` 実行時に store から自動的にファイルが配布されます。

```bash
ws new feature/bar
# → store から .envrc (symlink), .mcp.json (symlink), .env.local (copy) が配布される
```

### 状態の確認

```bash
ws store status
```

各ファイルの状態を表示します:

| ステータス | 意味 |
|-----------|------|
| `OK` | 正常 |
| `MISSING` | worktree にファイルがない |
| `MISSING(store)` | store にファイルがない |
| `MODIFIED` | copy ファイルが store と異なる |
| `NOT_LINK` | symlink であるべきファイルが通常ファイル |
| `WRONG_LINK` | symlink のリンク先が store と異なる |

### copy ファイルの同期

```bash
# worktree の変更を store に反映
ws store push
ws store push .env.local          # 特定ファイルのみ

# store から worktree に配布
ws store pull
ws store pull .envrc              # 特定ファイルのみ
ws store pull -f                  # 既存ファイルを上書き
```

### ファイルの追跡解除

ファイルの追跡を停止し、store から削除するには:

```bash
ws store untrack .envrc            # シンボリックリンクは実ファイルに復元される
ws store untrack .env.local        # copy ファイルは worktree にそのまま残る
```
