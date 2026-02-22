# ws new

Create a worktree and open it in VSCode.

## Usage

```bash
ws new [name] [options]
```

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | No | Workspace name. Auto-generates a random name if omitted (e.g., `gentle-happy-fox`) |

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--directory <path>` | `-d` | Path for the worktree (default: `../<name>` or `<name>`) |
| `--branch <branch>` | | Explicit branch name (default: same as name) |
| `--from <ref>` | | Starting point for the new branch (default: HEAD) |

## Behavior

1. If a branch with the same name already exists, checks it out and creates the worktree
2. If the branch doesn't exist, creates a new branch from `--from` (default: HEAD)
3. If HEAD is invalid (e.g., empty bare repo) and `--from` is not specified, creates an orphan branch
4. If a shared store exists, tracked files are automatically distributed
5. Launches VSCode (`code`) on completion

### Worktree location

- **Bare setup** (`.bare/` exists): creates `<name>/` in the current directory
- **Normal setup** (inside `.git/`): creates `../<name>` in the parent directory

Use the `-d` option to specify a custom path.

## Examples

```bash
# Basic worktree creation
ws new main

# Branch from main
ws new feature/awesome --from main

# Explicit branch name and directory
ws new my-work --branch feature/my-work -d ../my-work-dir

# Random name
ws new
```
