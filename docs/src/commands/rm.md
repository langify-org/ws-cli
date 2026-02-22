# ws rm

Remove a worktree.

## Usage

```bash
ws rm <path> [-f]
```

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `path` | Yes | Path of the worktree to remove |

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--force` | `-f` | Force removal even with uncommitted changes |

## Behavior

Runs `git worktree remove <path>` internally. If `-f` is specified, the `--force` flag is passed to git.

Attempting to remove a worktree with uncommitted changes without `-f` results in an error.

## Examples

```bash
# Remove a worktree
ws rm feature-foo

# Force removal
ws rm feature-foo -f
```
