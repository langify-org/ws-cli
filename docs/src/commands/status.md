# ws status

Display an integrated view of worktrees and shared file status.

## Usage

```bash
ws status
```

## Behavior

Displays two sections:

### Workspaces section

Lists all worktrees. The current worktree is marked with `*`. If a shared store exists, the number of tracked files is also shown.

### Shared files section

Shown only when a shared store exists and files are being tracked. Lists each file's strategy and status.

## Example output

```
Workspaces:
  * /Users/user/my-project/main              [main]  [3 files tracked]
    /Users/user/my-project/feature-foo       [feature/foo]  [3 files tracked]

Shared files:
  STRATEGY FILE                                     STATUS
  -------- ---------------------------------------- ----------
  symlink  .envrc                                   OK
  symlink  .mcp.json                                OK
  copy     .env.local                               MODIFIED
```

## Status values

| Status | Meaning |
|--------|---------|
| `OK` | Normal |
| `MISSING` | File is missing from the worktree |
| `MISSING(store)` | File is missing from the store |
| `MODIFIED` | Copy file differs from the store |
| `NOT_LINK` | File that should be a symlink is a regular file |
| `WRONG_LINK` | Symlink points to the wrong target |
