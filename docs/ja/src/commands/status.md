# ws status

リポジトリ、現在のワークスペース、共有ファイルを統合表示するステータスダッシュボードです。

## 使い方

```bash
ws status
```

## 動作

コンテキストに応じて最大3つのセクションを表示します:

### Repositories セクション

`~/.config/ws/config.toml` にリポジトリが登録されている場合に表示。各リポジトリのパス、GIT_DIR タイプ、worktree ツリーを一覧します。現在のリポジトリ（いる場合）には `*` マーカーが付きます。

### Current workspace セクション

worktree 内で実行した場合に表示。現在の worktree のパス、ブランチ、追跡ファイル数を表示します。

### Shared files セクション

共有ストアが存在し、追跡ファイルがある場合に表示。各ファイルの strategy と状態を一覧します。

リポジトリ外で登録済みリポジトリもない場合は、「登録済みリポジトリはありません」メッセージが表示されます。

## 出力例

```
Repositories:
  web
    Path: /Users/user/projects/web
    GIT_DIR: .bare
    Worktrees:
      └── release   [release] 9946e77

* my-project
    Path: /Users/user/projects/my-project
    GIT_DIR: .bare
    Worktrees:
      ├── main   [main] abc1234
      └── feature-foo   [feature/foo] def5678

Current workspace:
  * /Users/user/projects/my-project/main [main]  [3 files tracked]

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
