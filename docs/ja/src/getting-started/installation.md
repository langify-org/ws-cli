# インストール

## Nix flake（推奨）

ws は Nix flake として提供されています。

### バイナリキャッシュ (Cachix)

[Cachix](https://app.cachix.org/cache/langify-org) でバイナリキャッシュを提供しています。以下を一度実行するとソースからのビルドを省略できます:

```bash
cachix use langify-org
```

**Home Manager**、**nix-darwin**、**NixOS** の場合は、Nix の設定に substituter を追加してください:

```nix
nix.settings = {
  substituters = [ "https://langify-org.cachix.org" ];
  trusted-public-keys = [ "langify-org.cachix.org-1:zO6Hf3s6e3Ex7PDSazL1A7XwR/3Deui7G3LUrs4+nq4=" ];
};
```

### 直接実行

```bash
nix run github:langify-org/ws-cli
```

### プロファイルにインストール

```bash
nix profile install github:langify-org/ws-cli
```

### flake.nix の入力に追加

```nix
{
  inputs = {
    ws-cli.url = "github:langify-org/ws-cli";
  };

  # outputs で ws-cli.packages.${system}.default を参照
}
```

### Home Manager

flake の入力に `ws-cli` を追加し、Home Manager モジュールをインポートします:

```nix
# flake.nix
{
  inputs = {
    ws-cli.url = "github:langify-org/ws-cli";
  };
}
```

```nix
# home.nix
{ inputs, system, ... }:
{
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
}
```

#### `programs.ws` オプション

| オプション | 型 | デフォルト | 説明 |
|-----------|-----|-----------|------|
| `enable` | bool | `false` | ws を有効にする |
| `package` | package | `pkgs.ws` | インストールする ws パッケージ |
| `repos` | attrset | `{}` | `~/.config/ws/config.toml` に登録するリポジトリ |

`repos` の各エントリ:

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| `path` | string | はい | リポジトリのパス |
| `url` | string | いいえ | リモート URL |

## cargo install

Rust ツールチェインがインストール済みであれば、cargo から直接ビルドできます。

```bash
cargo install --git https://github.com/langify-org/ws-cli.git
```

## ソースからビルド

```bash
git clone https://github.com/langify-org/ws-cli.git
cd ws-cli
cargo build --release
# ./target/release/ws を PATH の通った場所にコピー
```

## 依存関係

ws は以下の外部コマンドを利用します:

| コマンド | 必須 | 用途 |
|---------|------|------|
| `git` | はい | worktree 操作全般 |
