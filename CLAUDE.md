# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

**ws** は git bare clone + worktree パターンでの開発を支援する Rust CLI ツール。worktree の作成・削除に加え、gitignored ファイル（`.envrc`, `.mcp.json` 等）を worktree 間で共有する store 機能を持つ。

## 開発環境

`direnv allow` で Nix flake ベースの開発シェルに自動的に入る（cargo, cargo-watch が利用可能になる）。

## ビルド・実行

```bash
# 開発ビルド
cargo build

# Nix で再現可能ビルド（./result/bin/ws に出力）
nix build

# ファイル変更監視
cargo watch -x build
```

テストスイートは未整備。

## アーキテクチャ

責務ごとにモジュール分割した構成:

```
src/
  main.rs            エントリポイント + run() ディスパッチ
  cli.rs             clap v4 derive による CLI 型定義 + parse_with_i18n()
  git.rs             git コマンド実行ヘルパー
  store.rs           shared store のデータ層（ManifestEntry, manifest 操作, file_status）
  commands/
    mod.rs           サブモジュール宣言
    worktree.rs      clone, new, rm, list, generate_name
    status.rs        status（統合表示）
    shared.rs        shared track/status/push/pull
  interactive.rs     inquire を使った対話モード
```

```
locales/
  en.yml             英語ロケール (デフォルト/フォールバック)
  ja.yml             日本語ロケール
  zh-CN.yml          簡体字中国語ロケール
```

依存グラフ（循環なし）:
- `cli` / `git` — 依存なし（最下層）
- `store` → `git`
- `commands/worktree` → `cli`, `git`, `store`
- `commands/status` → `git`, `store`
- `commands/shared` → `cli`, `git`, `store`
- `interactive` → `cli`, `git`, `commands::worktree`
- `main` → `cli`, `commands/*`, `interactive`

### shared store の仕組み

`<git-common-dir>/worktree-store/` に manifest（`strategy:filepath` の行形式）とファイルのマスターコピーを格納。strategy は `symlink`（全 worktree で同一内容を共有）と `copy`（worktree ごとにカスタマイズ可能）の2種。`ws new` 実行時に store から自動配布される。

## コーディング規約

- エラーハンドリングは `anyhow::Result` + `.context()` で統一
- ユーザー向け文字列は `rust_i18n` の `t!()` マクロで多言語化（en/ja/zh-CN 対応）
- ロケールキーの名前空間はモジュール構造に対応（`cli.*`, `git.*`, `store.*` 等）
- ステータスコード (`OK`, `MISSING`, `ERROR` 等) は技術用語として全ロケール英語固定
- CLI パーサーは `clap v4` derive + `parse_with_i18n()` でランタイムに i18n ヘルプを適用
- 外部コマンド（git, code）は `std::process::Command` で直接呼び出し
- 対話的な選択・入力は `inquire` クレートを使用
