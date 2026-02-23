# bare clone + worktree パターン

## 概要

git bare clone + worktree パターンは、1つの bare リポジトリに対して複数の worktree を並列に配置する開発スタイルです。通常の `git clone` とは異なり、作業ディレクトリを持たない bare リポジトリを中心に据え、各ブランチを独立したディレクトリとして展開します。

## 通常の clone との違い

### 通常の clone

```
my-project/
├── .git/
├── src/
└── ...
```

1つのディレクトリ = 1つのブランチ。ブランチを切り替えるには `git checkout` / `git switch` が必要で、未コミットの変更があると切り替えできません。

### bare clone + worktree

```
my-project/
├── .bare/              # bare リポジトリ（作業ディレクトリなし）
├── main/               # main ブランチの worktree
│   ├── src/
│   └── ...
└── feature-foo/        # feature/foo ブランチの worktree
    ├── src/
    └── ...
```

複数のブランチを同時に開けるため、以下のメリットがあります:

- **ブランチ切り替え不要** — 各ブランチが独立したディレクトリ
- **並行作業が容易** — レビュー中のブランチを開いたまま別の作業ができる
- **ビルドキャッシュの保持** — ブランチごとに `target/` や `node_modules/` が独立

## ws での運用

### bare リポジトリの作成

```bash
mkdir my-project && cd my-project
ws repos clone https://github.com/example/repo.git
```

`ws repos clone` は内部で `git clone --bare <url> .bare` を実行します。

### worktree の作成

```bash
ws new feature/foo                 # HEAD から新規ブランチを作成
```

`ws repos clone` はデフォルトブランチ（例: `main`）の worktree を自動作成します。`ws new` は内部で `git worktree add` を実行します。

### worktree の削除

```bash
ws rm feature-foo
```

`ws rm` は `git worktree remove` を実行します。

## worktree の命名規則

`ws new` に渡す名前がそのまま worktree のディレクトリ名とブランチ名になります。ディレクトリ名では `/` が `-` に変換されるため:

| 名前 | ディレクトリ | ブランチ |
|------|------------|---------|
| `main` | `main/` | `main` |
| `feature/foo` | `feature-foo/` | `feature/foo` |

`--branch` オプションでブランチ名を明示的に変更できます。

名前を省略した場合は、ランダムな名前（例: `gentle-happy-fox`）が自動生成されます。

## リポジトリルートの解決

`ws` のいくつかのコマンド（`ws repos add`、`ws repos clone` 時の自動登録など）は、プロジェクトを代表する最上位ディレクトリである**リポジトリルート**を特定する必要があります。

### 解決ルール

| リポジトリの種類 | 解決方法 | 結果 |
|---|---|---|
| **bare worktree**（`ws repos clone`） | `git rev-parse --git-common-dir` → `.bare` で終わる場合、その親ディレクトリ | `my-project/` |
| **通常の clone** | `git rev-parse --show-toplevel` | `my-project/` |

### 例: bare worktree

```
my-project/          ← リポジトリルート
├── .bare/
├── main/
│   └── （ここにいる）
└── feature-foo/
```

`main/` 内で `ws repos add` を実行すると、`main/` ではなく `my-project/` が登録されます。

### 例: 通常の clone

```
my-project/          ← リポジトリルート
├── .git/
├── src/
│   └── （ここにいる）
└── ...
```

`src/` 内で `ws repos add` を実行すると `my-project/` が登録されます。
