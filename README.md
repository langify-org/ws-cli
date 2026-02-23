# ws - workspace (git worktree) manager

> **[日本語版はこちら](README.ja.md)**

A CLI tool that streamlines development with the git bare clone + worktree pattern.

## Why ws?

Git worktree is a powerful feature that lets you work on multiple branches simultaneously, but setting up and managing worktrees involves friction:

- **Tedious bare clone initialization** — After `git clone --bare`, you need to manually add worktrees
- **Managing gitignored files** — Files like `.envrc`, `.mcp.json`, `.env`, and `.env.local` are outside git, so you must manually copy or link them every time you create a new worktree

ws solves these problems and makes worktree-based development seamless.

## Features

- **One-step bare clone + worktree setup** — Get started with `ws clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees (symlink / copy strategies)
- **Interactive mode** — Build and run commands interactively

## Bare Clone + Worktree Pattern

Unlike a normal `git clone`, this approach uses a bare repository (with no working directory) as the central hub, and each branch is checked out as an independent directory.

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

> [!TIP]
> See [Bare Clone + Worktree Pattern](https://langify-org.github.io/ws-cli/concepts/bare-worktree.html) for details on naming conventions and more.

## Shared Store

The shared store centrally manages gitignored files across worktrees. Register files once, and they are automatically distributed when new worktrees are created with `ws new`.

```bash
# Register files to the store
ws store track -s symlink .envrc       # Shared via symlink (same content across all worktrees)
ws store track -s symlink .mcp.json
ws store track -s copy .env            # Copied (each worktree can customize independently)
ws store track -s copy .env.local
ws store track -s copy .env.development
```

| | symlink | copy |
|---|---------|------|
| Distribution method | Symbolic link | File copy |
| Content sharing | Identical across all worktrees | Independent per worktree |
| Update propagation | Instant (same link target) | Requires `push` / `pull` |
| Use case | `.envrc`, `.mcp.json`, etc. | `.env`, `.env.local`, `.env.development`, etc. |

> [!TIP]
> See [Shared Store](https://langify-org.github.io/ws-cli/concepts/shared-store.html) for the full workflow including `push`, `pull`, `status`, and `untrack`.

## Quick Start

```bash
# Bare clone the repository (default branch worktree is created automatically)
ws clone https://github.com/example/repo.git

# Create a feature branch
ws new feature/awesome
```

> [!TIP]
> See [Quick Start](https://langify-org.github.io/ws-cli/getting-started/quick-start.html) for more details.

## Documentation

See the **[ws documentation](https://langify-org.github.io/ws-cli/)** for the full reference.

## Installation

### Shell installer (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/langify-org/ws-cli/releases/latest/download/ws-installer.sh | sh
```

### Homebrew

```bash
brew install langify-org/tap/ws
```

### Nix flake

```bash
nix run github:langify-org/ws-cli
```

### Home Manager (Nix)

```nix
# flake.nix の inputs に追加
inputs.ws-cli.url = "github:langify-org/ws-cli";

# home.nix
home.packages = [
  inputs.ws-cli.packages.${system}.default
];
```

### cargo

```bash
cargo install --git https://github.com/langify-org/ws-cli.git
```

## License

MIT
