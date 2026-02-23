# ws repos

リポジトリの登録を管理します。登録情報は `~/.config/ws/config.toml` に保存されます。

## サブコマンド

| サブコマンド | 説明 |
|-------------|------|
| [`ws repos add`](#ws-repos-add) | リポジトリを登録 |
| [`ws repos list`](#ws-repos-list) | 登録済みリポジトリの一覧表示 |
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

1. パスを解決して検証（git リポジトリである必要あり）
2. `origin` からリモート URL を自動検出
3. `~/.config/ws/config.toml` にエントリを追加

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
