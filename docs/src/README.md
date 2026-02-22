# ws - workspace manager

> **[日本語版はこちら](../ja/)**

**ws** is a CLI tool that streamlines development with the git bare clone + worktree pattern.

## Why ws?

Git worktree is a powerful feature that lets you work on multiple branches simultaneously, but setting up and managing worktrees involves friction. Common pain points include:

- **Tedious bare clone initialization** — After `git clone --bare`, you need to manually add worktrees
- **Managing gitignored files** — Files like `.envrc`, `.mcp.json`, and `.env.local` are outside git, so you must manually copy or link them every time you create a new worktree
- **Inconsistent directory structure** — Each team member ends up with a different layout

ws solves these problems and makes worktree-based development seamless.

## Features

- **One-step bare clone + worktree setup** — Get started with just `ws clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees (symlink / copy strategies)
- **Interactive mode** — Build and run commands interactively

## Quick Start

```bash
# Bare clone the repository
ws clone https://github.com/example/repo.git

# Create a worktree
ws new main

# Create a feature branch
ws new feature/awesome --from main
```

See [Quick Start](getting-started/quick-start.md) for more details.
