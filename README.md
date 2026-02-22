# ws - workspace (git worktree) マネージャー

git bare clone + worktree パターンでの開発を支援する CLI ツール。

## 特徴

- **bare clone + worktree の一括セットアップ** — `ws clone` → `ws new` で開発開始
- **共有ストア** — gitignored ファイルを worktree 間で自動共有
- **VSCode 連携** — worktree 作成後に自動で VSCode を起動
- **インタラクティブモード** — 対話的なコマンド選択

## クイックスタート

```bash
ws clone https://github.com/example/repo.git
ws new main
ws new feature/awesome --from main
```

## ドキュメント

詳細なドキュメントは **[ws ドキュメント](https://langify-org.github.io/ws-cli/)** を参照してください。

## インストール

```bash
# Nix flake
nix run github:langify-org/ws-cli

# cargo
cargo install --git https://github.com/langify-org/ws-cli.git
```

## ライセンス

MIT
