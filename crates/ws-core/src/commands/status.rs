use anyhow::Result;
use rust_i18n::t;

use crate::commands::repos::WorktreeEntry;
use crate::context::{AppContext, abbreviate_home, print_table};
use crate::store::file_status;

pub fn cmd_status(ctx: &AppContext) -> Result<()> {
    let mut has_output = false;

    // --- Repositories section ---
    if !ctx.config.repos.is_empty() {
        println!("{}", t!("status.repositories"));
        has_output = true;

        let headers = ["NAME", "PATH", "TYPE"];
        let num_cols = headers.len();

        let mut rows: Vec<(bool, Vec<String>)> = Vec::new();
        for (name, entry) in &ctx.config.repos {
            let is_current = ctx
                .current_repo
                .as_ref()
                .and_then(|r| entry.path.canonicalize().ok().map(|p| p == r.root))
                .unwrap_or(false);

            let display_path = abbreviate_home(&entry.path);
            let repo_type = if !entry.path.exists() {
                "NOT_FOUND"
            } else if entry.path.join(".bare").is_dir() {
                "bare"
            } else {
                "git"
            };

            rows.push((
                is_current,
                vec![name.clone(), display_path, repo_type.to_string()],
            ));
        }

        // Calculate column widths
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        for (_, row) in &rows {
            for (i, cell) in row.iter().enumerate().take(num_cols) {
                widths[i] = widths[i].max(cell.len());
            }
        }

        // Header (with 2-space indent)
        let header_line: String = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                if i == num_cols - 1 {
                    h.to_string()
                } else {
                    format!("{:<width$}", h, width = widths[i])
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        println!("  {header_line}");

        // Separator
        let sep_line: String = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let sep = "\u{2500}".repeat(h.len());
                if i == num_cols - 1 {
                    sep
                } else {
                    let padding = widths[i].saturating_sub(h.len());
                    format!("{sep}{}", " ".repeat(padding))
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        println!("  {sep_line}");

        // Data rows (marker replaces 2-space indent)
        for (is_current, row) in &rows {
            let marker = if *is_current { "*" } else { " " };
            let row_line: String = (0..num_cols)
                .map(|i| {
                    let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                    if i == num_cols - 1 {
                        cell.to_string()
                    } else {
                        format!("{:<width$}", cell, width = widths[i])
                    }
                })
                .collect::<Vec<_>>()
                .join("  ");
            println!("{marker} {row_line}");
        }
    }

    // --- Current Repository section ---
    if let Some(ref repo) = ctx.current_repo {
        if has_output {
            println!();
        }
        has_output = true;

        let display_name = repo.name.clone().unwrap_or_else(|| {
            repo.root
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string()
        });

        println!("{}", t!("status.current_repository", name = &display_name));
        println!("  Path: {}", abbreviate_home(&repo.root));

        let worktrees: Vec<&WorktreeEntry> = repo.worktrees.iter().filter(|w| !w.is_bare).collect();

        if !worktrees.is_empty() {
            println!("  Worktrees:");

            let current_rel = ctx.current_workspace.as_ref().and_then(|ws| {
                ws.root.canonicalize().ok().and_then(|canonical_ws| {
                    repo.root.canonicalize().ok().and_then(|canonical_repo| {
                        canonical_ws.strip_prefix(&canonical_repo).ok().map(|p| {
                            let s = p.to_string_lossy().to_string();
                            if s.is_empty() { ".".to_string() } else { s }
                        })
                    })
                })
            });

            for (i, wt) in worktrees.iter().enumerate() {
                let is_last = i == worktrees.len() - 1;
                let connector = if is_last { "└──" } else { "├──" };
                let is_current = current_rel
                    .as_deref()
                    .map(|cr| cr == wt.rel_path)
                    .unwrap_or(false);
                let marker = if is_current { "*" } else { " " };
                println!(
                    "    {} {} {}    [{}] {}",
                    connector, marker, wt.rel_path, wt.branch, wt.hash
                );
            }
        }
    }

    // --- Current Workspace section (store files) ---
    if let Some(ref ws) = ctx.current_workspace
        && ws.store_dir.is_some()
        && !ws.manifest.is_empty()
    {
        if has_output {
            println!();
        }
        has_output = true;

        let ws_name = ws
            .root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        println!(
            "{}",
            t!(
                "status.current_workspace",
                name = format!("{} [{}]", ws_name, ws.branch)
            )
        );

        let store = ws.store_dir.as_ref().unwrap();
        let wt_root = Some(ws.root.clone());

        let mut rows = Vec::new();
        for entry in &ws.manifest {
            let store_file = store.join(&entry.filepath);
            let status = file_status(entry, &store_file, &wt_root);
            rows.push(vec![
                entry.strategy.to_string(),
                entry.filepath.clone(),
                status.to_string(),
            ]);
        }

        print_table(&["STRATEGY", "FILE", "STATUS"], &rows, 2);
    }

    if !has_output {
        println!("{}", t!("repos.no_repos"));
    }

    Ok(())
}
