# ws new

worktree を作成します。

## 使い方

```bash
ws new [name] [options]
```

## 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `name` | いいえ | ワークスペース名。省略するとランダムな名前を自動生成（例: `gentle-happy-fox`） |

## オプション

| オプション | 短縮 | 説明 |
|-----------|------|------|
| `--directory <path>` | `-d` | worktree を作成するパス（デフォルト: `../<name>` または `<name>`） |
| `--branch <branch>` | | ブランチ名を明示的に指定（デフォルト: name と同じ） |
| `--from <ref>` | | 新規ブランチの起点（デフォルト: HEAD） |

## 動作

1. 同名のブランチが既に存在する場合は、そのブランチをチェックアウトして worktree を作成
2. ブランチが存在しない場合は、`--from` で指定した起点（デフォルト: HEAD）から新規ブランチを作成
3. HEAD が無効（空の bare リポジトリ等）かつ `--from` 未指定の場合は、orphan ブランチで作成
4. 共有ストア（store）が存在する場合、追跡ファイルを自動配布

### worktree の作成先

- **bare 構成**（`.bare/` が存在）: カレントディレクトリ直下に `<name>/` を作成
- **通常構成**（`.git/` 内で実行）: 親ディレクトリに `../<name>` を作成

`-d` オプションで任意のパスに変更できます。

## 例

```bash
# 基本的な worktree 作成
ws new main

# main ブランチから分岐
ws new feature/awesome --from main

# ブランチ名とディレクトリを明示指定
ws new my-work --branch feature/my-work -d ../my-work-dir

# ランダムな名前で作成
ws new
```
