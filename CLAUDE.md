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

単一ファイル構成（`src/main.rs`）。機能別に以下のセクションに分かれている:

1. **CLI 定義** — `argh` derive macro でサブコマンドを宣言的に定義
2. **ヘルパー** — git 操作（`git_output`）、bare ディレクトリ検出、manifest 読み書き
3. **コマンド実装** — `cmd_clone`, `cmd_new`, `cmd_rm`, `cmd_list`, `cmd_status`, `cmd_shared_*`
4. **インタラクティブモード** — fzf を外部プロセスとして呼び出し、対話的にコマンドを組み立てて再帰実行

### shared store の仕組み

`<git-common-dir>/worktree-store/` に manifest（`strategy:filepath` の行形式）とファイルのマスターコピーを格納。strategy は `symlink`（全 worktree で同一内容を共有）と `copy`（worktree ごとにカスタマイズ可能）の2種。`ws new` 実行時に store から自動配布される。

## コーディング規約

- エラーハンドリングは `anyhow::Result` + `.context()` で統一
- CLI 引数の doc comment とユーザー向けメッセージは日本語
- 外部コマンド（git, fzf, code）は `std::process::Command` で直接呼び出し
