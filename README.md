# ws - workspace (git worktree) manager

A CLI tool that streamlines development with the git bare clone + worktree pattern.

## Features

- **One-step bare clone + worktree setup** — Get started with `ws clone` then `ws new`
- **Shared store** — Automatically share gitignored files across worktrees
- **VSCode integration** — Automatically opens VSCode after creating a worktree
- **Interactive mode** — Build and run commands interactively

## Quick Start

```bash
ws clone https://github.com/example/repo.git
ws new main
ws new feature/awesome --from main
```

## Documentation

See the **[ws documentation](https://langify-org.github.io/ws-cli/)** for details.

## Installation

```bash
# Nix flake
nix run github:langify-org/ws-cli

# cargo
cargo install --git https://github.com/langify-org/ws-cli.git
```

## License

MIT
