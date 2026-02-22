use anyhow::Result;
use rust_i18n::t;

use crate::git::{git_output, worktree_root};
use crate::store::{file_status, read_manifest, store_dir};

pub(crate) fn cmd_status() -> Result<()> {
    let worktree_list = git_output(&["worktree", "list"])?;
    println!("{}", t!("status.workspaces"));

    let main_wt_root = worktree_root().ok();
    let store_available = store_dir()
        .ok()
        .filter(|s| s.is_dir() && s.join("manifest").is_file());

    for line in worktree_list.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let wt_path = parts[0];
        let branch_info = parts[2..].join(" ");

        let is_main = main_wt_root
            .as_ref()
            .map(|r| r.to_str() == Some(wt_path))
            .unwrap_or(false);

        let marker = if is_main { "*" } else { " " };

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

        println!(
            "  {} {:<40} {}{}",
            marker, wt_path, branch_info, tracked_info
        );
    }

    // Shared files セクション
    if let Some(store) = store_available {
        let entries = read_manifest(&store)?;
        if !entries.is_empty() {
            println!();
            println!("{}", t!("status.shared_files"));
            println!(
                "  {:<8} {:<40} {}",
                "STRATEGY", "FILE", "STATUS"
            );
            println!(
                "  {:<8} {:<40} {}",
                "--------", "----------------------------------------", "----------"
            );

            let wt_root = worktree_root().ok();

            for entry in &entries {
                let store_file = store.join(&entry.filepath);
                let status = file_status(entry, &store_file, &wt_root);
                println!(
                    "  {:<8} {:<40} {}",
                    entry.strategy, entry.filepath, status
                );
            }
        }
    }

    Ok(())
}
