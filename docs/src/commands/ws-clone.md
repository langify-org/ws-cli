# ws clone

Create a bare repository.

## Usage

```bash
ws clone [url]
```

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `url` | No | Remote URL. If omitted, creates an empty bare repository |

## Behavior

Creates a `.bare/` directory in the current directory.

- With a URL: runs `git clone --bare <url> .bare`
- Without a URL: runs `git init --bare .bare`

Fails with an error if `.bare` already exists.

## Examples

### Bare clone a remote repository

```bash
mkdir my-project && cd my-project
ws clone https://github.com/example/repo.git
```

### Create an empty bare repository

```bash
mkdir my-project && cd my-project
ws clone
ws new master    # Create a worktree with an orphan branch
```
