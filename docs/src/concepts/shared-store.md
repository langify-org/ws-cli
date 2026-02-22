# Shared Store

## Overview

The shared store is a mechanism for centrally managing gitignored files across worktrees. By registering files like `.envrc`, `.mcp.json`, and `.env.local` in the store, they are automatically distributed when new worktrees are created.

## Store structure

The store is located at `<git-common-dir>/worktree-store/`. In a bare setup, this is `.bare/worktree-store/`.

```
.bare/worktree-store/
├── manifest         # Line format: "strategy:filepath"
├── .mcp.json        # Master copy
├── .envrc           # Master copy
└── .env.local       # Master copy
```

### Manifest

The manifest is a text file that records each tracked file and its strategy, one per line.

```
symlink:.envrc
symlink:.mcp.json
copy:.env.local
```

## Strategies

The shared store supports two distribution strategies.

### symlink

Creates symbolic links in worktrees pointing to the file in the store.

```bash
ws shared track -s symlink .envrc
```

- **All worktrees share the same content** — Editing the store file is reflected across all worktrees
- On `track`, the existing file is moved to the store and replaced with a symbolic link

**Use for:** `.envrc`, `.mcp.json`, and other config files shared across all worktrees

### copy

Copies files from the store into worktrees.

```bash
ws shared track -s copy .env.local
```

- **Each worktree can be customized independently** — After copying, each worktree can edit the file freely
- Use `ws shared push` to write changes back to the store, and `ws shared pull` to fetch from the store

**Use for:** `.env.local` and other files that need different values per worktree

### Strategy comparison

| | symlink | copy |
|---|---------|------|
| Distribution method | Symbolic link | File copy |
| Content sharing | Identical across all worktrees | Independent per worktree |
| Update propagation | Instant (same link target) | Requires `push` / `pull` |
| Use case | Common config files | Environment-specific files |

## Workflow

### Initial setup

Register files you want to track from inside a worktree.

```bash
ws shared track -s symlink .envrc
ws shared track -s symlink .mcp.json
ws shared track -s copy .env.local
```

The store is automatically initialized on the first `ws shared track` invocation.

### Creating new worktrees

When you run `ws new`, tracked files are automatically distributed from the store.

```bash
ws new feature/bar
# → .envrc (symlink), .mcp.json (symlink), .env.local (copy) are distributed from the store
```

### Checking status

```bash
ws shared status
```

Displays the status of each tracked file:

| Status | Meaning |
|--------|---------|
| `OK` | Normal |
| `MISSING` | File is missing from the worktree |
| `MISSING(store)` | File is missing from the store |
| `MODIFIED` | Copy file differs from the store |
| `NOT_LINK` | File that should be a symlink is a regular file |
| `WRONG_LINK` | Symlink points to the wrong target |

### Syncing copy files

```bash
# Push worktree changes to the store
ws shared push
ws shared push .env.local          # Specific file only

# Pull from store to worktree
ws shared pull
ws shared pull .envrc              # Specific file only
ws shared pull -f                  # Overwrite existing files
```
