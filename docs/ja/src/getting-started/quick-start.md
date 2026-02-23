# クイックスタート

## clone 済みのリポジトリがある場合

既存の `git clone` リポジトリ内で `ws new` が使えます。worktree はリポジトリの親ディレクトリに作成されます。

```bash
cd my-project
ws new feature/foo
```

```
parent/
├── my-project/                    # 既存のリポジトリ（.git/ あり）
└── feature-foo/                   # worktree（../<name> に作成される）
```

## 新たにリポジトリを clone する場合

`ws repos clone` で bare リポジトリを作成し、デフォルトブランチの worktree が自動的にセットアップされます。

```bash
mkdir my-project && cd my-project
ws repos clone https://github.com/example/repo.git
```

その後、機能ブランチの worktree を作成します:

```bash
ws new feature/foo
```

### ディレクトリ構造

```
my-project/
├── .bare/                         # bare リポジトリ
│   └── worktree-store/            # shared 機能の store（任意）
├── main/                          # worktree（デフォルトブランチ）
└── feature-foo/                   # worktree
```

## 新規プロジェクトをローカルで始める場合

リモートなしで新規プロジェクトを始める場合:

```bash
mkdir my-project && cd my-project
ws repos clone               # 空の bare リポジトリを作成
ws new main                  # orphan ブランチで worktree を作成
```

## 共有ファイルのセットアップ

worktree 間で `.env` や `.claude/settings.local.json` を共有したい場合:

```bash
# worktree 内で実行
ws store track -s symlink .claude/settings.local.json
ws store track -s copy .env
```

以降、`ws new` で新しい worktree を作成するたびに、これらのファイルが自動的に配布されます。

詳しくは[共有ストア](../concepts/shared-store.md)を参照してください。
