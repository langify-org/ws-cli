# ws repos

リポジトリの登録を管理します。登録情報は `~/.config/ws/config.toml` に保存されます。

> [!TIP]
> Home Manager を使用している場合、`ws repos add` の代わりに `programs.ws.repos` でリポジトリを宣言的に管理できます。詳しくは[インストール](../getting-started/installation.md)を参照してください。

## サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| [`ws repos add`](#ws-repos-add) | リポジトリを登録 |
| [`ws repos list`](#ws-repos-list) | 登録済みリポジトリの一覧表示 |
| [`ws repos status`](#ws-repos-status) | 登録済み全リポジトリの詳細ステータスを表示 |
| [`ws repos rm`](#ws-repos-rm) | リポジトリの登録解除 |

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
my-repo              /Users/user/projects/my-repo (git@github.com:user/my-repo.git)
another              /Users/user/projects/another
```

---

## ws repos status

登録済み全リポジトリの詳細ステータスを表示します。GIT_DIR の種類や worktree のツリー表示を含みます。

### 使い方

```bash
ws repos status
```

### 動作

登録済みリポジトリごとに以下を表示します:

1. **リポジトリ名とパス**
2. **GIT_DIR** — bare worktree パターンの場合は `.bare`、通常の clone の場合は `.git`
3. **worktree ツリー** — 全 worktree のブランチ名とコミットハッシュ

登録済みリポジトリのパスが存在しない場合は `NOT_FOUND` が表示されます。

### 出力例

**bare worktree パターン:**

```
my-project (/Users/user/projects/my-project)
  GIT_DIR: .bare
  Worktrees:
    ├── main   [main] abc1234
    ├── feature-foo   [feature/foo] def5678
    └── fix-bar   [fix/bar] 9ab0123
```

**通常の clone:**

```
another-repo (/Users/user/projects/another-repo)
  GIT_DIR: .git
  Main worktree:
    .   [main] abc1234
  Linked worktrees:
    └── ../another-repo-feature   [feature/x] def5678
```

**パスが存在しない場合:**

```
old-repo (/Users/user/projects/old-repo)
  NOT_FOUND
```

### ステータス値

| 値 | 説明 |
|----|------|
| `GIT_DIR: .bare` | bare worktree パターンのリポジトリ |
| `GIT_DIR: .git` | 通常の git clone |
| `NOT_FOUND` | 登録済みパスがディスク上に存在しない |

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
