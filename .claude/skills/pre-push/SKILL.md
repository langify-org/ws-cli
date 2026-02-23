---
name: pre-push
description: push 前の品質チェック（fmt, clippy, test, docs build）を実行
disable-model-invocation: true
user-invocable: true
---

push 前の品質チェックを順に実行する。各ステップの結果を記録し、最後にサマリーテーブルを表示する。

## 手順

以下の 5 つのチェックを **順番に** 実行する。

### 1. cargo fmt --check

```bash
cargo fmt --check
```

- **成功**: 次へ進む
- **失敗**: `cargo fmt` を実行して自動修正し、修正した旨をユーザーに報告してから次へ進む

### 2. cargo clippy

```bash
cargo clippy --all-targets -- -D warnings
```

- 失敗したら警告内容を表示して記録する（中断はしない）

### 3. cargo test

```bash
LC_ALL=en_US.UTF-8 cargo test
```

- 失敗したらテスト結果を表示して記録する（中断はしない）

### 4. mdbook build docs

```bash
mdbook build docs
```

- 失敗したらエラー内容を表示して記録する（中断はしない）

### 5. mdbook build docs/ja

```bash
mdbook build docs/ja
```

- 失敗したらエラー内容を表示して記録する（中断はしない）

## 結果サマリー

全チェック完了後、以下の形式でテーブルを表示する:

| Check | Result |
|-------|--------|
| fmt | PASS / FIXED |
| clippy | PASS / FAIL |
| test | PASS / FAIL |
| docs (en) | PASS / FAIL |
| docs (ja) | PASS / FAIL |

全て PASS なら「All checks passed. Ready to push.」と表示する。
1 つでも FAIL があれば「Some checks failed. Please fix before pushing.」と表示する。
