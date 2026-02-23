use anyhow::{bail, Result};
use rust_i18n::t;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cli::{ReposAddCmd, ReposRmCmd};
use crate::config::{load_config, save_config, RepoEntry};

pub(crate) fn cmd_repos_add(cmd: &ReposAddCmd) -> Result<()> {
    let raw_path = match &cmd.path {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir()?,
    };

    let path = raw_path.canonicalize().map_err(|_| {
        anyhow::anyhow!(
            "{}",
            t!(
                "repos.path_not_found",
                path = raw_path.display().to_string()
            )
        )
    })?;

    // git リポジトリか検証
    let git_check = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(&path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match git_check {
        Ok(s) if s.success() => {}
        _ => bail!(
            "{}",
            t!("repos.not_a_git_repo", path = path.display().to_string())
        ),
    }

    // 名前を決定
    let name = match &cmd.name {
        Some(n) => n.clone(),
        None => path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string()),
    };

    let mut config = load_config()?;

    // 重複チェック
    if config.repos.contains_key(&name) {
        bail!("{}", t!("repos.already_registered", name = &name));
    }

    // URL を自動検出
    let url = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(&path)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty());

    config.repos.insert(
        name.clone(),
        RepoEntry {
            path: path.clone(),
            url,
        },
    );
    save_config(&config)?;

    println!(
        "{}",
        t!(
            "repos.added",
            name = &name,
            path = path.display().to_string()
        )
    );
    Ok(())
}

pub(crate) fn cmd_repos_list() -> Result<()> {
    let config = load_config()?;

    if config.repos.is_empty() {
        println!("{}", t!("repos.no_repos"));
        return Ok(());
    }

    for (name, entry) in &config.repos {
        match &entry.url {
            Some(url) => println!("{:<20} {} ({})", name, entry.path.display(), url),
            None => println!("{:<20} {}", name, entry.path.display()),
        }
    }

    Ok(())
}

pub(crate) fn cmd_repos_rm(cmd: &ReposRmCmd) -> Result<()> {
    let mut config = load_config()?;

    if config.repos.remove(&cmd.name).is_none() {
        bail!("{}", t!("repos.not_found", name = &cmd.name));
    }

    save_config(&config)?;
    println!("{}", t!("repos.removed", name = &cmd.name));
    Ok(())
}
