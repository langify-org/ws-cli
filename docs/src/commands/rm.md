# ws rm

指定した worktree を削除します。

## 使い方

```bash
ws rm <path> [-f]
```

## 引数

| 引数 | 必須 | 説明 |
|------|------|------|
| `path` | はい | 削除する worktree のパス |

## オプション

| オプション | 短縮 | 説明 |
|-----------|------|------|
| `--force` | `-f` | 未コミットの変更があっても強制削除する |

## 動作

内部で `git worktree remove <path>` を実行します。`-f` を指定した場合は `--force` フラグが追加されます。

未コミットの変更がある worktree を `-f` なしで削除しようとするとエラーになります。

## 例

```bash
# worktree を削除
ws rm feature-foo

# 強制削除
ws rm feature-foo -f
```
