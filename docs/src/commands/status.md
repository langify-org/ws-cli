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

## Color output

Output is colorized when connected to a terminal. Colors are automatically stripped when piping to another command or file. The `NO_COLOR` environment variable disables colors.

| Element | Style |
|---------|-------|
| Section header title | Bold |
| Section header ruler (`──`) | Dim |
| Table header | Bold |
| Table separator (`────`) | Dim |
| Status `OK` | Green |
| Status `MISSING`, `MISSING(store)` | Red |
| Status `ERROR` | Red + Bold |
| Status `MODIFIED`, `NOT_LINK`, `WRONG_LINK` | Yellow |
| Current marker `*` | Green + Bold |
| Branch name `[branch]` | Cyan |
| Commit hash | Dim |
| Repository type `bare` | Cyan |
| Repository type `NOT_FOUND` | Red |

## Example output

```
── Repositories ──────────────────────────
  NAME              PATH                                     TYPE
  ────              ────                                     ────
  langify-notebook  ~/Projects/langify-org/langify-notebook  git
  web               ~/Projects/spirinc/web                   bare
* ws-cli            ~/Projects/langify-org/ws-cli            bare

── Current Repository: ws-cli ────────────
  Path: ~/Projects/langify-org/ws-cli
  Worktrees:
    ├──   fix-ci    [fix-ci] fb7eff8
    └── * master    [master] 5b33080

── Current Workspace: master [master] ────
  STRATEGY  FILE       STATUS
  ────────  ────       ──────
  symlink   .claude/settings.local.json  OK
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
