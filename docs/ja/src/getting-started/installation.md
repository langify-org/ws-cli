# インストール

## Nix flake（推奨）

ws は Nix flake として提供されています。

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
