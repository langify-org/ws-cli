# ws clone

bare リポジトリを作成します。

## 使い方

```bash
ws clone [url]
```

## 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `url` | いいえ | リモート URL。省略すると空の bare リポジトリを作成 |

## 動作

カレントディレクトリに `.bare/` ディレクトリを作成します。

- URL を指定した場合: `git clone --bare <url> .bare` を実行し、デフォルトブランチ（例: `main` や `master`）の worktree を自動作成
- URL を省略した場合: `git init --bare .bare` を実行（コミットが存在しないため worktree は作成されない）

`.bare` が既に存在する場合はエラーになります。

## 例

### リモートリポジトリを bare clone

```bash
mkdir my-project && cd my-project
ws clone https://github.com/example/repo.git
# .bare/ が作成され、デフォルトブランチの worktree が自動的にセットアップされる
```

### 空の bare リポジトリを作成

```bash
mkdir my-project && cd my-project
ws clone
ws new master    # orphan ブランチで worktree を作成
```
