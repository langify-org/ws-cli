use anyhow::{Context, Result, bail};
use rust_i18n::t;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// カレントディレクトリ直下の `.bare` を検出する
pub fn find_bare_dir() -> Option<PathBuf> {
    let bare = PathBuf::from(".bare");
    if bare.is_dir() && bare.join("HEAD").is_file() {
        Some(bare)
    } else {
        None
    }
}

/// Git worktree 内にいるかどうかを判定する
pub fn is_inside_git_worktree() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn git_output(args: &[&str]) -> Result<String> {
    let mut cmd = Command::new("git");

    if !is_inside_git_worktree()
        && let Some(bare_dir) = find_bare_dir()
    {
        cmd.arg("--git-dir").arg(&bare_dir);
    }

    cmd.args(args);

    let output = cmd
        .output()
        .with_context(|| t!("git.exec_failed", args = args.join(" ")).to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "{}",
            t!(
                "git.command_failed",
                args = args.join(" "),
                stderr = stderr.trim()
            )
        );
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn worktree_root() -> Result<PathBuf> {
    let root = git_output(&["rev-parse", "--show-toplevel"])
        .context(t!("git.run_inside_worktree").to_string())?;
    Ok(PathBuf::from(root))
}

/// 指定パス（または cwd）が属するリポジトリのルートパス（canonical）を解決する。
///
/// 解決ロジック:
/// 1. `git rev-parse --git-common-dir` → `.bare` の親（bare worktree パターン）
/// 2. 同上 → `.git` の親（通常の clone）
/// 3. `git rev-parse --show-toplevel`（フォールバック）
/// 4. `.bare` ディレクトリの直接検出（bare root にいる場合）
pub fn resolve_repo_root(path: Option<&Path>) -> Option<PathBuf> {
    let working_dir = match path {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().ok()?,
    };

    // git-common-dir を取得
    let common_dir = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .current_dir(&working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if common_dir.status.success() {
        let common = String::from_utf8_lossy(&common_dir.stdout)
            .trim()
            .to_string();
        let common_path = if Path::new(&common).is_absolute() {
            PathBuf::from(&common)
        } else {
            working_dir.join(&common)
        };
        if let Ok(canonical) = common_path.canonicalize() {
            let name = canonical.file_name().and_then(|n| n.to_str());
            // bare worktree パターン: .bare の親がリポジトリルート
            if name == Some(".bare") {
                return canonical.parent().map(|p| p.to_path_buf());
            }
            // 通常の clone: .git の親がリポジトリルート
            if name == Some(".git") {
                return canonical.parent().map(|p| p.to_path_buf());
            }
        }
    }

    // フォールバック: show-toplevel
    let toplevel = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(&working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if toplevel.status.success() {
        let root = String::from_utf8_lossy(&toplevel.stdout).trim().to_string();
        return PathBuf::from(root).canonicalize().ok();
    }

    // 最終フォールバック: bare root にいる場合
    if working_dir.join(".bare").is_dir() {
        return std::fs::canonicalize(&working_dir).ok();
    }

    None
}
