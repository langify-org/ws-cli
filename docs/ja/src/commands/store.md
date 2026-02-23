# ws store

worktree 間で共有したい gitignored ファイルを一元管理します。

## サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| [`ws store track`](#ws-store-track) | ファイルを store に登録 |
| [`ws store status`](#ws-store-status) | 共有ファイルの状態表示 |
| [`ws store push`](#ws-store-push) | copy ファイルの変更を store に反映 |
| [`ws store pull`](#ws-store-pull) | store から追跡ファイルを配布 |
| [`ws store untrack`](#ws-store-untrack) | ファイルを store から登録解除 |

共有ストアの詳しい仕組みについては[共有ストア](../concepts/shared-store.md)を参照してください。

---

## ws store track

ファイルを store に登録して追跡を開始します。

### 使い方

```bash
ws store track -s <strategy> <file>
```

### 引数・オプション

| 名前 | 必須 | 説明 |
|------|------|------|
| `file` | はい | 追跡するファイルパス |
| `-s <strategy>` | はい | `symlink` または `copy` |

### 動作

1. ファイルを store にコピー
2. manifest に `strategy:filepath` を追記
3. strategy が `symlink` の場合、元ファイルを削除して store へのシンボリックリンクに置換

### 例

```bash
ws store track -s symlink .claude/settings.local.json
ws store track -s copy .env
```

---

## ws store status

共有ファイルの状態を一覧表示します。

### 使い方

```bash
ws store status
```

### 出力例

```
Store: ~/my-project/.bare/worktree-store

STRATEGY  FILE        STATUS
────────  ────        ──────
symlink   .claude/settings.local.json  OK
copy      .env                        MODIFIED
```

---

## ws store push

copy strategy で追跡しているファイルの変更を store に反映します。

### 使い方

```bash
ws store push [file]
```

### 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `file` | いいえ | ファイルパス。省略すると全 copy ファイルを対象 |

### 例

```bash
ws store push              # 全 copy ファイルを push
ws store push .env.local   # 特定ファイルのみ
```

---

## ws store pull

store から追跡ファイルを現在の worktree に配布します。

### 使い方

```bash
ws store pull [file] [-f]
```

### 引数・オプション

| 名前 | 必須 | 説明 |
|------|------|------|
| `file` | いいえ | ファイルパス。省略すると全追跡ファイルを対象 |
| `-f` | いいえ | 既存ファイルを上書きして配布 |

### 動作

- symlink ファイル: store へのシンボリックリンクを作成
- copy ファイル: store からファイルをコピー
- 既存ファイルがある場合はスキップ（`-f` で上書き）

### 例

```bash
ws store pull              # 全追跡ファイルを pull
ws store pull .env         # 特定ファイルのみ
ws store pull -f           # 既存ファイルを上書き
```

---

## ws store untrack

ファイルを store から登録解除し、追跡を停止します。

### 使い方

```bash
ws store untrack <file>
```

### 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `file` | はい | 登録解除するファイルパス |

### 動作

1. `symlink` strategy の場合、全 worktree のシンボリックリンクを実ファイルに復元（store からコピー）
2. manifest からエントリを削除
3. store のマスターコピーを削除

### 例

```bash
ws store untrack .claude/settings.local.json
ws store untrack .env
```
