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

        let headers = ["NAME", "PATH", "TYPE"];
        let num_cols = headers.len();

        let mut rows: Vec<(bool, Vec<StyledCell>)> = Vec::new();
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
                vec![
                    StyledCell::plain(name.clone()),
                    StyledCell::plain(display_path),
                    StyledCell::new(repo_type, ui::repo_type_style(repo_type)),
                ],
            ));
        }

        // Calculate column widths
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        for (_, row) in &rows {
            for (i, cell) in row.iter().enumerate().take(num_cols) {
                widths[i] = widths[i].max(cell.plain.len());
            }
        }

        // Header (with 2-space indent, bold)
        let header_line: String = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let styled_h = ui::styled(ui::STYLE_TABLE_HEADER, h);
                if i == num_cols - 1 {
                    styled_h
                } else {
                    let padding = widths[i].saturating_sub(h.len());
                    format!("{styled_h}{}", " ".repeat(padding))
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        anstream::println!("  {header_line}");

        // Separator (dim)
        let sep_line: String = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let sep = "\u{2500}".repeat(h.len());
                let styled_sep = ui::styled(ui::STYLE_DIM, &sep);
                if i == num_cols - 1 {
                    styled_sep
                } else {
                    let padding = widths[i].saturating_sub(h.len());
                    format!("{styled_sep}{}", " ".repeat(padding))
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        anstream::println!("  {sep_line}");

        // Data rows (marker replaces 2-space indent)
        for (is_current, row) in &rows {
            let marker = if *is_current {
                ui::styled(ui::STYLE_MARKER, "*")
            } else {
                " ".to_string()
            };
            let row_line: String = (0..num_cols)
                .map(|i| {
                    let cell = row.get(i);
                    let (plain, styled) = cell
                        .map(|c| (c.plain.as_str(), c.styled.as_str()))
                        .unwrap_or(("", ""));
                    if i == num_cols - 1 {
                        styled.to_string()
                    } else {
                        let padding = widths[i].saturating_sub(plain.len());
                        format!("{styled}{}", " ".repeat(padding))
                    }
                })
                .collect::<Vec<_>>()
                .join("  ");
            anstream::println!("{marker} {row_line}");
        }
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
                StyledCell::new(status, ui::status_style(status)),
            ]);
        }

        print_table(&["STRATEGY", "FILE", "STATUS"], &rows, 2);
    }

    if !has_output {
        anstream::println!("{}", t!("repos.no_repos"));
    }

    Ok(())
}
