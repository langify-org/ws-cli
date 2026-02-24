use anyhow::{Result, bail};
use rust_i18n::t;
use std::process::{Command, Stdio};

use crate::cli::OpenCmd;
use crate::commands::repos::parse_worktree_list;
use crate::config::load_config;
use crate::ui;

pub fn cmd_open(cmd: &OpenCmd) -> Result<()> {
    let config = load_config()?;

    let entry = config
        .repos
        .get(&cmd.repository)
        .ok_or_else(|| anyhow::anyhow!("{}", t!("open.repo_not_found", name = &cmd.repository)))?;

    let repo_root = &entry.path;
    if !repo_root.exists() {
        bail!(
            "{}",
            t!(
                "open.repo_path_not_found",
                path = repo_root.display().to_string()
            )
        );
    }

    // worktree list を取得
    let is_bare = repo_root.join(".bare").is_dir();
    let wt_output = if is_bare {
        let output = Command::new("git")
            .args(["--git-dir", ".bare", "worktree", "list"])
            .current_dir(repo_root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        if !output.status.success() {
            bail!(
                "{}",
                t!("open.worktree_list_failed", name = &cmd.repository)
            );
        }
        String::from_utf8(output.stdout)?.trim().to_string()
    } else {
        let output = Command::new("git")
            .args(["worktree", "list"])
            .current_dir(repo_root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        if !output.status.success() {
            bail!(
                "{}",
                t!("open.worktree_list_failed", name = &cmd.repository)
            );
        }
        String::from_utf8(output.stdout)?.trim().to_string()
    };

    let entries = parse_worktree_list(&wt_output, repo_root);

    // bare エントリを除外し、rel_path でマッチ
    let worktree_entry = entries
        .iter()
        .filter(|e| !e.is_bare)
        .find(|e| e.rel_path == cmd.worktree);

    let worktree_path = match worktree_entry {
        Some(e) => repo_root.join(&e.rel_path),
        None => bail!(
            "{}",
            t!(
                "open.worktree_not_found",
                worktree = &cmd.worktree,
                name = &cmd.repository
            )
        ),
    };

    // エディタを解決
    let editor = resolve_editor(&cmd.editor)?;

    anstream::println!(
        "{}",
        ui::styled(
            ui::STYLE_OK,
            &t!(
                "open.opening",
                path = worktree_path.display().to_string(),
                editor = &editor
            )
        )
    );

    let status = Command::new(&editor)
        .arg(&worktree_path)
        .status()
        .map_err(|_| anyhow::anyhow!("{}", t!("open.editor_failed", editor = &editor)))?;

    if !status.success() {
        bail!("{}", t!("open.editor_failed", editor = &editor));
    }

    Ok(())
}

fn resolve_editor(flag: &Option<String>) -> Result<String> {
    if let Some(editor) = flag {
        return Ok(editor.clone());
    }
    if let Ok(visual) = std::env::var("VISUAL") {
        if !visual.is_empty() {
            return Ok(visual);
        }
    }
    if let Ok(editor) = std::env::var("EDITOR") {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }
    bail!("{}", t!("open.no_editor"));
}
