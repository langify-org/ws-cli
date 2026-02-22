# ws status

worktree 一覧と共有ファイルの状態を統合表示します。

## 使い方

```bash
ws status
```

## 動作

以下の2つのセクションを表示します:

### Workspaces セクション

全 worktree の一覧を表示します。現在の worktree には `*` マーカーが付きます。共有ストアが存在する場合、追跡ファイル数も表示されます。

### Shared files セクション

共有ストアが存在し、追跡ファイルがある場合に表示されます。各ファイルの strategy と状態を一覧します。

## 出力例

```
Workspaces:
  * /Users/user/my-project/main              [main]  [3 files tracked]
    /Users/user/my-project/feature-foo       [feature/foo]  [3 files tracked]

Shared files:
  STRATEGY FILE                                     STATUS
  -------- ---------------------------------------- ----------
  symlink  .envrc                                   OK
  symlink  .mcp.json                                OK
  copy     .env.local                               MODIFIED
```

## ステータスの意味

| ステータス | 意味 |
|-----------|------|
| `OK` | 正常 |
| `MISSING` | worktree にファイルがない |
| `MISSING(store)` | store にファイルがない |
| `MODIFIED` | copy ファイルの内容が store と異なる |
| `NOT_LINK` | symlink であるべきファイルが通常ファイルになっている |
| `WRONG_LINK` | symlink のリンク先が store のファイルと異なる |
