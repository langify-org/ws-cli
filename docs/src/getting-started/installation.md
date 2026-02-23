# Installation

## Nix flake (Recommended)

ws is available as a Nix flake.

### Run directly

```bash
nix run github:langify-org/ws-cli
```

### Install to profile

```bash
nix profile install github:langify-org/ws-cli
```

### Add as a flake input

```nix
{
  inputs = {
    ws-cli.url = "github:langify-org/ws-cli";
  };

  # Reference ws-cli.packages.${system}.default in outputs
}
```

### Home Manager

Add `ws-cli` as a flake input and import the Home Manager module:

```nix
# flake.nix
{
  inputs = {
    ws-cli.url = "github:langify-org/ws-cli";
  };
}
```

```nix
# home.nix
{ inputs, system, ... }:
{
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
}
```

#### `programs.ws` options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enable` | bool | `false` | Enable ws |
| `package` | package | `pkgs.ws` | The ws package to install |
| `repos` | attrset | `{}` | Repositories to register in `~/.config/ws/config.toml` |

Each entry in `repos`:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | string | Yes | Path to the repository |
| `url` | string | No | Remote URL |

## cargo install

If you have the Rust toolchain installed, you can build directly with cargo.

```bash
cargo install --git https://github.com/langify-org/ws-cli.git
```

## Build from source

```bash
git clone https://github.com/langify-org/ws-cli.git
cd ws-cli
cargo build --release
# Copy ./target/release/ws to a directory in your PATH
```

## Dependencies

ws uses the following external commands:

| Command | Required | Purpose |
|---------|----------|---------|
| `git` | Yes | All worktree operations |
