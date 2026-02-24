# ws open

登録済みリポジトリの worktree をエディタで開きます。

## 使い方

```bash
ws open <repository> <worktree> [options]
```

## 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `repository` | はい | 登録済みリポジトリの名前（`ws repos list` で表示される名前） |
| `worktree` | はい | worktree 名（リポジトリルートからの相対パス） |

## オプション

| オプション | 説明 |
|-----------|------|
| `--editor <command>` | 使用するエディタコマンド（デフォルト: `$VISUAL` または `$EDITOR`） |

## エディタの解決順序

エディタは以下の優先順位で解決されます:

1. `--editor` フラグ（最優先）
2. `$VISUAL` 環境変数
3. `$EDITOR` 環境変数
4. いずれも未設定の場合はエラー

## 例

```bash
# "my-repo" の "main" worktree をデフォルトエディタで開く
ws open my-repo main

# エディタを指定して開く
ws open my-repo feature/awesome --editor code

# $EDITOR を使用
EDITOR=vim ws open my-repo main
```

## インタラクティブモード

インタラクティブモード（`ws i`）では、`open` コマンドをガイド付きワークフローで利用できます:

1. 登録済みリポジトリの一覧からリポジトリを選択
2. リポジトリの worktree 一覧から worktree を選択
3. 解決されたエディタで worktree が開かれる
