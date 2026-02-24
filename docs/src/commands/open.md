# ws open

Open a registered repository's worktree in an editor.

## Usage

```bash
ws open <repository> <worktree> [options]
```

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `repository` | Yes | Name of a registered repository (as shown in `ws repos list`) |
| `worktree` | Yes | Worktree name (relative path from the repository root) |

## Options

| Option | Description |
|--------|-------------|
| `--editor <command>` | Editor command to use (default: `$VISUAL` or `$EDITOR`) |

## Editor resolution

The editor is resolved in the following order:

1. `--editor` flag (highest priority)
2. `$VISUAL` environment variable
3. `$EDITOR` environment variable
4. Error if none are set

## Examples

```bash
# Open the "main" worktree of "my-repo" in the default editor
ws open my-repo main

# Open with a specific editor
ws open my-repo feature/awesome --editor code

# Use with $EDITOR
EDITOR=vim ws open my-repo main
```

## Interactive mode

In interactive mode (`ws i`), the `open` command provides a guided workflow:

1. Select a repository from the registered list
2. Select a worktree from the repository's worktree list
3. The worktree opens in the resolved editor
