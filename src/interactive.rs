use anyhow::{bail, Context, Result};
use inquire::{Select, Text};
use rust_i18n::t;
use std::process::Command;

use crate::commands::worktree::generate_name;
use crate::git::{find_bare_dir, git_output, is_inside_git_worktree};

pub(crate) fn interactive_mode() -> Result<()> {
    let top_items: Vec<String> = vec![
        format!("clone     {}", t!("interactive.menu.clone")),
        format!("new       {}", t!("interactive.menu.new")),
        format!("rm        {}", t!("interactive.menu.rm")),
        format!("list      {}", t!("interactive.menu.list")),
        format!("status    {}", t!("interactive.menu.status")),
        format!("shared    {}", t!("interactive.menu.shared")),
    ];

    let items_ref: Vec<&str> = top_items.iter().map(|s| s.as_str()).collect();
    let selected = Select::new(&t!("interactive.select_command").to_string(), items_ref)
        .prompt_skippable()
        .context(t!("interactive.selection_failed").to_string())?;

    let selected = match selected {
        Some(s) => s,
        None => {
            println!("{}", t!("interactive.cancelled"));
            return Ok(());
        }
    };

    let cmd = selected.split_whitespace().next().unwrap_or("");

    let args = match cmd {
        "clone" => interactive_clone()?,
        "new" => interactive_new()?,
        "rm" => interactive_rm()?,
        "list" => vec!["list".to_string()],
        "status" => vec!["status".to_string()],
        "shared" => interactive_shared()?,
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    };

    let cmd_str = format!("ws {}", args.join(" "));
    eprintln!("> {}", cmd_str);

    let status = Command::new("ws")
        .args(&args)
        .status()
        .with_context(|| t!("interactive.exec_failed", cmd = &cmd_str).to_string())?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

fn interactive_clone() -> Result<Vec<String>> {
    let url_input = Text::new(&t!("interactive.clone.url_prompt").to_string())
        .with_help_message(&t!("interactive.clone.url_help").to_string())
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let mut args = vec!["clone".to_string()];
    if !url_input.is_empty() {
        args.push(url_input);
    }
    Ok(args)
}

fn interactive_new() -> Result<Vec<String>> {
    let default_name = generate_name();
    let name_input = Text::new(&t!("interactive.new.name_prompt").to_string())
        .with_default(&default_name)
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or(default_name.clone());

    let name = if name_input.is_empty() {
        default_name
    } else {
        name_input
    };

    let is_bare_root = !is_inside_git_worktree() && find_bare_dir().is_some();
    let default_dir = if is_bare_root {
        name.clone()
    } else {
        format!("../{}", name)
    };
    let dir_input = Text::new(&t!("interactive.new.dir_prompt").to_string())
        .with_default(&default_dir)
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let default_branch = name.clone();
    let branch_input = Text::new(&t!("interactive.new.branch_prompt").to_string())
        .with_default(&default_branch)
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let mut args = vec!["new".to_string(), name];
    if !dir_input.is_empty() && dir_input != default_dir {
        args.push("-d".to_string());
        args.push(dir_input);
    }
    if !branch_input.is_empty() && branch_input != default_branch {
        args.push("--branch".to_string());
        args.push(branch_input);
    }

    let from_input = Text::new(&t!("interactive.new.from_prompt").to_string())
        .with_help_message(&t!("interactive.new.from_help").to_string())
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    if !from_input.is_empty() {
        args.push("--from".to_string());
        args.push(from_input);
    }
    Ok(args)
}

fn interactive_rm() -> Result<Vec<String>> {
    let worktree_list = git_output(&["worktree", "list"])?;
    let lines: Vec<&str> = worktree_list.lines().skip(1).collect();

    if lines.is_empty() {
        bail!("{}", t!("interactive.rm.no_worktrees"));
    }

    let selected = Select::new(&t!("interactive.rm.select_worktree").to_string(), lines)
        .prompt_skippable()
        .context(t!("interactive.selection_failed").to_string())?;

    let selected = match selected {
        Some(s) => s,
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let path = selected
        .split_whitespace()
        .next()
        .context(t!("interactive.rm.path_failed").to_string())?
        .to_string();

    Ok(vec!["rm".to_string(), path])
}

fn interactive_shared() -> Result<Vec<String>> {
    let shared_items: Vec<String> = vec![
        format!("track     {}", t!("interactive.shared_menu.track")),
        format!("status    {}", t!("interactive.shared_menu.status")),
        format!("push      {}", t!("interactive.shared_menu.push")),
        format!("pull      {}", t!("interactive.shared_menu.pull")),
    ];

    let items_ref: Vec<&str> = shared_items.iter().map(|s| s.as_str()).collect();
    let selected = Select::new(&t!("interactive.shared_select").to_string(), items_ref)
        .prompt_skippable()
        .context(t!("interactive.selection_failed").to_string())?;

    let selected = match selected {
        Some(s) => s,
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let cmd = selected.split_whitespace().next().unwrap_or("");

    match cmd {
        "track" => interactive_shared_track(),
        "status" => Ok(vec!["shared".to_string(), "status".to_string()]),
        "push" => {
            let file_input = Text::new(&t!("interactive.shared_push.file_prompt").to_string())
                .with_help_message(&t!("interactive.shared_push.file_help").to_string())
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or_default();

            let mut args = vec!["shared".to_string(), "push".to_string()];
            if !file_input.is_empty() {
                args.push(file_input);
            }
            Ok(args)
        }
        "pull" => {
            let file_input = Text::new(&t!("interactive.shared_pull.file_prompt").to_string())
                .with_help_message(&t!("interactive.shared_pull.file_help").to_string())
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or_default();

            let mut args = vec!["shared".to_string(), "pull".to_string()];
            if !file_input.is_empty() {
                args.push(file_input);
            }
            Ok(args)
        }
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    }
}

fn interactive_shared_track() -> Result<Vec<String>> {
    let strategy_items = vec!["symlink", "copy"];
    let strategy = Select::new(&t!("interactive.shared_track.select_strategy").to_string(), strategy_items)
        .prompt_skippable()
        .context(t!("interactive.selection_failed").to_string())?;

    let strategy = match strategy {
        Some(s) => s.to_string(),
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let file = Text::new(&t!("interactive.shared_track.file_prompt").to_string())
        .prompt()
        .context(t!("interactive.input_failed").to_string())?;

    if file.is_empty() {
        bail!("{}", t!("interactive.shared_track.empty_file"));
    }

    Ok(vec![
        "shared".to_string(),
        "track".to_string(),
        "-s".to_string(),
        strategy,
        file,
    ])
}
