# ws repos

リポジトリの登録を管理します。登録情報は `~/.config/ws/config.toml` に保存されます。

> [!TIP]
> Home Manager を使用している場合、`ws repos add` の代わりに `programs.ws.repos` でリポジトリを宣言的に管理できます。詳しくは[インストール](../getting-started/installation.md)を参照してください。

## サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| [`ws repos clone`](#ws-repos-clone) | bare リポジトリを作成 |
| [`ws repos add`](#ws-repos-add) | リポジトリを登録 |
| [`ws repos list`](#ws-repos-list) | 登録済みリポジトリの一覧表示 |
| [`ws repos rm`](#ws-repos-rm) | リポジトリの登録解除 |

---

## ws repos clone

bare リポジトリを作成します。

### 使い方

```bash
ws repos clone [url]
```

### 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `url` | いいえ | リモート URL。省略すると空の bare リポジトリを作成 |

### 動作

カレントディレクトリに `.bare/` ディレクトリを作成します。

- URL を指定した場合: `git clone --bare <url> .bare` を実行し、デフォルトブランチ（例: `main` や `master`）の worktree を自動作成
- URL を省略した場合: `git init --bare .bare` を実行（コミットが存在しないため worktree は作成されない）

`.bare` が既に存在する場合はエラーになります。リポジトリは config に自動登録されます。

### 例

```bash
mkdir my-project && cd my-project
ws repos clone https://github.com/example/repo.git
# .bare/ が作成され、デフォルトブランチの worktree が自動的にセットアップされる
```

```bash
mkdir my-project && cd my-project
ws repos clone                  # 空の bare リポジトリを作成
ws new master                   # orphan ブランチで worktree を作成
```

---

## ws repos add

リポジトリをレジストリに登録します。

### 使い方

```bash
ws repos add [path] [--name <name>]
```

### 引数・オプション

| 名前 | 必須 | 説明 |
|------|------|------|
| `path` | いいえ | リポジトリのパス。省略するとカレントディレクトリ |
| `--name <name>` | いいえ | 表示名。省略するとディレクトリ名 |

### 動作

1. パスを[リポジトリルート](../concepts/bare-worktree.md#リポジトリルートの解決)に解決（任意の worktree やサブディレクトリから実行可能）
2. パスを検証（git リポジトリである必要あり）
3. `origin` からリモート URL を自動検出
4. `~/.config/ws/config.toml` にエントリを追加

同名のリポジトリが既に登録されている場合はエラーになります。

### 例

```bash
ws repos add                              # カレントディレクトリを登録
ws repos add ~/projects/my-repo           # 特定のパスを登録
ws repos add . --name my-app              # カスタム名で登録
```

---

## ws repos list

登録済みリポジトリの一覧を表示します。

### 使い方

```bash
ws repos list
```

### 出力例

```
NAME     PATH                          URL
────     ────                          ───
my-repo  ~/projects/my-repo            git@github.com:user/my-repo.git
another  ~/projects/another
```

---

## ws repos rm

リポジトリの登録を解除します。

### 使い方

```bash
ws repos rm <name>
```

### 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `name` | はい | 登録解除するリポジトリの名前 |

### 例

```bash
ws repos rm my-repo
```
