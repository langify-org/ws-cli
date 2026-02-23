# ws status

リポジトリ、現在のリポジトリ、現在のワークスペースを統合表示するステータスダッシュボードです。

## 使い方

```bash
ws status
```

## 動作

コンテキストに応じて最大3つのセクションを表示します:

### Repositories セクション

`~/.config/ws/config.toml` にリポジトリが登録されている場合に表示。各リポジトリの名前、パス（`~` で短縮表示）、タイプ（`bare` / `git`）をテーブル形式で一覧します。現在のリポジトリ（いる場合）には `*` マーカーが付きます。

### Current Repository セクション

git リポジトリ内で実行した場合に表示（config に未登録でも表示）。リポジトリ名、パス、全 worktree のツリー表示を含みます。現在の worktree には `*` マーカーが付きます。

### Current Workspace セクション

共有ストアが存在し追跡ファイルがある worktree 内で実行した場合に表示。各ファイルの strategy と状態をテーブル形式で一覧します。

リポジトリ外で登録済みリポジトリもない場合は、「登録済みリポジトリはありません」メッセージが表示されます。

## カラー出力

ターミナル接続時は出力がカラー表示されます。パイプやファイルへのリダイレクト時は自動的にカラーコードが除去されます。`NO_COLOR` 環境変数でカラーを無効化できます。

| 要素 | スタイル |
|------|---------|
| セクションヘッダーのタイトル | Bold |
| セクションヘッダーの罫線 (`──`) | Dim |
| テーブルヘッダー | Bold |
| テーブルセパレータ (`────`) | Dim |
| ステータス `OK` | Green |
| ステータス `MISSING`, `MISSING(store)` | Red |
| ステータス `ERROR` | Red + Bold |
| ステータス `MODIFIED`, `NOT_LINK`, `WRONG_LINK` | Yellow |
| カレントマーカー `*` | Green + Bold |
| ブランチ名 `[branch]` | Cyan |
| コミットハッシュ | Dim |
| リポジトリ種別 `bare` | Cyan |
| リポジトリ種別 `NOT_FOUND` | Red |

## 出力例

```
── Repositories ──────────────────────────
  NAME              PATH                                     TYPE
  ────              ────                                     ────
  langify-notebook  ~/Projects/langify-org/langify-notebook  git
  web               ~/Projects/spirinc/web                   bare
* ws-cli            ~/Projects/langify-org/ws-cli            bare

── Current Repository: ws-cli ────────────
  Path: ~/Projects/langify-org/ws-cli
  Worktrees:
    ├──   fix-ci    [fix-ci] fb7eff8
    └── * master    [master] 5b33080

── Current Workspace: master [master] ────
  STRATEGY  FILE       STATUS
  ────────  ────       ──────
  symlink   .mcp.json  OK
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
