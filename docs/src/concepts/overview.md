# Conceptual Overview

## Four core concepts

ws organizes development around four concepts. Two are native to git; two are introduced by ws.

| Concept | git native? | What ws adds |
|---------|-------------|--------------|
| **Registry** | No | A system-wide catalog of repositories, stored in `~/.config/ws/config.toml` |
| **Repository** | Yes (bare / normal) | Registration in the Registry, simplified bare cloning via `ws repos clone` |
| **Workspace** (= worktree) | Yes (`git worktree`) | Naming conventions (`/` → `-`), automatic Store distribution on creation, lifecycle management |
| **Store** | No | A mechanism inside a Repository that shares gitignored files across Workspaces |

## How they relate

Registry, Repository, and Workspace form a **containment hierarchy** — each nests inside the one above it. Store is not a layer in this hierarchy; it is a **cross-cutting mechanism** that lives inside a Repository and distributes files into Workspaces.

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
│  │  │  │  ├── .mcp.json    (master copy)             │  │   │   │
│  │  │  │  └── .env         (master copy)             │  │   │   │
│  │  │  └─────────────────────────────────────────────┘  │   │   │
│  │  └───────────────────────────────────────────────────┘   │   │
│  │                    │ distribute                           │   │
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

**Key points:**

- Registry → Repository → Workspace is a containment relationship (nesting)
- Store resides inside a Repository (within `.bare/`) and distributes files into Workspaces
- Store is depicted as a **mechanism**, not a layer

## Command mapping

Each ws command operates on one or more of these concepts:

```
ws repos clone/add/list/rm  ─── Registry + Repository
ws new / ws rm              ─── Workspace
ws store track/status/...   ─── Store (mechanism within a Repository)
ws status                   ─── Unified view across all concepts
ws i                        ─── Interactive mode (entry point to all commands)
```

## Typical workflow

The following example shows how the four concepts work together from initial setup to daily use.

### 1. Clone a repository (Registry + Repository)

```bash
ws repos clone https://github.com/example/my-project.git
```

This does three things:
- Creates a bare clone at `my-project/.bare/`
- Creates a Workspace for the default branch (e.g. `main/`)
- Registers the Repository in the Registry

### 2. Set up shared files (Store)

Inside the `main/` Workspace, register files that should be shared:

```bash
ws store track -s symlink .envrc
ws store track -s copy .env
```

The Store is initialized automatically on the first `track` command.

### 3. Create a new Workspace (Workspace + Store)

```bash
ws new feature/auth
```

ws creates the `feature-auth/` Workspace and automatically distributes tracked files from the Store — `.envrc` as a symlink, `.env` as a copy.

### 4. Check overall status (all concepts)

```bash
ws status
```

Displays a unified dashboard covering:
- **Repositories** registered in the Registry
- **Current Workspace** information
- **Shared files** status from the Store

### 5. Clean up (Workspace)

```bash
ws rm feature-auth
```

Removes the Workspace. The Store's master copies remain intact for future Workspaces.

## Further reading

- [Bare Clone + Worktree Pattern](bare-worktree.md) — Details on Repository and Workspace structure
- [Shared Store](shared-store.md) — Details on Store strategies and operations
