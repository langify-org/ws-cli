---
name: release
description: バージョンバンプ、lint/test、コミット、タグ、push、CI 監視を一括実行
argument-hint: "<version> (例: 0.5.0)"
disable-model-invocation: true
user-invocable: true
---

リリースワークフローを一括実行する。**ユーザー確認なしにコミット・タグ・push を行ってはならない。**

## 手順

### 1. 引数バリデーション

`$ARGUMENTS` が `X.Y.Z`（セマンティックバージョニング）形式であることを検証する。

- 正規表現: `^[0-9]+\.[0-9]+\.[0-9]+$`
- 不正な場合は以下を表示して **終了**:
  ```
  Usage: /release <version>
  Example: /release 0.5.0
  ```

以降、引数のバージョンを `{VERSION}` と表記する。

### 2. 事前チェック

以下を **順に** 実行する。1 つでも失敗したらエラー内容を表示して **中断** する。

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
LC_ALL=en_US.UTF-8 cargo test
mdbook build docs
mdbook build docs/ja
```

### 3. 現在バージョン確認

以下の 4 箇所からバージョンを読み取り、**全て一致しているか** 検証する:

1. `crates/ws-cli/Cargo.toml` — `version = "..."` フィールド
2. `crates/ws-core/Cargo.toml` — `version = "..."` フィールド
3. `flake.nix` — `commonArgs` ブロック内の `version = "...";`（約39行目）
4. `flake.nix` — `ws-docs` ブロック内の `version = "...";`（約58行目）

不一致がある場合はどこが異なるか表示して **中断** する。

現在バージョンを `{CURRENT}` と表記する。
`{CURRENT}` と `{VERSION}` が同じ場合は「既に同じバージョンです」と表示して **中断** する。

### 4. バージョン書き換え

上記 4 箇所のバージョンを `{VERSION}` に更新する。

更新後、`Cargo.lock` を同期するために以下を実行:

```bash
cargo check
```

### 5. ユーザー確認

以下を実行して差分を表示する:

```bash
git diff
```

差分を表示した上で、ユーザーに承認を求める。
**承認が得られるまで絶対に次のステップに進んではならない。**

### 6. コミット & タグ & プッシュ

承認を得たら以下を **順に** 実行する:

```bash
git add crates/ws-cli/Cargo.toml crates/ws-core/Cargo.toml flake.nix Cargo.lock
git commit -m "chore: bump version to v{VERSION}"
git tag -a "v{VERSION}" -m "v{VERSION}"
git push && git push origin "v{VERSION}"
```

### 7. CI 監視

push 完了後、以下を実行して CI の状態を表示する:

```bash
gh run list --limit 3
```

リリースワークフローの状態をユーザーに報告する。
