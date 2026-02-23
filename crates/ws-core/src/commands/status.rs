use anyhow::Result;
use rust_i18n::t;

use crate::commands::repos::WorktreeEntry;
use crate::context::{AppContext, abbreviate_home, print_table};
use crate::store::file_status;
use crate::ui::{self, StyledCell};

pub fn cmd_status(ctx: &AppContext) -> Result<()> {
    let mut has_output = false;

    // --- Repositories section ---
    if !ctx.config.repos.is_empty() {
        anstream::println!("{}", ui::section_header(&t!("status.repositories")));
        has_output = true;

        let mut rows = Vec::new();
        let mut markers = Vec::new();
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

            markers.push(is_current);
            rows.push(vec![
                StyledCell::plain(name.clone()),
                StyledCell::plain(display_path),
                StyledCell::new(repo_type, ui::repo_type_style(repo_type)),
            ]);
        }

        print_table(&["NAME", "PATH", "TYPE"], &rows, 2, Some(&markers));
    }

    // --- Current Repository section ---
    if let Some(ref repo) = ctx.current_repo {
        if has_output {
            anstream::println!();
        }
        has_output = true;

        let display_name = repo.name.clone().unwrap_or_else(|| {
            repo.root
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string()
        });

        anstream::println!(
            "{}",
            ui::section_header(&t!("status.current_repository", name = &display_name))
        );
        anstream::println!("  Path: {}", abbreviate_home(&repo.root));

        let worktrees: Vec<&WorktreeEntry> = repo.worktrees.iter().filter(|w| !w.is_bare).collect();

        if !worktrees.is_empty() {
            anstream::println!("  Worktrees:");

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
                let connector_str = if is_last { "└──" } else { "├──" };
                let connector = ui::styled(ui::STYLE_DIM, connector_str);
                let is_current = current_rel
                    .as_deref()
                    .map(|cr| cr == wt.rel_path)
                    .unwrap_or(false);
                let marker = if is_current {
                    ui::styled(ui::STYLE_MARKER, "*")
                } else {
                    " ".to_string()
                };
                let branch = ui::styled(ui::STYLE_INFO, &format!("[{}]", wt.branch));
                let hash = ui::styled(ui::STYLE_DIM, &wt.hash);
                anstream::println!(
                    "    {} {} {}    {} {}",
                    connector,
                    marker,
                    wt.rel_path,
                    branch,
                    hash
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
            anstream::println!();
        }
        has_output = true;

        let ws_name = ws
            .root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        anstream::println!(
            "{}",
            ui::section_header(&t!(
                "status.current_workspace",
                name = format!("{} [{}]", ws_name, ws.branch)
            ))
        );

        let store = ws.store_dir.as_ref().unwrap();
        let wt_root = Some(ws.root.clone());

        let mut rows = Vec::new();
        for entry in &ws.manifest {
            let store_file = store.join(&entry.filepath);
            let status = file_status(entry, &store_file, &wt_root);
            rows.push(vec![
                StyledCell::plain(entry.strategy.to_string()),
                StyledCell::plain(entry.filepath.clone()),
                StyledCell::new(status.to_string(), ui::status_style(&status)),
            ]);
        }

        print_table(&["STRATEGY", "FILE", "STATUS"], &rows, 2, None);
    }

    if !has_output {
        anstream::println!("{}", t!("repos.no_repos"));
    }

    Ok(())
}
