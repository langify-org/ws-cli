# ws shared

worktree 間で共有したい gitignored ファイルを一元管理します。

## サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| [`ws shared track`](#ws-shared-track) | ファイルを store に登録 |
| [`ws shared status`](#ws-shared-status) | 共有ファイルの状態表示 |
| [`ws shared push`](#ws-shared-push) | copy ファイルの変更を store に反映 |
| [`ws shared pull`](#ws-shared-pull) | store から追跡ファイルを配布 |

共有ストアの詳しい仕組みについては[共有ストア](../concepts/shared-store.md)を参照してください。

---

## ws shared track

ファイルを store に登録して追跡を開始します。

### 使い方

```bash
ws shared track -s <strategy> <file>
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
ws shared track -s symlink .envrc
ws shared track -s copy .env.local
```

---

## ws shared status

共有ファイルの状態を一覧表示します。

### 使い方

```bash
ws shared status
```

### 出力例

```
Store: /Users/user/my-project/.bare/worktree-store

STRATEGY FILE                                     STATUS
-------- ---------------------------------------- ----------
symlink  .envrc                                   OK
symlink  .mcp.json                                OK
copy     .env.local                               MODIFIED
```

---

## ws shared push

copy strategy で追跡しているファイルの変更を store に反映します。

### 使い方

```bash
ws shared push [file]
```

### 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `file` | いいえ | ファイルパス。省略すると全 copy ファイルを対象 |

### 例

```bash
ws shared push              # 全 copy ファイルを push
ws shared push .env.local   # 特定ファイルのみ
```

---

## ws shared pull

store から追跡ファイルを現在の worktree に配布します。

### 使い方

```bash
ws shared pull [file] [-f]
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
ws shared pull              # 全追跡ファイルを pull
ws shared pull .envrc       # 特定ファイルのみ
ws shared pull -f           # 既存ファイルを上書き
```
