use anyhow::{bail, Context, Result};
use rust_i18n::t;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cli::{CloneCmd, NewCmd, RmCmd};
use crate::git::{find_bare_dir, is_inside_git_worktree};
use crate::store;

pub(crate) fn generate_name() -> String {
    petname::petname(3, "-").expect(&t!("store.name_generation_failed"))
}

pub(crate) fn cmd_clone(cmd: &CloneCmd) -> Result<()> {
    let bare_dir = PathBuf::from(".bare");
    if bare_dir.exists() {
        bail!("{}", t!("worktree.bare_already_exists"));
    }

    let status = if let Some(ref url) = cmd.url {
        Command::new("git")
            .args(["clone", "--bare", url, ".bare"])
            .status()
            .context(t!("worktree.clone_bare_failed").to_string())?
    } else {
        Command::new("git")
            .args(["init", "--bare", ".bare"])
            .status()
            .context(t!("worktree.init_bare_failed").to_string())?
    };

    if !status.success() {
        bail!("{}", t!("worktree.bare_creation_failed"));
    }

    println!("{}", t!("worktree.bare_created"));
    Ok(())
}

pub(crate) fn cmd_new(cmd: &NewCmd) -> Result<()> {
    let name = match &cmd.name {
        Some(n) => n.clone(),
        None => generate_name(),
    };

    let branch = cmd.branch.clone().unwrap_or_else(|| name.clone());

    let is_bare_root = !is_inside_git_worktree() && find_bare_dir().is_some();
    let directory = cmd.directory.clone().unwrap_or_else(|| {
        if is_bare_root {
            name.clone()
        } else {
            format!("../{}", name)
        }
    });

    let mut check_cmd = Command::new("git");
    if is_bare_root {
        check_cmd.arg("--git-dir").arg(".bare");
    }
    let branch_exists = check_cmd
        .args(["show-ref", "--verify", "--quiet", &format!("refs/heads/{}", branch)])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let start_point = cmd.from.as_deref().unwrap_or("HEAD");

    // 起点の参照が有効かチェック（空の bare リポジトリでは HEAD が無効）
    let mut rev_parse_cmd = Command::new("git");
    if is_bare_root {
        rev_parse_cmd.arg("--git-dir").arg(".bare");
    }
    let start_point_valid = rev_parse_cmd
        .args(["rev-parse", "--verify", "--quiet", start_point])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let args = if branch_exists {
        // 既存ブランチをチェックアウト
        vec!["worktree", "add", &directory, &branch]
    } else if start_point_valid {
        // 新規ブランチを作成
        vec!["worktree", "add", "-b", &branch, &directory, start_point]
    } else if cmd.from.is_none() {
        // --from 省略 & HEAD が無効（空リポジトリ等）→ orphan ブランチで作成
        vec!["worktree", "add", "--orphan", "-b", &branch, &directory]
    } else {
        bail!("{}", t!("worktree.start_point_not_found", point = start_point));
    };

    let mut git_cmd = Command::new("git");
    if is_bare_root {
        git_cmd.arg("--git-dir").arg(".bare");
    }
    let status = git_cmd
        .args(&args)
        .status()
        .context(t!("worktree.worktree_add_failed").to_string())?;

    if !status.success() {
        bail!("{}", t!("worktree.worktree_add_git_failed"));
    }

    // store が存在すればファイルを適用
    if let Ok(sd) = store::store_dir() {
        if sd.is_dir() && sd.join("manifest").is_file() {
            let abs_directory = fs::canonicalize(&directory)
                .with_context(|| t!("worktree.dir_canonicalize_failed", dir = &directory).to_string())?;
            println!("{}", t!("worktree.applying_store_files"));
            let entries = store::read_manifest(&sd)?;
            for entry in &entries {
                store::apply_file(&entry.strategy, &entry.filepath, &sd, &abs_directory)?;
            }
        }
    }

    Command::new("code")
        .arg(&directory)
        .status()
        .context(t!("worktree.vscode_launch_failed").to_string())?;

    Ok(())
}

pub(crate) fn cmd_list() -> Result<()> {
    let output = crate::git::git_output(&["worktree", "list"])?;
    println!("{}", output);
    Ok(())
}

pub(crate) fn cmd_rm(cmd: &RmCmd) -> Result<()> {
    let mut args = vec!["worktree", "remove"];
    if cmd.force {
        args.push("--force");
    }
    args.push(&cmd.directory);

    let status = Command::new("git")
        .args(&args)
        .status()
        .context(t!("worktree.worktree_remove_failed").to_string())?;

    if !status.success() {
        bail!("{}", t!("worktree.worktree_remove_git_failed"));
    }

    Ok(())
}
