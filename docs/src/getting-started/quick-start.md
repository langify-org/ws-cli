# Quick Start

## Bare setup (Recommended)

### 1. Bare clone

```bash
mkdir my-project && cd my-project
ws clone https://github.com/example/repo.git
```

A bare repository is created in the `.bare/` directory.

### 2. Create a worktree

```bash
ws new main
```

A worktree for the `main` branch is created and VSCode opens automatically.

### 3. Work on a feature branch

```bash
ws new feature/foo --from main
```

A worktree for a new `feature/foo` branch is created, branching from `main`.

### Resulting directory structure

```
my-project/
├── .bare/                         # Bare repository
│   └── worktree-store/            # Shared store (optional)
├── main/                          # Worktree
└── feature-foo/                   # Worktree
```

## Normal setup

You can also use `ws new` inside an existing `git clone` repository. Worktrees are created in the parent directory of the repository.

```
parent/
├── my-project/                    # Normal git repository (with .git/)
└── feature-foo/                   # Worktree (created at ../<name>)
```

## Bare setup without a URL

To start a new project without a remote:

```bash
mkdir my-project && cd my-project
ws clone                            # Create an empty bare repository
ws new master                      # Create a worktree with an orphan branch
```

## Setting up shared files

To share `.envrc` or `.mcp.json` across worktrees:

```bash
# Run inside a worktree
ws shared track -s symlink .envrc
ws shared track -s symlink .mcp.json
ws shared track -s copy .env.local
```

From now on, every time you create a new worktree with `ws new`, these files are automatically distributed.

See [Shared Store](../concepts/shared-store.md) for more details.
