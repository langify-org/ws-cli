# ws repos

Manage the repository registry. Registered repositories are stored in `~/.config/ws/config.toml`.

> [!TIP]
> If you use Home Manager, you can declaratively manage repositories with `programs.ws.repos` instead of `ws repos add`. See [Installation](../getting-started/installation.md) for details.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| [`ws repos clone`](#ws-repos-clone) | Create a bare repository |
| [`ws repos add`](#ws-repos-add) | Register a repository |
| [`ws repos list`](#ws-repos-list) | List registered repositories |
| [`ws repos rm`](#ws-repos-rm) | Unregister a repository |

---

## ws repos clone

Create a bare repository.

### Usage

```bash
ws repos clone [url]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `url` | No | Remote URL. If omitted, creates an empty bare repository |

### Behavior

Creates a `.bare/` directory in the current directory.

- With a URL: runs `git clone --bare <url> .bare`, then automatically creates a worktree for the default branch (e.g. `main` or `master`)
- Without a URL: runs `git init --bare .bare` (no worktree is created since no commits exist)

Fails with an error if `.bare` already exists. The repository is automatically registered in the config.

### Examples

```bash
mkdir my-project && cd my-project
ws repos clone https://github.com/example/repo.git
# .bare/ is created and a worktree for the default branch is set up automatically
```

```bash
mkdir my-project && cd my-project
ws repos clone                  # Create an empty bare repository
ws new master                   # Create a worktree with an orphan branch
```

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
