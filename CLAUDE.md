# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

**ws** は git bare clone + worktree パターンでの開発を支援する Rust CLI ツール。worktree の作成・削除に加え、gitignored ファイル（`.env`, `.claude/settings.local.json` 等）を worktree 間で共有する store 機能を持つ。

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

- ユニットテスト: `crates/ws-core/src/store.rs`, `crates/ws-core/src/context.rs`, `crates/ws-core/src/commands/worktree.rs` にインライン (`#[cfg(test)] mod tests`)
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
      git.rs              git コマンド実行ヘルパー + resolve_repo_root()
      store.rs            shared store のデータ層（Strategy, FileStatus, ManifestEntry, manifest 操作, file_status, path_or_symlink_exists）
      config.rs           設定ファイル（~/.config/ws/config.toml）の読み書き
      context.rs          AppContext（3層の状態を一括構築）+ abbreviate_home() + print_table()
      ui.rs               カラー出力（スタイル定数, styled(), section_header(), StyledCell）
      commands/
        mod.rs            サブモジュール宣言
        completions.rs    completions（シェル補完スクリプト生成）
        worktree.rs       clone, new, rm
        open.rs           open（登録済みリポジトリの worktree をエディタで開く）
        status.rs         status（3セクション統合ダッシュボード: Repositories / Current Repository / Current Workspace）
        store.rs          store track/status/push/pull/untrack
        repos.rs          repos add/list/rm（リポジトリ登録管理）+ WorktreeEntry/parse_worktree_list（status.rs/open.rs と共有）
  ws-cli/                 バイナリクレート（package name = "ws"）
    src/
      main.rs             エントリポイント + run() ディスパッチ
      interactive.rs      inquire を使った対話モード（ws_core:: 関数を直接呼び出し）
    tests/                E2E テスト
```

依存グラフ（循環なし）:
- `cli` / `git` — 依存なし（最下層。ただし `cli` は `store::Strategy` を参照）
- `ui` — 依存なし（`anstyle` のみ使用）
- `store` → `git`, `ui`
- `config` — 依存なし
- `context` → `config`, `git`, `store`, `ui`, `commands/repos`
- `commands/completions` → `cli`
- `commands/worktree` → `cli`, `git`, `store`, `ui`
- `commands/open` → `cli`, `config`, `commands/repos`, `ui`
- `commands/status` → `context`, `store`, `ui`, `commands/repos`
- `commands/store` → `cli`, `git`, `store`, `ui`, `context`
- `commands/repos` → `cli`, `config`, `ui`, `context`
- `ws-cli/interactive` → `ws_core::cli`, `ws_core::config`, `ws_core::context`, `ws_core::git`, `ws_core::store`, `ws_core::commands::*`
- `ws-cli/main` → `ws_core::cli`, `ws_core::context`, `ws_core::ui`, `ws_core::commands::*`, `interactive`

### AppContext と3層の状態モデル

`context.rs` の `AppContext` は読み取り系コマンド（`status`, `repos list`, `store status`）が共有する3層の状態を一括構築する:

- **Config** (`config.rs`): `~/.config/ws/config.toml` のリポジトリ登録情報
- **CurrentRepo**: カレントディレクトリが属するリポジトリのルート、bare/git 判定、worktree 一覧。config に登録があれば名前も解決
- **CurrentWorkspace**: 現在の worktree のルート、ブランチ、store の manifest

`AppContext::build()` は `load_config()` + `resolve_repo_root(None)` + `worktree_root()` + `store_dir()` を統合し、各コマンドが個別にこれらを呼び出す必要をなくしている。書き込み系コマンド（`repos add/rm`, `store track/push/pull/untrack`, `clone/new/rm`）は従来通り個別に必要な情報を取得する。

### shared store の仕組み

`<git-common-dir>/worktree-store/` に manifest（`strategy:filepath` の行形式）とファイルのマスターコピーを格納。strategy は `Strategy` enum で型安全に管理され、`symlink`（全 worktree で同一内容を共有）と `copy`（worktree ごとにカスタマイズ可能）の2種がある。`ws new` 実行時に store から自動配布される。

strategy の使い分け:
- `symlink` の典型例: `.claude/settings.local.json` など全 worktree で共通の設定ファイル
- `copy` の典型例: `.env`, `.env.local` など worktree ごとにカスタマイズが必要なファイル

### リポジトリルート解決

`git.rs` の `resolve_repo_root(path: Option<&Path>)` がリポジトリルート解決の統一関数。`AppContext`（`path=None` で cwd）と `ws repos add`（`path=Some(&path)`）の両方から呼ばれる。

解決ロジック:
1. `git rev-parse --git-common-dir` → `.bare` の親（bare worktree パターン）
2. 同上 → `.git` の親（通常の clone）
3. `git rev-parse --show-toplevel`（フォールバック）
4. `.bare` ディレクトリの直接検出（bare root にいる場合）

これにより worktree 内から実行しても bare root が登録され、`ws repos clone` の自動登録と一致する。

### 対話モードの関数直接呼び出し

`ws-cli/src/interactive.rs` は `ws_core::commands::*` の関数を直接呼び出す。外部プロセス (`Command::new("ws")`) は使用しない。これにより `cargo run -- interactive`（または `cargo run -- i`）で開発中のビルドがそのまま対話モードに反映される。

### 対話モードのコンパイル時安全機構

`interactive.rs` の `_ensure_all_commands_in_interactive()` は `WsCommand` の全バリアントをワイルドカードなしで列挙する sentinel 関数。新しいサブコマンドを追加した際に対話モードの更新漏れをコンパイルエラーで検出する。

### カラー出力

`anstyle` + `anstream` でターミナルカラーを実現。`clap v4` の transitive dependency のため追加クレートは最小限。

- スタイル定数・ヘルパーは `ui.rs` に集約。各モジュールは `use crate::ui` で利用
- `StyledCell` で plain（幅計算用）と styled（表示用）を分離し、ANSI-aware なテーブルパディングを実現
- `anstream::println!` / `anstream::eprintln!` を使用。非TTY環境（パイプ、E2Eテスト）で自動的にANSIコードを除去するため、テストでの明示的な除去は不要
- `NO_COLOR` 環境変数によるカラー無効化に対応（`anstream` が自動処理）
- セクションヘッダーは `ui::section_header()` でルーラー付き表示（`── Title ──────` 形式）

カラースキーム:
- ステータス `OK` → Green, `MISSING`/`MISSING(store)` → Red, `ERROR` → Red+Bold, `MODIFIED`/`NOT_LINK`/`WRONG_LINK` → Yellow
- セクションヘッダー → Bold（タイトル）+ Dim（罫線）、テーブルヘッダー → Bold、セパレータ → Dim
- カレントマーカー `*` → Green+Bold、ブランチ名 `[branch]` → Cyan、コミットハッシュ → Dim
- 成功メッセージ → Green、警告/Skip → Yellow、エラー → Red+Bold

## コーディング規約

- エラーハンドリングは `anyhow::Result` + `.context()` で統一
- ユーザー向け文字列は `rust_i18n` の `t!()` マクロで多言語化（en/ja/zh-CN 対応）
- ロケールキーの名前空間はモジュール構造に対応（`cli.*`, `git.*`, `store.*` 等）
- ステータスコード (`OK`, `MISSING`, `ERROR` 等) およびテーブルヘッダー (`STRATEGY`, `FILE`, `STATUS`) は全ロケール英語固定
- strategy は `Strategy` enum (`Symlink`, `Copy`) で管理する。`clap::ValueEnum` を derive しており CLI パース段階で型安全にバリデーションされる。文字列リテラル `"symlink"` / `"copy"` を直接使わないこと
- ファイルステータスは `FileStatus` enum で管理する。`Display` trait で表示文字列を提供。文字列リテラル `"OK"`, `"MISSING"` 等を直接使わないこと
- ファイル存在チェックには `path_or_symlink_exists()` を使う（`Path::exists()` はリンク切れ symlink で false を返すため）
- CLI パーサーは `clap v4` derive + `parse_with_i18n()` でランタイムに i18n ヘルプを適用
- 外部コマンド（git, code）は `std::process::Command` で直接呼び出し
- 対話的な選択・入力は `inquire` クレートを使用
- ターミナル出力は `anstream::println!` / `anstream::eprintln!` を使用（`println!` / `eprintln!` を直接使わない）
- カラースタイルは `ui.rs` のスタイル定数を使用。ハードコードしない
