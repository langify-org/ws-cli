# ws デモ（日本語版）— git worktree ワークフローマネージャー
#
# Usage: nix run .#demo -- demo.ja.tape
# This tape assumes `ws` and `git` are in PATH.

Output demo.ja.gif

Set Shell "bash"
Set Width 960
Set Height 540
Set FontSize 16
Set Padding 16
Set TypingSpeed 40ms
Set Theme "Catppuccin Mocha"

# --- Setup (hidden) ---

Hide

Type `export LC_ALL=ja`
Enter
Type `export PS1='$ '`
Enter

# Isolated HOME so paths display as ~/my-app.
# pwd -P resolves /tmp -> /private/tmp on macOS to match ws abbreviate_home().
Type `export HOME="$(cd "$(mktemp -d)" && pwd -P)" && export XDG_CONFIG_HOME="$HOME/.config" && mkdir -p "$XDG_CONFIG_HOME/ws"`
Enter

# Create a demo bare repo
Type `cd "$HOME" && mkdir my-app && cd my-app && git init --bare .bare 2>/dev/null && echo "gitdir: .bare" > .git`
Enter
Sleep 1s
Type `git worktree add main --orphan 2>/dev/null`
Enter
Sleep 1s
# Create gitignored files: .env and .claude/settings.local.json
Type `cd main && echo '# My App' > README.md && echo 'SECRET=xxx' > .env && mkdir -p .claude && echo '{}' > .claude/settings.local.json && printf '.env\n.claude/settings.local.json\n' > .gitignore && git add -A && git commit -m "init" -q`
Enter
Sleep 1s
Type `cd .. && ws repos add . --name my-app 2>/dev/null && cd main`
Enter
Sleep 1s
Type "clear"
Enter
Sleep 500ms

Show

# --- Demo starts ---

Sleep 300ms

# 1. ダッシュボード表示
Type "ws status"
Sleep 500ms
Enter
Sleep 3s

# 2. gitignored ファイルを共有ストアに登録
Type "ws store track -s copy .env"
Sleep 500ms
Enter
Sleep 2s

Type "ws store track -s symlink .claude/settings.local.json"
Sleep 500ms
Enter
Sleep 2s

# 3. 新しい worktree を作成（store から自動配布）
Type "ws new feature/auth"
Sleep 500ms
Enter
Sleep 3s

# 4. store のステータス確認
Type "ws store status"
Sleep 500ms
Enter
Sleep 3s

# 5. 更新されたダッシュボード
Type "ws status"
Sleep 500ms
Enter
Sleep 4s
