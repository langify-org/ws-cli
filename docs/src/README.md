# ws - workspace & repository manager

> **[日本語版はこちら](ja/)**

**ws** is a CLI tool that manages your system's repositories, worktrees, and shared configuration in one place.

## Why ws?

Working across multiple repositories and branches involves persistent friction. Common pain points include:

- **Tedious bare clone initialization** — After `git clone --bare`, you need to manually add worktrees
- **Managing gitignored files** — Files like `.env` and `.claude/settings.local.json` are outside git, so you must manually copy or link them every time you create a new worktree
- **Scattered repository management** — Repositories live in different directories with no unified way to track or inspect them

ws solves these problems by providing a single CLI for repository registration, worktree management, and gitignored file sharing.

## Features

- **Repository registry** — Register and manage all your repositories with `ws repos` for system-wide visibility
- **Bare clone + worktree management** — One-step setup with just `ws repos clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees (symlink / copy strategies)
- **Quick open** — Open any registered repository's worktree in your editor with `ws open`
- **Interactive mode** — Build and run commands interactively

## Quick Start

```bash
# Bare clone the repository (default branch worktree is created automatically)
ws repos clone https://github.com/example/repo.git

# Create a feature branch
ws new feature/awesome
```

See [Quick Start](getting-started/quick-start.md) for more details.
