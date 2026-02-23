# クイックスタート

## bare 構成（推奨）

### 1. bare clone

```bash
mkdir my-project && cd my-project
ws repos clone https://github.com/example/repo.git
```

`.bare/` ディレクトリに bare リポジトリが作成され、デフォルトブランチ（例: `main`）の worktree が自動的にセットアップされます。

### 2. 機能ブランチで作業

```bash
ws new feature/foo
```

`feature/foo` ブランチの worktree が作成されます。

### 結果のディレクトリ構造

```
my-project/
├── .bare/                         # bare リポジトリ
│   └── worktree-store/            # shared 機能の store（任意）
├── main/                          # worktree
└── feature-foo/                   # worktree
```

## 通常構成

既存の `git clone` リポジトリ内でも `ws new` が使えます。worktree はリポジトリの親ディレクトリに作成されます。

```
parent/
├── my-project/                    # 通常の git リポジトリ（.git/ あり）
└── feature-foo/                   # worktree（../<name> に作成される）
```

## URL なしの bare 構成

リモートなしで新規プロジェクトを始める場合:

```bash
mkdir my-project && cd my-project
ws repos clone                            # 空の bare リポジトリを作成
ws new master                      # orphan ブランチで worktree を作成
```

## 共有ファイルのセットアップ

worktree 間で `.envrc` や `.mcp.json` を共有したい場合:

```bash
# worktree 内で実行
ws store track -s symlink .envrc
ws store track -s symlink .mcp.json
ws store track -s copy .env.local
```

以降、`ws new` で新しい worktree を作成するたびに、これらのファイルが自動的に配布されます。

詳しくは[共有ストア](../concepts/shared-store.md)を参照してください。
