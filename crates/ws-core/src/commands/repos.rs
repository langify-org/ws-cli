use anyhow::{Result, bail};
use rust_i18n::t;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cli::{ReposAddCmd, ReposRmCmd};
use crate::config::{RepoEntry, load_config, save_config};

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
    let path = resolve_repo_root(&path).unwrap_or(path);

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

pub fn cmd_repos_list() -> Result<()> {
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

/// 指定パスからリポジトリのルートディレクトリを解決する。
/// - bare worktree パターン: git-common-dir (.bare) の親ディレクトリ
/// - 通常の clone: git rev-parse --show-toplevel
fn resolve_repo_root(path: &std::path::Path) -> Option<PathBuf> {
    // bare worktree パターン: .bare の親がリポジトリルート
    let common_dir = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .current_dir(path)
        .output()
        .ok()?;
    if common_dir.status.success() {
        let common = String::from_utf8_lossy(&common_dir.stdout)
            .trim()
            .to_string();
        let common_path = if std::path::Path::new(&common).is_absolute() {
            PathBuf::from(&common)
        } else {
            path.join(&common)
        };
        if let Ok(canonical) = common_path.canonicalize()
            && canonical.file_name().and_then(|n| n.to_str()) == Some(".bare")
            && let Some(parent) = canonical.parent()
        {
            return Some(parent.to_path_buf());
        }
    }

    // 通常の clone: show-toplevel がリポジトリルート
    let toplevel = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .ok()?;
    if toplevel.status.success() {
        let root = String::from_utf8_lossy(&toplevel.stdout).trim().to_string();
        return PathBuf::from(root).canonicalize().ok();
    }

    None
}

pub fn cmd_repos_status() -> Result<()> {
    let config = load_config()?;

    if config.repos.is_empty() {
        println!("{}", t!("repos.no_repos"));
        return Ok(());
    }

    let mut first = true;
    for (name, entry) in &config.repos {
        if !first {
            println!();
        }
        first = false;

        println!("{} ({})", name, entry.path.display());

        if !entry.path.exists() {
            println!("  NOT_FOUND");
            continue;
        }

        let is_bare = entry.path.join(".bare").is_dir();

        // GIT_DIR を表示
        if is_bare {
            println!("  GIT_DIR: .bare");
        } else {
            println!("  GIT_DIR: .git");
        }

        // worktree 一覧を取得
        let wt_output = if is_bare {
            Command::new("git")
                .args(["--git-dir", ".bare", "worktree", "list"])
                .current_dir(&entry.path)
                .output()
        } else {
            Command::new("git")
                .args(["worktree", "list"])
                .current_dir(&entry.path)
                .output()
        };

        let entries = match wt_output {
            Ok(output) if output.status.success() => {
                parse_worktree_list(&String::from_utf8_lossy(&output.stdout), &entry.path)
            }
            _ => vec![],
        };

        if is_bare {
            // bare パターン: (bare) を除外し Worktrees: で一括表示
            let worktrees: Vec<&WorktreeEntry> = entries.iter().filter(|e| !e.is_bare).collect();
            if !worktrees.is_empty() {
                println!("  Worktrees:");
                for (i, wt) in worktrees.iter().enumerate() {
                    let connector = if i == worktrees.len() - 1 {
                        "└──"
                    } else {
                        "├──"
                    };
                    println!(
                        "    {} {}   [{}] {}",
                        connector, wt.rel_path, wt.branch, wt.hash
                    );
                }
            }
        } else {
            // 通常 clone: 最初が Main worktree、残りが Linked worktrees
            if let Some(main_wt) = entries.first() {
                println!("  Main worktree:");
                println!(
                    "    {}   [{}] {}",
                    main_wt.rel_path, main_wt.branch, main_wt.hash
                );

                let linked: Vec<&WorktreeEntry> = entries[1..].iter().collect();
                if !linked.is_empty() {
                    println!("  Linked worktrees:");
                    for (i, wt) in linked.iter().enumerate() {
                        let connector = if i == linked.len() - 1 {
                            "└──"
                        } else {
                            "├──"
                        };
                        println!(
                            "    {} {}   [{}] {}",
                            connector, wt.rel_path, wt.branch, wt.hash
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

struct WorktreeEntry {
    rel_path: String,
    branch: String,
    hash: String,
    is_bare: bool,
}

fn parse_worktree_list(output: &str, repo_root: &std::path::Path) -> Vec<WorktreeEntry> {
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
    println!("{}", t!("repos.removed", name = &cmd.name));
    Ok(())
}
