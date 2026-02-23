use anyhow::{Result, bail};
use rust_i18n::t;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cli::{ReposAddCmd, ReposRmCmd};
use crate::config::{RepoEntry, load_config, save_config};
use crate::ui::{self, StyledCell};

pub fn cmd_repos_add(cmd: &ReposAddCmd) -> Result<()> {
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

    // リポジトリルートを解決
    let path = crate::git::resolve_repo_root(Some(&path)).unwrap_or(path);

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

    anstream::println!(
        "{}",
        ui::styled(
            ui::STYLE_OK,
            &t!(
                "repos.added",
                name = &name,
                path = path.display().to_string()
            )
        )
    );
    Ok(())
}

pub fn cmd_repos_list(ctx: &crate::context::AppContext) -> Result<()> {
    if ctx.config.repos.is_empty() {
        anstream::println!("{}", t!("repos.no_repos"));
        return Ok(());
    }

    let mut rows = Vec::new();
    for (name, entry) in &ctx.config.repos {
        let display_path = crate::context::abbreviate_home(&entry.path);
        let url = entry.url.as_deref().unwrap_or("").to_string();
        rows.push(vec![
            StyledCell::plain(name.clone()),
            StyledCell::plain(display_path),
            StyledCell::plain(url),
        ]);
    }

    crate::context::print_table(&["NAME", "PATH", "URL"], &rows, 0, None);

    Ok(())
}

pub struct WorktreeEntry {
    pub rel_path: String,
    pub branch: String,
    pub hash: String,
    pub is_bare: bool,
}

pub fn parse_worktree_list(output: &str, repo_root: &std::path::Path) -> Vec<WorktreeEntry> {
    let repo_root_canonical = repo_root
        .canonicalize()
        .unwrap_or_else(|_| repo_root.to_path_buf());

    output
        .lines()
        .filter_map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() < 2 {
                return None;
            }

            let abs_path = PathBuf::from(tokens[0]);

            // (bare) エントリ
            if tokens.get(1) == Some(&"(bare)") {
                return Some(WorktreeEntry {
                    rel_path: String::new(),
                    branch: String::new(),
                    hash: String::new(),
                    is_bare: true,
                });
            }

            // 通常エントリ: <path> <hash> [<branch>]
            if tokens.len() < 3 {
                return None;
            }
            let hash = tokens[1].to_string();
            // branch は [branch] 形式 → [] を除去
            let branch_raw = tokens[2..].join(" ");
            let branch = branch_raw
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_string();

            // 相対パスに変換
            let rel_path = abs_path
                .canonicalize()
                .ok()
                .and_then(|canonical| {
                    canonical.strip_prefix(&repo_root_canonical).ok().map(|p| {
                        let s = p.to_string_lossy().to_string();
                        if s.is_empty() { ".".to_string() } else { s }
                    })
                })
                .unwrap_or_else(|| {
                    // prefix 外の場合: リポジトリルートからの相対パスを試みる
                    pathdiff::diff_paths(&abs_path, &repo_root_canonical)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| abs_path.to_string_lossy().to_string())
                });

            Some(WorktreeEntry {
                rel_path,
                branch,
                hash,
                is_bare: false,
            })
        })
        .collect()
}

pub fn cmd_repos_rm(cmd: &ReposRmCmd) -> Result<()> {
    let mut config = load_config()?;

    if config.repos.remove(&cmd.name).is_none() {
        bail!("{}", t!("repos.not_found", name = &cmd.name));
    }

    save_config(&config)?;
    anstream::println!(
        "{}",
        ui::styled(ui::STYLE_OK, &t!("repos.removed", name = &cmd.name))
    );
    Ok(())
}
