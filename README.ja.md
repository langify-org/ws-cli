# ws - workspace & repository manager

> **[English version](README.md)**

システムのリポジトリ・worktree・共有設定を一元管理する CLI ツールです。

![デモ](demo.ja.gif)

## なぜ ws？

複数のリポジトリやブランチにまたがる開発には、繰り返し発生する手間があります:

- **bare clone の初期化が煩雑** — `git clone --bare` 後に worktree を手動で追加する必要がある
- **gitignored ファイルの管理** — `.env`, `.env.local`, `.claude/settings.local.json` などは git 管理外のため、worktree を作るたびに手動でコピーやリンクが必要
- **リポジトリの散在** — リポジトリがあちこちのディレクトリに存在し、統一的に把握・管理する手段がない

ws はリポジトリの登録、worktree の管理、gitignored ファイルの共有を単一の CLI で解決します。

## 特徴

- **リポジトリレジストリ** — `ws repos` でシステム全体のリポジトリを登録・管理
- **bare clone + worktree の一括セットアップ** — `ws repos clone` → `ws new` の2コマンドで開発開始
- **共有ストア** — gitignored ファイルを worktree 間で自動共有（symlink / copy の2戦略）
- **インタラクティブモード** — 対話的なコマンド選択

## 共有ストア

共有ストアは worktree 間で gitignored ファイルを一元管理する仕組みです。ファイルを一度登録すれば、`ws new` で新しい worktree を作成するたびに自動的に配布されます。

```bash
# ファイルを store に登録
ws store track -s symlink .claude/settings.local.json  # symlink で共有（全 worktree で同じ内容）
ws store track -s copy .env            # copy で配布（worktree ごとにカスタマイズ可能）
ws store track -s copy .env.local
ws store track -s copy .env.development
```

| | symlink | copy |
|---|---------|------|
| 配布方法 | シンボリックリンク | ファイルコピー |
| 内容の共有 | 全 worktree で同一 | worktree ごとに独立 |
| 更新の反映 | 即座（リンク先が同じ） | `push` / `pull` が必要 |
| 用途 | `.claude/settings.local.json` など | `.env`, `.env.local`, `.env.development` など |

> [!TIP]
> `push`, `pull`, `status`, `untrack` を含む詳細なワークフローは [共有ストア](https://langify-org.github.io/ws-cli/ja/concepts/shared-store.html) を参照してください。

## クイックスタート

### clone 済みのリポジトリがある場合

```bash
cd my-project
ws new feature/awesome       # 新しいブランチの worktree を作成
```

### 新たにリポジトリを clone する場合

```bash
ws repos clone https://github.com/example/repo.git
ws new feature/awesome
```

### 新規プロジェクトをローカルで始める場合

```bash
mkdir my-project && cd my-project
ws repos clone               # 空の bare リポジトリを作成
ws new main                  # orphan ブランチで worktree を作成
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

[Cachix](https://app.cachix.org/cache/langify-org) でバイナリキャッシュを提供しています。ソースからのビルドを省略できます:

```bash
cachix use langify-org
```

```bash
nix run github:langify-org/ws-cli
```

### Home Manager (Nix)

```nix
# flake.nix の inputs に追加
inputs.ws-cli.url = "github:langify-org/ws-cli";

# home.nix
imports = [ inputs.ws-cli.homeManagerModules.default ];

programs.ws = {
  enable = true;
  package = inputs.ws-cli.packages.${system}.default;
  repos = {
    my-repo = {
      path = "/Users/user/projects/my-repo";
      url = "git@github.com:user/my-repo.git";
    };
  };
};
```

> [!TIP]
> Home Manager、nix-darwin、NixOS の場合は `cachix use` の代わりに Nix の設定に substituter を追加してください:
>
> ```nix
> nix.settings = {
>   substituters = [ "https://langify-org.cachix.org" ];
>   trusted-public-keys = [ "langify-org.cachix.org-1:zO6Hf3s6e3Ex7PDSazL1A7XwR/3Deui7G3LUrs4+nq4=" ];
> };
> ```

### cargo

```bash
cargo install --git https://github.com/langify-org/ws-cli.git
```

## 開発

### デモ GIF の再生成

デモ GIF は [VHS](https://github.com/charmbracelet/vhs) の tape ファイルから生成されます。ローカルでのビルドやツールのインストールは不要で、すべて Nix 経由で実行できます:

```bash
# 英語版 (demo.gif)
nix run .#demo

# 日本語版 (demo.ja.gif)
nix run .#demo -- demo.ja.tape
```

tape ファイル（`demo.tape`, `demo.ja.tape`）は隔離された環境を自動構築する自己完結型スクリプトのため、ローカルの設定に影響しません。

## ライセンス

MIT
