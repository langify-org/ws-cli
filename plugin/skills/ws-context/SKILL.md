---
name: ws-context
description: >
  Use this skill when the user asks about the current workspace, branch, worktree,
  repository state, or when you need to understand the git bare clone + worktree
  structure of the project. Also use when the user asks "which branch am I on",
  "what worktrees exist", "show workspace status", or any question about the
  project's git/worktree layout.
---

# ws workspace context

You are working inside a repository managed by **ws** — a CLI tool for **git bare clone + worktree** workflows. This is NOT a normal `git clone`. Understand the structure before taking any action.

## Current workspace state

### Repository list

!`ws repos list 2>/dev/null || echo "(ws repos list unavailable — ws may not be installed or this is not a ws-managed repo)"`

### Repository status

!`ws status 2>/dev/null || echo "(ws status unavailable)"`

### Store status

!`ws store status 2>/dev/null || echo "(ws store status unavailable)"`

## Key concepts: bare clone + worktree pattern

In a ws-managed repository, the directory structure looks like this:

```
~/Projects/org/repo/          # Repository root (bare clone)
├── .bare/                    # Bare git data (equivalent of .git/)
├── .git                      # File (not directory!) pointing to .bare/
├── worktree-store/           # Shared file store managed by ws
│   ├── manifest              # Tracks which files are stored and their strategy
│   └── files/                # Master copies of stored files
├── main/                     # Worktree for the main branch
│   ├── .git                  # File pointing to ../../.bare/worktrees/main
│   ├── src/
│   └── ...
├── feature-x/                # Worktree for feature-x branch
│   ├── .git
│   ├── src/
│   └── ...
└── fix-bug/                  # Worktree for fix-bug branch
```

### Critical rules

1. **NEVER suggest `git checkout` or `git switch`** to change branches. In a worktree workflow, each branch has its own directory. To work on a different branch, navigate to its worktree directory or create a new worktree with `ws new`.

2. **NEVER suggest `git clone`** to set up a repository. Use `ws clone` instead, which sets up the bare + worktree structure automatically.

3. **Symlink files in the store are shared.** If a file's store strategy is `symlink`, editing it in one worktree changes it in ALL worktrees. Check `ws store status` before editing store-managed files.

4. **The current working directory determines the active worktree.** There is no "current branch" to switch — you are always in the directory of the branch you're working on.

## Store file handling

Files managed by the ws store have two strategies:

- **symlink**: The file is a symbolic link to the master copy in `worktree-store/`. Editing it affects ALL worktrees. Typical use: `.claude/settings.local.json`, shared config files.
- **copy**: The file is an independent copy in each worktree. Editing it affects only the current worktree. Typical use: `.env`, `.env.local`.

When you encounter a store-managed file:
- Run `ws store status` to check the strategy
- If the file is a **symlink**, warn the user before editing that changes will propagate to all worktrees
- If the user wants worktree-specific changes to a symlinked file, suggest changing the strategy to `copy` with `ws store untrack` + `ws store track --strategy copy`

## Common operations

| Task | Command |
|------|---------|
| See all worktrees | `ws status` |
| Create new worktree | `ws new <branch-name>` |
| Remove a worktree | `ws rm <directory>` |
| List registered repos | `ws repos list` |
| Check store status | `ws store status` |
| Track a file in store | `ws store track <file> --strategy <symlink\|copy>` |
| Push store to worktrees | `ws store push` |
| Pull file into store | `ws store pull <file>` |
