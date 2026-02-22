# ws shared

Centrally manage gitignored files shared across worktrees.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| [`ws shared track`](#ws-shared-track) | Register a file in the store |
| [`ws shared status`](#ws-shared-status) | Show shared file status |
| [`ws shared push`](#ws-shared-push) | Push copy file changes to the store |
| [`ws shared pull`](#ws-shared-pull) | Distribute tracked files from the store |

For details on how the shared store works, see [Shared Store](../concepts/shared-store.md).

---

## ws shared track

Register a file in the store and start tracking it.

### Usage

```bash
ws shared track -s <strategy> <file>
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
ws shared track -s symlink .envrc
ws shared track -s copy .env.local
```

---

## ws shared status

Display the status of all shared files.

### Usage

```bash
ws shared status
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

## ws shared push

Push changes to copy-tracked files back to the store.

### Usage

```bash
ws shared push [file]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `file` | No | File path. If omitted, pushes all copy files |

### Examples

```bash
ws shared push              # Push all copy files
ws shared push .env.local   # Specific file only
```

---

## ws shared pull

Distribute tracked files from the store to the current worktree.

### Usage

```bash
ws shared pull [file] [-f]
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
ws shared pull              # Pull all tracked files
ws shared pull .envrc       # Specific file only
ws shared pull -f           # Overwrite existing files
```
