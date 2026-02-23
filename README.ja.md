# ws - AI-ready workspace & repository manager

> **[English version](README.md)**

システムのリポジトリ・worktree・共有設定を一元管理する CLI ツールです。

## なぜ ws？

複数のリポジトリやブランチにまたがる開発には、繰り返し発生する手間があります:

- **bare clone の初期化が煩雑** — `git clone --bare` 後に worktree を手動で追加する必要がある
- **gitignored ファイルの管理** — `.envrc`, `.mcp.json`, `.env`, `.env.local` などは git 管理外のため、worktree を作るたびに手動でコピーやリンクが必要
- **リポジトリの散在** — リポジトリがあちこちのディレクトリに存在し、統一的に把握・管理する手段がない

ws はリポジトリの登録、worktree の管理、gitignored ファイルの共有を単一の CLI で解決します。

## 特徴

- **リポジトリレジストリ** — `ws repos` でシステム全体のリポジトリを登録・管理
- **bare clone + worktree の一括セットアップ** — `ws repos clone` → `ws new` の2コマンドで開発開始
- **共有ストア** — gitignored ファイルを worktree 間で自動共有（symlink / copy の2戦略）
- **AI エージェント連携** — エージェント設定の worktree 間共有 + システム全体のリポジトリ認識
- **インタラクティブモード** — 対話的なコマンド選択

## AI エージェント連携

Claude Code などの AI コーディングエージェントは `.mcp.json`、`.claude/settings.local.json`、`.env` など複数の設定ファイルを必要とします。これらはすべて gitignored のため、新しい worktree を作るたびに手動で配置する必要があります。

### worktree 間の設定共有

ws なら設定ファイルを共有ストアに一度登録するだけで、すべての worktree に自動配布されます:

```bash
# 共有設定 — symlink で全 worktree を同期
ws store track -s symlink .mcp.json
ws store track -s symlink .claude/settings.local.json

# worktree 固有のシークレット — copy で個別カスタマイズ可能
ws store track -s copy .env
ws store track -s copy .env.local

# 新しい worktree は最初から AI エージェント対応
ws new feature/awesome
cd ../feature-awesome
# Claude Code がすぐに動く — セットアップ不要
```

symlink 戦略では `.mcp.json` や `.claude/settings.local.json` を1つの worktree で更新すれば、他のすべての worktree に即座に反映されます。copy 戦略では各 worktree が独自の `.env` を持ちつつ、動作するベースラインから始められます。

### システム全体のリポジトリ認識

AI エージェントはリポジトリの境界を越えて作業することがよくあります — 別プロジェクトのコードを参照したり、リポジトリ間で変更を調整したり、システム全体の構造を把握したり。`ws repos` はエージェントにリポジトリのレジストリを提供します:

```bash
# リポジトリを登録
ws repos add ~/projects/frontend
ws repos add ~/projects/backend
ws repos add ~/projects/shared-lib

# エージェントがプロジェクト全体の構造を把握できる
ws repos list
ws status
```

一元管理されたレジストリがあることで、AI エージェントは関連プロジェクトの場所を発見し、システム全体の構造を理解し、毎回パスを手動で説明しなくてもリポジトリ間を移動できるようになります。

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
ws repos clone https://github.com/example/repo.git

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

## ライセンス

MIT
