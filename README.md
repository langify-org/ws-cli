# ws - workspace (git worktree) manager

A CLI tool that streamlines development with the git bare clone + worktree pattern.

## Features

- **One-step bare clone + worktree setup** — Get started with `ws clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees
- **Interactive mode** — Build and run commands interactively

## Quick Start

```bash
ws clone https://github.com/example/repo.git
ws new feature/awesome
```

## Documentation

See the **[ws documentation](https://langify-org.github.io/ws-cli/)** for details.

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
