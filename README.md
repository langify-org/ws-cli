# ws - workspace & repository manager

> **[日本語版はこちら](README.ja.md)**

A CLI tool that manages your system's repositories, worktrees, and shared configuration in one place.

![demo](demo.gif)

## Why ws?

Working across multiple repositories and branches involves persistent friction:

- **Tedious bare clone initialization** — After `git clone --bare`, you need to manually add worktrees
- **Managing gitignored files** — Files like `.env`, `.env.local`, and `.claude/settings.local.json` are outside git, so you must manually copy or link them every time you create a new worktree
- **Scattered repository management** — Repositories live in different directories with no unified way to track or inspect them

ws solves these problems by providing a single CLI for repository registration, worktree management, and gitignored file sharing.

## Features

- **Repository registry** — Register and manage all your repositories with `ws repos` for system-wide visibility
- **Bare clone + worktree management** — One-step setup with `ws repos clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees (symlink / copy strategies)
- **Interactive mode** — Build and run commands interactively

## Shared Store

The shared store centrally manages gitignored files across worktrees. Register files once, and they are automatically distributed when new worktrees are created with `ws new`.

```bash
# Register files to the store
ws store track -s symlink .claude/settings.local.json  # Shared via symlink (same content across all worktrees)
ws store track -s copy .env            # Copied (each worktree can customize independently)
ws store track -s copy .env.local
ws store track -s copy .env.development
```

| | symlink | copy |
|---|---------|------|
| Distribution method | Symbolic link | File copy |
| Content sharing | Identical across all worktrees | Independent per worktree |
| Update propagation | Instant (same link target) | Requires `push` / `pull` |
| Use case | `.claude/settings.local.json`, etc. | `.env`, `.env.local`, `.env.development`, etc. |

> [!TIP]
> See [Shared Store](https://langify-org.github.io/ws-cli/concepts/shared-store.html) for the full workflow including `push`, `pull`, `status`, and `untrack`.

## Quick Start

### Already have a cloned repository?

```bash
cd my-project
ws new feature/awesome       # Create a worktree for a new branch
```

### Cloning a new repository?

```bash
ws repos clone https://github.com/example/repo.git
ws new feature/awesome
```

### Starting a new project from scratch?

```bash
mkdir my-project && cd my-project
ws repos clone               # Create an empty bare repository
ws new main                  # Create a worktree with an orphan branch
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

## Development

### Regenerating demo GIFs

The demo GIFs are generated from [VHS](https://github.com/charmbracelet/vhs) tape files. No local build or tool installation is required — everything runs through Nix:

```bash
# English (demo.gif)
nix run .#demo

# Japanese (demo.ja.gif)
nix run .#demo -- demo.ja.tape
```

The tape files (`demo.tape`, `demo.ja.tape`) are self-contained scripts that create an isolated environment, so they don't affect your local configuration.

## License

MIT
