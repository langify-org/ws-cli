# ws - workspace マネージャー

> **[English version](../)**

**ws** は git bare clone + worktree パターンでの開発を支援する CLI ツールです。

## なぜ ws？

git worktree は複数のブランチを同時に開ける強力な機能ですが、セットアップや運用に手間がかかります。特に以下の課題があります:

- **bare clone の初期化が煩雑** — `git clone --bare` 後に worktree を手動で追加する必要がある
- **gitignored ファイルの管理** — `.envrc`, `.mcp.json`, `.env.local` などは git 管理外のため、worktree を作るたびに手動でコピーやリンクが必要

ws はこれらの課題を解決し、worktree ベースの開発を快適にします。

## 特徴

- **bare clone + worktree の一括セットアップ** — `ws clone` → `ws new` の2コマンドで開発開始
- **共有ストア** — gitignored ファイルを worktree 間で自動共有（symlink / copy の2戦略）
- **リポジトリ登録** — `ws repos add` で既存リポジトリを登録して ws で管理
- **インタラクティブモード** — 対話的なコマンド選択

## クイックスタート

```bash
# リポジトリを bare clone（デフォルトブランチの worktree は自動作成）
ws clone https://github.com/example/repo.git

# 機能ブランチを作成
ws new feature/awesome
```

詳しくは[クイックスタート](getting-started/quick-start.md)を参照してください。
