# ws status

Display an integrated status dashboard covering repositories, the current repository, and the current workspace.

## Usage

```bash
ws status
```

## Behavior

Displays up to three sections depending on context:

### Repositories section

Shown when repositories are registered in `~/.config/ws/config.toml`. Displays a table with each repository's name, path (abbreviated with `~`), and type (`bare` or `git`). The current repository (if any) is marked with `*`.

### Current Repository section

Shown when running inside a git repository (even if not registered in the config). Displays the repository name, path, and a tree view of all worktrees. The current worktree is marked with `*`.

### Current Workspace section

Shown when running inside a worktree that has a shared store with tracked files. Displays a table of each tracked file's strategy and status.

When running outside any repository with no registered repositories, a "No registered repositories" message is shown.

## Example output

```
Repositories:
  NAME              PATH                                     TYPE
  ────              ────                                     ────
  langify-notebook  ~/Projects/langify-org/langify-notebook  git
  web               ~/Projects/spirinc/web                   bare
* ws-cli            ~/Projects/langify-org/ws-cli            bare

Current Repository: ws-cli
  Path: ~/Projects/langify-org/ws-cli
  Worktrees:
    ├──   fix-ci    [fix-ci] fb7eff8
    └── * master    [master] 5b33080

Current Workspace: master [master]
  STRATEGY  FILE       STATUS
  ────────  ────       ──────
  symlink   .mcp.json  OK
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
