# ws - AI-ready workspace & repository manager

> **[English version](../)**

**ws** はシステムのリポジトリ・worktree・共有設定を一元管理する CLI ツールです。

## なぜ ws？

複数のリポジトリやブランチにまたがる開発には、繰り返し発生する手間があります。特に以下の課題があります:

- **bare clone の初期化が煩雑** — `git clone --bare` 後に worktree を手動で追加する必要がある
- **gitignored ファイルの管理** — `.envrc`, `.mcp.json`, `.env.local` などは git 管理外のため、worktree を作るたびに手動でコピーやリンクが必要
- **リポジトリの散在** — リポジトリがあちこちのディレクトリに存在し、統一的に把握・管理する手段がない

ws はリポジトリの登録、worktree の管理、gitignored ファイルの共有を単一の CLI で解決します。

## 特徴

- **リポジトリレジストリ** — `ws repos` でシステム全体のリポジトリを登録・管理
- **bare clone + worktree の一括セットアップ** — `ws repos clone` → `ws new` の2コマンドで開発開始
- **共有ストア** — gitignored ファイルを worktree 間で自動共有（symlink / copy の2戦略）
- **AI エージェント連携** — エージェント設定の worktree 間共有 + システム全体のリポジトリ認識
- **インタラクティブモード** — 対話的なコマンド選択

## クイックスタート

```bash
# リポジトリを bare clone（デフォルトブランチの worktree は自動作成）
ws repos clone https://github.com/example/repo.git

# 機能ブランチを作成
ws new feature/awesome
```

詳しくは[クイックスタート](getting-started/quick-start.md)を参照してください。
