# ws status

Display an integrated status dashboard covering repositories, the current workspace, and shared files.

## Usage

```bash
ws status
```

## Behavior

Displays up to three sections depending on context:

### Repositories section

Shown when repositories are registered in `~/.config/ws/config.toml`. Lists each repository with its path, GIT_DIR type, and worktree tree. The current repository (if any) is marked with `*`.

### Current workspace section

Shown when running inside a worktree. Displays the current worktree path, branch, and tracked file count.

### Shared files section

Shown when a shared store exists and files are being tracked. Lists each file's strategy and status.

When running outside any repository with no registered repositories, a "No registered repositories" message is shown.

## Example output

```
Repositories:
  web
    Path: /Users/user/projects/web
    GIT_DIR: .bare
    Worktrees:
      └── release   [release] 9946e77

* my-project
    Path: /Users/user/projects/my-project
    GIT_DIR: .bare
    Worktrees:
      ├── main   [main] abc1234
      └── feature-foo   [feature/foo] def5678

Current workspace:
  * /Users/user/projects/my-project/main [main]  [3 files tracked]

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
