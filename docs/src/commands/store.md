# ws store

Centrally manage gitignored files shared across worktrees.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| [`ws store track`](#ws-store-track) | Register a file in the store |
| [`ws store status`](#ws-store-status) | Show shared file status |
| [`ws store push`](#ws-store-push) | Push copy file changes to the store |
| [`ws store pull`](#ws-store-pull) | Distribute tracked files from the store |
| [`ws store untrack`](#ws-store-untrack) | Unregister a file from the store |

For details on how the shared store works, see [Shared Store](../concepts/shared-store.md).

---

## ws store track

Register a file in the store and start tracking it.

### Usage

```bash
ws store track -s <strategy> <file>
```

### Arguments and options

| Name | Required | Description |
|------|----------|-------------|
| `file` | Yes | File path to track |
| `-s <strategy>` | Yes | `symlink` or `copy` |

### Behavior

1. Copies the file to the store
2. Appends `strategy:filepath` to the manifest
3. If the strategy is `symlink`, deletes the original file and replaces it with a symbolic link to the store

### Examples

```bash
ws store track -s symlink .envrc
ws store track -s copy .env.local
```

---

## ws store status

Display the status of all shared files.

### Usage

```bash
ws store status
```

### Example output

```
Store: /Users/user/my-project/.bare/worktree-store

STRATEGY FILE                                     STATUS
-------- ---------------------------------------- ----------
symlink  .envrc                                   OK
symlink  .mcp.json                                OK
copy     .env.local                               MODIFIED
```

---

## ws store push

Push changes to copy-tracked files back to the store.

### Usage

```bash
ws store push [file]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `file` | No | File path. If omitted, pushes all copy files |

### Examples

```bash
ws store push              # Push all copy files
ws store push .env.local   # Specific file only
```

---

## ws store pull

Distribute tracked files from the store to the current worktree.

### Usage

```bash
ws store pull [file] [-f]
```

### Arguments and options

| Name | Required | Description |
|------|----------|-------------|
| `file` | No | File path. If omitted, pulls all tracked files |
| `-f` | No | Overwrite existing files |

### Behavior

- symlink files: creates a symbolic link to the store
- copy files: copies the file from the store
- Existing files are skipped unless `-f` is specified

### Examples

```bash
ws store pull              # Pull all tracked files
ws store pull .envrc       # Specific file only
ws store pull -f           # Overwrite existing files
```

---

## ws store untrack

Unregister a file from the store and stop tracking it.

### Usage

```bash
ws store untrack <file>
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `file` | Yes | File path to untrack |

### Behavior

1. If the file uses the `symlink` strategy, symbolic links in all worktrees are restored to regular files (copied from the store)
2. Removes the entry from the manifest
3. Deletes the master copy from the store

### Examples

```bash
ws store untrack .envrc
ws store untrack .env.local
```
