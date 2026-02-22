# ws list

worktree の一覧を表示します。

## 使い方

```bash
ws list
```

## 動作

内部で `git worktree list` を実行し、結果をそのまま出力します。

## 出力例

```
/Users/user/my-project/.bare       (bare)
/Users/user/my-project/main        abc1234 [main]
/Users/user/my-project/feature-foo def5678 [feature/foo]
```

各行にはworktree のパス、HEAD のコミットハッシュ、ブランチ名が表示されます。
