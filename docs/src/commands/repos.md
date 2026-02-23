# ws repos

Manage the repository registry. Registered repositories are stored in `~/.config/ws/config.toml`.

> [!TIP]
> If you use Home Manager, you can declaratively manage repositories with `programs.ws.repos` instead of `ws repos add`. See [Installation](../getting-started/installation.md) for details.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| [`ws repos add`](#ws-repos-add) | Register a repository |
| [`ws repos list`](#ws-repos-list) | List registered repositories |
| [`ws repos status`](#ws-repos-status) | Show detailed status of all registered repositories |
| [`ws repos rm`](#ws-repos-rm) | Unregister a repository |

---

## ws repos add

Register a repository in the registry.

### Usage

```bash
ws repos add [path] [--name <name>]
```

### Arguments and options

| Name | Required | Description |
|------|----------|-------------|
| `path` | No | Path to the repository. Defaults to the current directory |
| `--name <name>` | No | Display name. Defaults to the directory name |

### Behavior

1. Resolves the path to the [repository root](../concepts/bare-worktree.md#repository-root-resolution) (works from any worktree or subdirectory)
2. Validates the path (must be a git repository)
3. Auto-detects the remote URL from `origin`
4. Adds the entry to `~/.config/ws/config.toml`

Duplicate names are rejected.

### Examples

```bash
ws repos add                              # Register the current directory
ws repos add ~/projects/my-repo           # Register a specific path
ws repos add . --name my-app              # Register with a custom name
```

---

## ws repos list

List all registered repositories.

### Usage

```bash
ws repos list
```

### Example output

```
my-repo              /Users/user/projects/my-repo (git@github.com:user/my-repo.git)
another              /Users/user/projects/another
```

---

## ws repos status

Show detailed status of all registered repositories, including GIT_DIR type and worktree tree.

### Usage

```bash
ws repos status
```

### Behavior

For each registered repository, displays:

1. **Repository name and path**
2. **GIT_DIR** — `.bare` for bare worktree pattern, `.git` for normal clones
3. **Worktree tree** — Lists all worktrees with their branch and commit hash

If a registered repository's path no longer exists, `NOT_FOUND` is displayed.

### Example output

**Bare worktree pattern:**

```
my-project (/Users/user/projects/my-project)
  GIT_DIR: .bare
  Worktrees:
    ├── main   [main] abc1234
    ├── feature-foo   [feature/foo] def5678
    └── fix-bar   [fix/bar] 9ab0123
```

**Normal clone:**

```
another-repo (/Users/user/projects/another-repo)
  GIT_DIR: .git
  Main worktree:
    .   [main] abc1234
  Linked worktrees:
    └── ../another-repo-feature   [feature/x] def5678
```

**Missing repository:**

```
old-repo (/Users/user/projects/old-repo)
  NOT_FOUND
```

### Status values

| Value | Description |
|-------|-------------|
| `GIT_DIR: .bare` | Repository uses the bare worktree pattern |
| `GIT_DIR: .git` | Repository is a normal git clone |
| `NOT_FOUND` | Registered path does not exist on disk |

---

## ws repos rm

Unregister a repository from the registry.

### Usage

```bash
ws repos rm <name>
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | Yes | Name of the repository to unregister |

### Examples

```bash
ws repos rm my-repo
```
