use anyhow::{Context, Result, bail};
use rust_i18n::t;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// カレントディレクトリ直下の `.bare` を検出する
pub(crate) fn find_bare_dir() -> Option<PathBuf> {
    let bare = PathBuf::from(".bare");
    if bare.is_dir() && bare.join("HEAD").is_file() {
        Some(bare)
    } else {
        None
    }
}

/// Git worktree 内にいるかどうかを判定する
pub(crate) fn is_inside_git_worktree() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub(crate) fn git_output(args: &[&str]) -> Result<String> {
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

pub(crate) fn worktree_root() -> Result<PathBuf> {
    let root = git_output(&["rev-parse", "--show-toplevel"])
        .context(t!("git.run_inside_worktree").to_string())?;
    Ok(PathBuf::from(root))
}
