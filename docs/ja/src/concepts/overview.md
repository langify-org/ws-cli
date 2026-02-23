# 概念の全体像

## 4 つのコアコンセプト

ws は 4 つの概念を中心に開発ワークフローを構成します。2 つは git ネイティブ、2 つは ws 独自のものです。

| 概念 | git ネイティブ? | ws が追加するもの |
|------|---------------|-----------------|
| **Registry** | No | `~/.config/ws/config.toml` によるシステム全体のリポジトリカタログ |
| **Repository** | Yes (bare / normal) | Registry への登録、`ws repos clone` による bare clone の簡易化 |
| **Workspace** (= worktree) | Yes (`git worktree`) | 命名規則（`/` → `-`）、作成時の Store 自動配布、ライフサイクル管理 |
| **Store** | No | Repository 内で gitignored ファイルを Workspace 間に共有するメカニズム |

## 概念の関係

Registry、Repository、Workspace は**包含関係の階層**を形成します — それぞれが上位の概念の中にネストします。Store はこの階層のレイヤーではなく、Repository 内に存在し Workspace にファイルを配布する**横断的なメカニズム**です。

```
┌─ Registry ──────────────────────────────────────────────────────┐
│  ~/.config/ws/config.toml                                       │
│                                                                 │
│  ┌─ Repository ─────────────────────────────────────────────┐   │
│  │  my-project/                                             │   │
│  │                                                          │   │
│  │  ┌─ .bare/ ──────────────────────────────────────────┐   │   │
│  │  │  objects/, refs/, ...          (git data)         │   │   │
│  │  │                                                   │   │   │
│  │  │  ┌─ Store ─────────────────────────────────────┐  │   │   │
│  │  │  │  worktree-store/                            │  │   │   │
│  │  │  │  ├── manifest                               │  │   │   │
│  │  │  │  ├── .mcp.json    (マスターコピー)            │  │   │   │
│  │  │  │  └── .env         (マスターコピー)            │  │   │   │
│  │  │  └─────────────────────────────────────────────┘  │   │   │
│  │  └───────────────────────────────────────────────────┘   │   │
│  │                    │ 配布                                 │   │
│  │          ┌─────────┼─────────┐                           │   │
│  │          ▼         ▼         ▼                           │   │
│  │  ┌─ Workspace ┐ ┌─ Workspace ┐ ┌─ Workspace ┐          │   │
│  │  │ main/      │ │ feature-a/ │ │ fix-bug/    │          │   │
│  │  │ .mcp.json→ │ │ .mcp.json→ │ │ .mcp.json→  │          │   │
│  │  │  (symlink) │ │  (symlink) │ │  (symlink)  │          │   │
│  │  │ .env       │ │ .env       │ │ .env        │          │   │
│  │  │  (copy)    │ │  (copy)    │ │  (copy)     │          │   │
│  │  └────────────┘ └────────────┘ └─────────────┘          │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─ Repository ──────────┐                                      │
│  │  another-project/     │                                      │
│  │  ...                  │                                      │
│  └───────────────────────┘                                      │
└─────────────────────────────────────────────────────────────────┘
```

**ポイント:**

- Registry → Repository → Workspace は包含関係（入れ子）
- Store は Repository 内（`.bare/` 内）に存在し、Workspace にファイルを配布する
- Store は「レイヤー」ではなく「メカニズム」として描かれている

## コマンドとの対応

各 ws コマンドは、これらの概念のいずれかを操作対象とします:

```
ws repos clone/add/list/rm  ─── Registry + Repository
ws new / ws rm              ─── Workspace
ws store track/status/...   ─── Store（Repository 内のメカニズム）
ws status                   ─── 全概念の統合ビュー
ws i                        ─── 対話モード（全コマンドへのエントリポイント）
```

## 典型的なワークフロー

以下の例で、4 つの概念が初期セットアップから日常の利用までどう連携するかを示します。

### 1. リポジトリのクローン（Registry + Repository）

```bash
ws repos clone https://github.com/example/my-project.git
```

これにより 3 つのことが行われます:
- `my-project/.bare/` に bare clone を作成
- デフォルトブランチ（例: `main/`）の Workspace を作成
- Repository を Registry に登録

### 2. 共有ファイルの設定（Store）

`main/` Workspace 内で、共有したいファイルを登録します:

```bash
ws store track -s symlink .envrc
ws store track -s copy .env
```

Store は最初の `track` コマンドで自動的に初期化されます。

### 3. 新しい Workspace の作成（Workspace + Store）

```bash
ws new feature/auth
```

ws は `feature-auth/` Workspace を作成し、Store からトラック済みファイルを自動的に配布します — `.envrc` は symlink として、`.env` は copy として。

### 4. 全体の状態確認（全概念）

```bash
ws status
```

以下をカバーする統合ダッシュボードを表示します:
- Registry に登録された **Repositories**
- 現在の **Workspace** 情報
- Store からの **共有ファイル** の状態

### 5. クリーンアップ（Workspace）

```bash
ws rm feature-auth
```

Workspace を削除します。Store のマスターコピーは将来の Workspace のためにそのまま残ります。

## 詳細

- [bare clone + worktree パターン](bare-worktree.md) — Repository と Workspace の構造の詳細
- [共有ストア](shared-store.md) — Store の strategy と操作の詳細
