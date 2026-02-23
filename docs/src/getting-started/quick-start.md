# Quick Start

## Already have a cloned repository?

You can use `ws new` inside any existing `git clone` repository. Worktrees are created in the parent directory of the repository.

```bash
cd my-project
ws new feature/foo
```

```
parent/
├── my-project/                    # Your existing repository (with .git/)
└── feature-foo/                   # Worktree (created at ../<name>)
```

## Cloning a new repository?

`ws repos clone` creates a bare repository and automatically sets up a worktree for the default branch.

```bash
mkdir my-project && cd my-project
ws repos clone https://github.com/example/repo.git
```

Then create worktrees for feature branches:

```bash
ws new feature/foo
```

### Directory structure

```
my-project/
├── .bare/                         # Bare repository
│   └── worktree-store/            # Shared store (optional)
├── main/                          # Worktree (default branch)
└── feature-foo/                   # Worktree
```

## Starting a new project from scratch?

To start a new project without a remote:

```bash
mkdir my-project && cd my-project
ws repos clone               # Create an empty bare repository
ws new main                  # Create a worktree with an orphan branch
```

## Setting up shared files

To share `.env` or `.claude/settings.local.json` across worktrees:

```bash
# Run inside a worktree
ws store track -s symlink .claude/settings.local.json
ws store track -s copy .env
```

From now on, every time you create a new worktree with `ws new`, these files are automatically distributed.

See [Shared Store](../concepts/shared-store.md) for more details.
