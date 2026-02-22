# ws i (Interactive mode)

Build and run commands interactively.

## Usage

```bash
ws i
```

## Behavior

1. Displays a list of available commands
2. After selecting a command, prompts for required parameters interactively
3. Executes the assembled command as a `ws` subprocess

## Available commands

| Command | Description |
|---------|-------------|
| `clone` | Create a bare repository |
| `new` | Create a workspace |
| `rm` | Remove a workspace |
| `list` | List worktrees |
| `status` | Show overall status |
| `shared` | Manage shared files (has a submenu) |

## Example

```bash
$ ws i
# → Command selection menu appears
# → Select "new"
# → Enter name, location, branch, and starting point interactively
# → "ws new feature/foo --from main" is executed
```

The assembled command is printed to stderr before execution, so you can see exactly what will run.

```
> ws new feature/foo --from main
```
