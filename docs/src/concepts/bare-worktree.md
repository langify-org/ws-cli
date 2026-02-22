# Bare Clone + Worktree Pattern

## Overview

The bare clone + worktree pattern is a development style where multiple worktrees are laid out alongside a single bare repository. Unlike a normal `git clone`, this approach uses a bare repository (with no working directory) as the central hub, and each branch is checked out as an independent directory.

## Comparison with a normal clone

### Normal clone

```
my-project/
├── .git/
├── src/
└── ...
```

One directory = one branch. Switching branches requires `git checkout` / `git switch`, and you can't switch if there are uncommitted changes.

### Bare clone + worktree

```
my-project/
├── .bare/              # Bare repository (no working directory)
├── main/               # Worktree for the main branch
│   ├── src/
│   └── ...
└── feature-foo/        # Worktree for the feature/foo branch
    ├── src/
    └── ...
```

Having multiple branches open at once offers several advantages:

- **No branch switching needed** — Each branch lives in its own directory
- **Easy parallel work** — Keep a branch open for review while working on something else
- **Preserved build caches** — Each branch has independent `target/`, `node_modules/`, etc.

## Using ws

### Creating a bare repository

```bash
mkdir my-project && cd my-project
ws clone https://github.com/example/repo.git
```

`ws clone` runs `git clone --bare <url> .bare` internally.

### Creating worktrees

```bash
ws new main                        # Check out an existing branch
ws new feature/foo --from main     # Create a new branch from main
```

`ws new` runs `git worktree add` internally and launches VSCode on completion.

### Removing worktrees

```bash
ws rm feature-foo
```

`ws rm` runs `git worktree remove` internally.

## Worktree naming convention

The name passed to `ws new` becomes both the worktree directory name and the branch name. In directory names, `/` is converted to `-`:

| Name | Directory | Branch |
|------|-----------|--------|
| `main` | `main/` | `main` |
| `feature/foo` | `feature-foo/` | `feature/foo` |

Use the `--branch` option to explicitly set a different branch name.

If you omit the name, a random name is generated automatically (e.g., `gentle-happy-fox`).
