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
ws repos clone https://github.com/example/repo.git
```

`ws repos clone` runs `git clone --bare <url> .bare` internally.

### Creating worktrees

```bash
ws new feature/foo                 # Create a new branch from HEAD
```

`ws repos clone` automatically creates a worktree for the default branch (e.g. `main`). `ws new` runs `git worktree add` internally.

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

## Repository root resolution

Several `ws` commands (e.g., `ws repos add`, automatic registration on `ws repos clone`) need to determine the **repository root** — the top-level directory that represents the project.

### Resolution rules

| Repository type | How the root is determined | Result |
|---|---|---|
| **Bare worktree** (`ws repos clone`) | `git rev-parse --git-common-dir` → if it ends in `.bare`, use its parent | `my-project/` |
| **Normal clone** | `git rev-parse --show-toplevel` | `my-project/` |

### Example: bare worktree

```
my-project/          ← repository root
├── .bare/
├── main/
│   └── (you are here)
└── feature-foo/
```

Running `ws repos add` from `main/` registers `my-project/`, not `main/`.

### Example: normal clone

```
my-project/          ← repository root
├── .git/
├── src/
│   └── (you are here)
└── ...
```

Running `ws repos add` from `src/` registers `my-project/`.
