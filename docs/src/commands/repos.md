# ws repos

Manage the repository registry. Registered repositories are stored in `~/.config/ws/config.toml`.

> [!TIP]
> If you use Home Manager, you can declaratively manage repositories with `programs.ws.repos` instead of `ws repos add`. See [Installation](../getting-started/installation.md) for details.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| [`ws repos add`](#ws-repos-add) | Register a repository |
| [`ws repos list`](#ws-repos-list) | List registered repositories |
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
