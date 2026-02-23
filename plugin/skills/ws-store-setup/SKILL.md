---
name: ws-store-setup
description: >
  Analyze the project's gitignored files and set up the ws store with
  appropriate tracking strategies. Guides initial store configuration.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep
---

# ws store setup

Guide the user through setting up the ws shared store for their repository. The store ensures that gitignored files (like `.env`, editor configs, local settings) are automatically shared across worktrees.

## Step 1: Analyze .gitignore

Read the project's `.gitignore` file(s) to identify candidates for the store:

```bash
# Read gitignore at repo root (worktree level)
cat .gitignore 2>/dev/null

# Also check for nested gitignore files
find . -name .gitignore -not -path './.bare/*' -not -path './worktree-store/*' 2>/dev/null
```

## Step 2: Find existing gitignored files

For each gitignored pattern, check which files actually exist in the current worktree:

```bash
# List files that exist but are gitignored
git ls-files --others --ignored --exclude-standard
```

Focus on files that are:
- Configuration files (`.env`, `.env.local`, `.env.development`)
- Editor/tool settings (`.vscode/settings.json`, `.idea/`, `.claude/settings.local.json`)
- Local overrides that developers typically maintain per-project

Ignore:
- Build artifacts (`node_modules/`, `target/`, `dist/`, `__pycache__/`)
- Cache directories (`.cache/`, `.turbo/`, `.next/`)
- OS files (`.DS_Store`, `Thumbs.db`)
- Log files (`*.log`)

## Step 3: Determine strategy for each file

Present a table of recommended files and strategies to the user:

### Strategy guidelines

| Strategy | When to use | Examples |
|----------|-------------|---------|
| **symlink** | File should be identical across all worktrees. Editing in one worktree should propagate everywhere. | `.claude/settings.local.json`, `.vscode/settings.json`, `.editorconfig` (local) |
| **copy** | File may need worktree-specific values. Each worktree gets its own independent copy. | `.env`, `.env.local`, `.env.development.local` |

### Decision rules

1. **Environment files** (`.env*`) → Almost always **copy** (different worktrees may run on different ports or have different feature flags)
2. **Editor/IDE settings** → Usually **symlink** (you want consistent editor behavior)
3. **Claude/AI settings** → Usually **symlink** (consistent AI tool behavior)
4. **Credentials/tokens** → Usually **copy** (may differ per environment/branch)
5. **Local config overrides** → Depends on the file; ask the user if unsure

## Step 4: Get user confirmation

Present the proposed configuration as a clear table:

```
File                              Strategy    Reason
.env                              copy        Environment-specific values
.env.local                        copy        Local overrides
.claude/settings.local.json       symlink     Shared AI tool settings
.vscode/settings.json             symlink     Consistent editor config
```

Ask the user to:
1. Review and approve/modify the list
2. Confirm strategies for any files you're unsure about
3. Add any files they want that weren't auto-detected

## Step 5: Execute tracking

After user confirmation, track each file:

```bash
# Track each file with its strategy
ws store track .env --strategy copy
ws store track .claude/settings.local.json --strategy symlink
# ... repeat for each file

# Push store files to all existing worktrees
ws store push
```

## Step 6: Verify

Run `ws store status` to verify everything was set up correctly:

```bash
ws store status
```

Check that all files show `OK` status. If any show `MISSING` or `ERROR`, troubleshoot:
- `MISSING`: File doesn't exist in the current worktree yet — may need to create it first, then `ws store pull`
- `MISSING(store)`: File exists in worktree but not in store — run `ws store pull <file>` to import it
- `NOT_LINK`: Strategy is symlink but file is a regular file — `ws store push` should fix this
