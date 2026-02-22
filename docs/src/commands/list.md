# ws list

List all worktrees.

## Usage

```bash
ws list
```

## Behavior

Runs `git worktree list` internally and outputs the result.

## Example output

```
/Users/user/my-project/.bare       (bare)
/Users/user/my-project/main        abc1234 [main]
/Users/user/my-project/feature-foo def5678 [feature/foo]
```

Each line shows the worktree path, HEAD commit hash, and branch name.
