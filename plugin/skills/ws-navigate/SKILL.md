---
name: ws-navigate
description: >
  Use this skill when the user wants to read, compare, or copy files from
  another worktree. Also use when the user says "compare with main",
  "check the file on another branch", "look at the other worktree",
  or needs to reference files across different worktrees/branches.
allowed-tools: Bash, Read, Glob, Grep
---

# ws cross-worktree navigation

Navigate and compare files across worktrees in a ws-managed bare clone repository.

## Available worktrees

!`ws status 2>/dev/null || echo "(ws status unavailable — run from inside a ws-managed repo)"`

## Path resolution

In a ws-managed repository, worktrees are sibling directories under the repository root:

```
~/Projects/org/repo/         # Repository root
├── main/                    # Worktree: main branch
├── feature-x/               # Worktree: feature-x branch
└── fix-bug/                  # Worktree: fix-bug branch
```

To reference a file in another worktree, resolve the path relative to the repository root:

```bash
# If you're in:     ~/Projects/org/repo/feature-x/src/main.rs
# The same file in main is: ~/Projects/org/repo/main/src/main.rs

# General pattern:
# <repo-root>/<worktree-name>/<relative-path-within-worktree>
```

### Finding the repository root

```bash
# Get the repo root (parent of all worktrees)
git rev-parse --git-common-dir 2>/dev/null | xargs dirname

# Get the current worktree root
git rev-parse --show-toplevel
```

## Operations

### Read a file from another worktree

```bash
# Resolve the target path
REPO_ROOT=$(git rev-parse --git-common-dir 2>/dev/null | xargs dirname)
cat "$REPO_ROOT/<worktree-name>/<relative-path>"
```

Or use the Read tool directly with the resolved absolute path.

### Compare files between worktrees

```bash
# Diff a specific file between current worktree and another
REPO_ROOT=$(git rev-parse --git-common-dir 2>/dev/null | xargs dirname)
CURRENT=$(git rev-parse --show-toplevel)
CURRENT_NAME=$(basename "$CURRENT")

diff "$CURRENT/<relative-path>" "$REPO_ROOT/<other-worktree>/<relative-path>"

# For a richer diff
git diff --no-index "$CURRENT/<relative-path>" "$REPO_ROOT/<other-worktree>/<relative-path>"
```

### Compare entire directories between worktrees

```bash
REPO_ROOT=$(git rev-parse --git-common-dir 2>/dev/null | xargs dirname)

# Compare src/ directories between two worktrees
diff -rq "$REPO_ROOT/main/src" "$REPO_ROOT/feature-x/src"

# Detailed diff
git diff --no-index "$REPO_ROOT/main/src" "$REPO_ROOT/feature-x/src"
```

### Copy a file from another worktree

```bash
REPO_ROOT=$(git rev-parse --git-common-dir 2>/dev/null | xargs dirname)

# Copy a file from another worktree to the current one
cp "$REPO_ROOT/<source-worktree>/<relative-path>" "./<relative-path>"
```

### List files in another worktree

```bash
REPO_ROOT=$(git rev-parse --git-common-dir 2>/dev/null | xargs dirname)

# List files in another worktree's directory
ls "$REPO_ROOT/<worktree-name>/src/"

# Search for a pattern in another worktree
grep -r "pattern" "$REPO_ROOT/<worktree-name>/src/"
```

## Argument handling

When the user invokes `/ws:navigate`:

- **`/ws:navigate compare src/main.rs main`** → Compare `src/main.rs` between current worktree and `main`
- **`/ws:navigate read main src/lib.rs`** → Read `src/lib.rs` from the `main` worktree
- **`/ws:navigate diff main feature-x src/`** → Compare `src/` between `main` and `feature-x`
- **`/ws:navigate`** (no args) → Show available worktrees and ask what the user wants to do

## Tips

- Always use `ws status` to discover available worktrees before navigating
- Remember that store-managed symlink files will be identical across worktrees (no point diffing them)
- When comparing, focus on tracked (non-gitignored) files for meaningful diffs
