# ws - AI-ready workspace & repository manager

> **[日本語版はこちら](README.ja.md)**

A CLI tool that manages your system's repositories, worktrees, and shared configuration in one place.

## Why ws?

Working across multiple repositories and branches involves persistent friction:

- **Tedious bare clone initialization** — After `git clone --bare`, you need to manually add worktrees
- **Managing gitignored files** — Files like `.envrc`, `.mcp.json`, `.env`, and `.env.local` are outside git, so you must manually copy or link them every time you create a new worktree
- **Scattered repository management** — Repositories live in different directories with no unified way to track or inspect them

ws solves these problems by providing a single CLI for repository registration, worktree management, and gitignored file sharing.

## Features

- **Repository registry** — Register and manage all your repositories with `ws repos` for system-wide visibility
- **Bare clone + worktree management** — One-step setup with `ws repos clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees (symlink / copy strategies)
- **AI agent integration** — Share agent config across worktrees; give agents system-wide repository awareness
- **Interactive mode** — Build and run commands interactively

## AI Agent Integration

AI coding agents like Claude Code need several configuration files — `.mcp.json`, `.claude/settings.local.json`, `.env`, and more. These are all gitignored, so every new worktree starts without them. Manually copying them each time is tedious and error-prone.

### Shared config across worktrees

ws lets you register these files once in the shared store and automatically distributes them to every new worktree:

```bash
# Shared config — symlink keeps all worktrees in sync
ws store track -s symlink .mcp.json
ws store track -s symlink .claude/settings.local.json

# Per-worktree secrets — copy allows independent customization
ws store track -s copy .env
ws store track -s copy .env.local

# New worktrees are ready for AI agents from the start
ws new feature/awesome
cd ../feature-awesome
# Claude Code works immediately — no setup needed
```

With symlink strategy, updating `.mcp.json` or `.claude/settings.local.json` in one worktree instantly applies to all others. With copy strategy, each worktree can have its own `.env` values while still starting from a working baseline.

### System-wide repository awareness

AI agents often need to work across repository boundaries — referencing another project's code, coordinating changes across repos, or navigating your system's project structure. `ws repos` gives agents a registry of all your repositories:

```bash
# Register your repositories
ws repos add ~/projects/frontend
ws repos add ~/projects/backend
ws repos add ~/projects/shared-lib

# Agents can discover your project landscape
ws repos list
ws status
```

With a centralized registry, an AI agent can discover where related projects live, understand your system's structure, and navigate between repositories without you having to explain paths manually each time.

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
ws repos clone https://github.com/example/repo.git

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

A binary cache is available via [Cachix](https://app.cachix.org/cache/langify-org) to avoid building from source:

```bash
cachix use langify-org
```

```bash
nix run github:langify-org/ws-cli
```

### Home Manager (Nix)

```nix
# flake.nix の inputs に追加
inputs.ws-cli.url = "github:langify-org/ws-cli";

# home.nix
imports = [ inputs.ws-cli.homeManagerModules.default ];

programs.ws = {
  enable = true;
  package = inputs.ws-cli.packages.${system}.default;
  repos = {
    my-repo = {
      path = "/Users/user/projects/my-repo";
      url = "git@github.com:user/my-repo.git";
    };
  };
};
```

> [!TIP]
> To use the binary cache with Home Manager, nix-darwin, or NixOS, add the Cachix substituter to your Nix configuration instead of running `cachix use`:
>
> ```nix
> nix.settings = {
>   substituters = [ "https://langify-org.cachix.org" ];
>   trusted-public-keys = [ "langify-org.cachix.org-1:zO6Hf3s6e3Ex7PDSazL1A7XwR/3Deui7G3LUrs4+nq4=" ];
> };
> ```

### cargo

```bash
cargo install --git https://github.com/langify-org/ws-cli.git
```

## License

MIT
