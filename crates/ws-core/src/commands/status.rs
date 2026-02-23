use anyhow::Result;
use rust_i18n::t;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::commands::repos::{WorktreeEntry, parse_worktree_list};
use crate::config::load_config;
use crate::git::worktree_root;
use crate::store::{file_status, read_manifest, store_dir};

pub fn cmd_status() -> Result<()> {
    let config = load_config()?;
    let current_repo_root = resolve_current_repo_root();
    let mut has_output = false;

    // --- Repositories section ---
    if !config.repos.is_empty() {
        println!("{}", t!("status.repositories"));
        has_output = true;

        let mut first = true;
        for (name, entry) in &config.repos {
            if !first {
                println!();
            }
            first = false;

            let is_current = current_repo_root
                .as_ref()
                .and_then(|r| entry.path.canonicalize().ok().map(|p| p == *r))
                .unwrap_or(false);

            let marker = if is_current { "*" } else { " " };
            println!("{} {}", marker, name);
            println!("    Path: {}", entry.path.display());

            if !entry.path.exists() {
                println!("    NOT_FOUND");
                continue;
            }

            let is_bare = entry.path.join(".bare").is_dir();
            if is_bare {
                println!("    GIT_DIR: .bare");
            } else {
                println!("    GIT_DIR: .git");
            }

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
                let worktrees: Vec<&WorktreeEntry> =
                    entries.iter().filter(|e| !e.is_bare).collect();
                if !worktrees.is_empty() {
                    println!("    Worktrees:");
                    for (i, wt) in worktrees.iter().enumerate() {
                        let connector = if i == worktrees.len() - 1 {
                            "└──"
                        } else {
                            "├──"
                        };
                        println!(
                            "      {} {}   [{}] {}",
                            connector, wt.rel_path, wt.branch, wt.hash
                        );
                    }
                }
            } else {
                if let Some(main_wt) = entries.first() {
                    println!("    Main worktree:");
                    println!(
                        "      {}   [{}] {}",
                        main_wt.rel_path, main_wt.branch, main_wt.hash
                    );

                    let linked: Vec<&WorktreeEntry> = entries[1..].iter().collect();
                    if !linked.is_empty() {
                        println!("    Linked worktrees:");
                        for (i, wt) in linked.iter().enumerate() {
                            let connector = if i == linked.len() - 1 {
                                "└──"
                            } else {
                                "├──"
                            };
                            println!(
                                "      {} {}   [{}] {}",
                                connector, wt.rel_path, wt.branch, wt.hash
                            );
                        }
                    }
                }
            }
        }
    }

    // --- Current workspace section (only in a worktree) ---
    let wt_root = worktree_root().ok();
    let store_available = store_dir()
        .ok()
        .filter(|s| s.is_dir() && s.join("manifest").is_file());

    if let Some(ref root) = wt_root {
        if has_output {
            println!();
        }
        has_output = true;

        println!("{}", t!("status.current_workspace"));

        let branch_info = crate::git::git_output(&["rev-parse", "--abbrev-ref", "HEAD"])
            .unwrap_or_else(|_| "???".to_string());

        let tracked_info = if let Some(ref store) = store_available {
            let entries = read_manifest(store).unwrap_or_default();
            if !entries.is_empty() {
                t!("status.files_tracked", count = entries.len()).to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        println!("  * {} [{}]{}", root.display(), branch_info, tracked_info);
    }

    // --- Shared files section ---
    if let Some(store) = store_available {
        let entries = read_manifest(&store)?;
        if !entries.is_empty() {
            if has_output {
                println!();
            }
            has_output = true;

            println!("{}", t!("status.shared_files"));
            println!("  {:<8} {:<40} STATUS", "STRATEGY", "FILE");
            println!(
                "  {:<8} {:<40} ----------",
                "--------", "----------------------------------------"
            );

            for entry in &entries {
                let store_file = store.join(&entry.filepath);
                let status = file_status(entry, &store_file, &wt_root);
                println!("  {:<8} {:<40} {}", entry.strategy, entry.filepath, status);
            }
        }
    }

    if !has_output {
        println!("{}", t!("repos.no_repos"));
    }

    Ok(())
}

/// カレントディレクトリが属するリポジトリのルートパス（canonical）を解決する。
fn resolve_current_repo_root() -> Option<PathBuf> {
    let common_dir = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if common_dir.status.success() {
        let common = String::from_utf8_lossy(&common_dir.stdout)
            .trim()
            .to_string();
        let common_path = if std::path::Path::new(&common).is_absolute() {
            PathBuf::from(&common)
        } else {
            std::env::current_dir().ok()?.join(&common)
        };
        if let Ok(canonical) = common_path.canonicalize() {
            // bare worktree パターン: .bare がリポジトリルート
            if canonical.file_name().and_then(|n| n.to_str()) == Some(".bare") {
                return canonical.parent().map(|p| p.to_path_buf());
            }
            // 通常の clone: .git の親がリポジトリルート
            if canonical.file_name().and_then(|n| n.to_str()) == Some(".git") {
                return canonical.parent().map(|p| p.to_path_buf());
            }
        }
    }

    // フォールバック: bare root にいる場合
    if PathBuf::from(".bare").is_dir() {
        return std::fs::canonicalize(".").ok();
    }

    None
}
