# ws - workspace (git worktree) マネージャー

> **[English version](README.md)**

git bare clone + worktree パターンでの開発を支援する CLI ツールです。

## なぜ ws？

git worktree は複数のブランチを同時に開ける強力な機能ですが、セットアップや運用に手間がかかります:

- **bare clone の初期化が煩雑** — `git clone --bare` 後に worktree を手動で追加する必要がある
- **gitignored ファイルの管理** — `.envrc`, `.mcp.json`, `.env`, `.env.local` などは git 管理外のため、worktree を作るたびに手動でコピーやリンクが必要

ws はこれらの課題を解決し、worktree ベースの開発を快適にします。

## 特徴

- **bare clone + worktree の一括セットアップ** — `ws clone` → `ws new` の2コマンドで開発開始
- **共有ストア** — gitignored ファイルを worktree 間で自動共有（symlink / copy の2戦略）
- **リポジトリ登録** — `ws repos add` で既存リポジトリを登録して ws で管理
- **インタラクティブモード** — 対話的なコマンド選択

## bare clone + worktree パターン

通常の `git clone` とは異なり、作業ディレクトリを持たない bare リポジトリを中心に据え、各ブランチを独立したディレクトリとして展開します。

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

> [!TIP]
> 命名規則などの詳細は [bare clone + worktree パターン](https://langify-org.github.io/ws-cli/ja/concepts/bare-worktree.html) を参照してください。

## 共有ストア

共有ストアは worktree 間で gitignored ファイルを一元管理する仕組みです。ファイルを一度登録すれば、`ws new` で新しい worktree を作成するたびに自動的に配布されます。

```bash
# ファイルを store に登録
ws store track -s symlink .envrc       # symlink で共有（全 worktree で同じ内容）
ws store track -s symlink .mcp.json
ws store track -s copy .env            # copy で配布（worktree ごとにカスタマイズ可能）
ws store track -s copy .env.local
ws store track -s copy .env.development
```

| | symlink | copy |
|---|---------|------|
| 配布方法 | シンボリックリンク | ファイルコピー |
| 内容の共有 | 全 worktree で同一 | worktree ごとに独立 |
| 更新の反映 | 即座（リンク先が同じ） | `push` / `pull` が必要 |
| 用途 | `.envrc`, `.mcp.json` など | `.env`, `.env.local`, `.env.development` など |

> [!TIP]
> `push`, `pull`, `status`, `untrack` を含む詳細なワークフローは [共有ストア](https://langify-org.github.io/ws-cli/ja/concepts/shared-store.html) を参照してください。

## クイックスタート

```bash
# リポジトリを bare clone（デフォルトブランチの worktree は自動作成）
ws clone https://github.com/example/repo.git

# 機能ブランチを作成
ws new feature/awesome
```

> [!TIP]
> 詳しくは [クイックスタート](https://langify-org.github.io/ws-cli/ja/getting-started/quick-start.html) を参照してください。

## ドキュメント

詳細なリファレンスは **[ws ドキュメント](https://langify-org.github.io/ws-cli/ja/)** を参照してください。

## インストール

### シェルインストーラー (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/langify-org/ws-cli/releases/latest/download/ws-installer.sh | sh
```

### Homebrew

```bash
brew install langify-org/tap/ws
```

### Nix flake

```bash
nix run github:langify-org/ws-cli
```

### Home Manager (Nix)

```nix
# flake.nix の inputs に追加
inputs.ws-cli.url = "github:langify-org/ws-cli";

# home.nix
home.packages = [
  inputs.ws-cli.packages.${system}.default
];
```

### cargo

```bash
cargo install --git https://github.com/langify-org/ws-cli.git
```

## ライセンス

MIT
