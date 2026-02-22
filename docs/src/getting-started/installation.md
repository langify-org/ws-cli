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
