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

## テスト

```bash
cargo test
```

- ユニットテスト: `crates/ws-core/src/store.rs`, `crates/ws-core/src/commands/worktree.rs` にインライン (`#[cfg(test)] mod tests`)
- E2E テスト: `crates/ws-cli/tests/` ディレクトリに `assert_cmd` でバイナリ実行（`tempfile` で一時 git リポジトリを構築）
- E2E テストのロケールは `LC_ALL=en` で英語固定

## アーキテクチャ

Cargo workspace で `ws-core`（ライブラリ）と `ws-cli`（バイナリ）に分割:

```
Cargo.toml               [workspace] 定義
locales/                  i18n ファイル（両クレートから参照）
  en.yml                  英語ロケール (デフォルト/フォールバック)
  ja.yml                  日本語ロケール
  zh-CN.yml               簡体字中国語ロケール
crates/
  ws-core/                ライブラリクレート（コアロジック）
    src/
      lib.rs              i18n!() + pub mod 宣言 + detect_and_set_locale()
      cli.rs              clap v4 derive による CLI 型定義 + parse_with_i18n()
      git.rs              git コマンド実行ヘルパー
      store.rs            shared store のデータ層（Strategy, ManifestEntry, manifest 操作, file_status, path_or_symlink_exists）
      config.rs           設定ファイル（~/.config/ws/config.toml）の読み書き
      commands/
        mod.rs            サブモジュール宣言
        worktree.rs       clone, new, rm, list, generate_name
        status.rs         status（統合表示）
        store.rs          store track/status/push/pull/untrack
        repos.rs          repos add/list/rm（リポジトリ登録管理）
  ws-cli/                 バイナリクレート（package name = "ws"）
    src/
      main.rs             エントリポイント + run() ディスパッチ
      interactive.rs      inquire を使った対話モード（ws_core:: 関数を直接呼び出し）
    tests/                E2E テスト
```

依存グラフ（循環なし）:
- `cli` / `git` — 依存なし（最下層）
- `store` → `git`
- `config` — 依存なし
- `commands/worktree` → `cli`, `git`, `store`, `config`
- `commands/status` → `git`, `store`
- `commands/store` → `cli`, `git`, `store`
- `commands/repos` → `cli`, `config`
- `ws-cli/interactive` → `ws_core::cli`, `ws_core::config`, `ws_core::git`, `ws_core::store`, `ws_core::commands::*`
- `ws-cli/main` → `ws_core::cli`, `ws_core::commands::*`, `interactive`

### shared store の仕組み

`<git-common-dir>/worktree-store/` に manifest（`strategy:filepath` の行形式）とファイルのマスターコピーを格納。strategy は `Strategy` enum で型安全に管理され、`symlink`（全 worktree で同一内容を共有）と `copy`（worktree ごとにカスタマイズ可能）の2種がある。`ws new` 実行時に store から自動配布される。

strategy の使い分け:
- `symlink` の典型例: `.envrc`, `.tool-versions` など全 worktree で共通の設定ファイル
- `copy` の典型例: `.mcp.json`, `.env` など worktree ごとにカスタマイズが必要なファイル

### repos のリポジトリルート解決

`ws repos add` は指定パス（またはcwd）からリポジトリルートを自動解決して登録する（`resolve_repo_root()`）。

解決ロジック:
1. `git rev-parse --git-common-dir` が `.bare` で終わる → その親ディレクトリ（bare worktree パターン）
2. それ以外 → `git rev-parse --show-toplevel`（通常の clone）
3. どちらも失敗 → 指定パスをそのまま使用

これにより worktree 内から実行しても bare root が登録され、`ws clone` の自動登録と一致する。

### 対話モードの関数直接呼び出し

`ws-cli/src/interactive.rs` は `ws_core::commands::*` の関数を直接呼び出す。外部プロセス (`Command::new("ws")`) は使用しない。これにより `cargo run -- i` で開発中のビルドがそのまま対話モードに反映される。

### 対話モードのコンパイル時安全機構

`interactive.rs` の `_ensure_all_commands_in_interactive()` は `WsCommand` の全バリアントをワイルドカードなしで列挙する sentinel 関数。新しいサブコマンドを追加した際に対話モードの更新漏れをコンパイルエラーで検出する。

## コーディング規約

- エラーハンドリングは `anyhow::Result` + `.context()` で統一
- ユーザー向け文字列は `rust_i18n` の `t!()` マクロで多言語化（en/ja/zh-CN 対応）
- ロケールキーの名前空間はモジュール構造に対応（`cli.*`, `git.*`, `store.*` 等）
- ステータスコード (`OK`, `MISSING`, `ERROR` 等) およびテーブルヘッダー (`STRATEGY`, `FILE`, `STATUS`) は全ロケール英語固定
- strategy は `Strategy` enum (`Symlink`, `Copy`) で管理する。文字列リテラル `"symlink"` / `"copy"` を直接使わないこと
- ファイル存在チェックには `path_or_symlink_exists()` を使う（`Path::exists()` はリンク切れ symlink で false を返すため）
- CLI パーサーは `clap v4` derive + `parse_with_i18n()` でランタイムに i18n ヘルプを適用
- 外部コマンド（git, code）は `std::process::Command` で直接呼び出し
- 対話的な選択・入力は `inquire` クレートを使用
