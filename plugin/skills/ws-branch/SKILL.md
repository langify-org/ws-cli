---
name: ws-branch
description: >
  Manage worktrees in a ws-managed repository. Create new worktrees,
  remove existing ones, or switch between them.
disable-model-invocation: true
argument-hint: "[new <name> | rm <dir> | switch <name>]"
allowed-tools: Bash, Read
---

# ws branch management

You are managing worktrees in a **ws-managed bare clone + worktree** repository.

## CRITICAL RULE

**NEVER use `git checkout`, `git switch`, or `git branch` to create or switch branches.** In this workflow, each branch lives in its own worktree directory. Use `ws` commands instead.

## Operations

### Create a new worktree: `new <name>`

Create a new worktree for a branch.

```bash
# Create worktree for a new branch (branching from current HEAD)
ws new <branch-name>

# The new worktree will be created as a sibling directory
# e.g., if you're in ~/Projects/org/repo/main/
#        the new worktree: ~/Projects/org/repo/<branch-name>/
```

After creation:
1. The new worktree directory is created with the branch checked out
2. Store files are automatically distributed (symlinks linked, copies copied)
3. Tell the user the path to the new worktree so they can navigate to it

### Remove a worktree: `rm <dir>`

Remove an existing worktree.

Before removing, always check:
1. **Uncommitted changes**: Run `git -C <worktree-path> status` to check for uncommitted work
2. **Unpushed commits**: Run `git -C <worktree-path> log @{u}..HEAD 2>/dev/null` to check for unpushed commits
3. **Warn the user** if there are uncommitted or unpushed changes and ask for confirmation

```bash
# Remove a worktree
ws rm <directory-name>
```

### Switch to another worktree: `switch <name>`

There is no "switch" command in ws because switching means navigating to a different directory. Guide the user:

1. Run `ws status` to see available worktrees and their paths
2. Tell the user to `cd` to the target worktree directory
3. If the desired branch doesn't have a worktree yet, offer to create one with `ws new`

## Argument handling

When the user invokes `/ws:branch`:

- **`/ws:branch new feature-auth`** → Create worktree for branch `feature-auth`
- **`/ws:branch rm feature-auth`** → Remove the `feature-auth` worktree
- **`/ws:branch switch main`** → Guide navigation to the `main` worktree
- **`/ws:branch`** (no args) → Show `ws status` and ask what the user wants to do

## Error handling

- If `ws new` fails because the branch already exists remotely, suggest `ws new <name>` which will track the remote branch
- If `ws rm` fails because of uncommitted changes, show the changes and ask the user how to proceed
- If the user asks to "checkout" or "switch to" a branch, interpret this as navigating to the corresponding worktree directory
