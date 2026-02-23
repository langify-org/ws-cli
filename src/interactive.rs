use anyhow::{Context, Result, bail};
use inquire::{Select, Text};
use rust_i18n::t;
use std::process::Command;

use crate::cli::WsCommand;
use crate::commands::worktree::generate_name;
use crate::config::load_config;
use crate::git::{find_bare_dir, git_output, is_inside_git_worktree};
use crate::store::{read_manifest, require_store};

pub(crate) fn interactive_mode() -> Result<()> {
    let top_items: Vec<String> = vec![
        format!("clone     {}", t!("interactive.menu.clone")),
        format!("new       {}", t!("interactive.menu.new")),
        format!("rm        {}", t!("interactive.menu.rm")),
        format!("list      {}", t!("interactive.menu.list")),
        format!("status    {}", t!("interactive.menu.status")),
        format!("store     {}", t!("interactive.menu.store")),
        format!("repos     {}", t!("interactive.menu.repos")),
    ];

    let items_ref: Vec<&str> = top_items.iter().map(|s| s.as_str()).collect();
    let selected = Select::new(&t!("interactive.select_command"), items_ref)
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
        "store" => interactive_store()?,
        "repos" => interactive_repos()?,
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
    let url_input = Text::new(&t!("interactive.clone.url_prompt"))
        .with_help_message(&t!("interactive.clone.url_help"))
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
    let name_input = Text::new(&t!("interactive.new.name_prompt"))
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
    let dir_input = Text::new(&t!("interactive.new.dir_prompt"))
        .with_default(&default_dir)
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let default_branch = name.clone();
    let branch_input = Text::new(&t!("interactive.new.branch_prompt"))
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

    let from_input = Text::new(&t!("interactive.new.from_prompt"))
        .with_help_message(&t!("interactive.new.from_help"))
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

    let selected = Select::new(&t!("interactive.rm.select_worktree"), lines)
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

fn interactive_store() -> Result<Vec<String>> {
    let store_items: Vec<String> = vec![
        format!("track     {}", t!("interactive.store_menu.track")),
        format!("status    {}", t!("interactive.store_menu.status")),
        format!("push      {}", t!("interactive.store_menu.push")),
        format!("pull      {}", t!("interactive.store_menu.pull")),
        format!("untrack   {}", t!("interactive.store_menu.untrack")),
    ];

    let items_ref: Vec<&str> = store_items.iter().map(|s| s.as_str()).collect();
    let selected = Select::new(&t!("interactive.store_select"), items_ref)
        .prompt_skippable()
        .context(t!("interactive.selection_failed").to_string())?;

    let selected = match selected {
        Some(s) => s,
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let cmd = selected.split_whitespace().next().unwrap_or("");

    match cmd {
        "track" => interactive_store_track(),
        "status" => Ok(vec!["store".to_string(), "status".to_string()]),
        "push" => {
            let file_input = Text::new(&t!("interactive.store_push.file_prompt"))
                .with_help_message(&t!("interactive.store_push.file_help"))
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or_default();

            let mut args = vec!["store".to_string(), "push".to_string()];
            if !file_input.is_empty() {
                args.push(file_input);
            }
            Ok(args)
        }
        "pull" => {
            let file_input = Text::new(&t!("interactive.store_pull.file_prompt"))
                .with_help_message(&t!("interactive.store_pull.file_help"))
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or_default();

            let mut args = vec!["store".to_string(), "pull".to_string()];
            if !file_input.is_empty() {
                args.push(file_input);
            }
            Ok(args)
        }
        "untrack" => interactive_store_untrack(),
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    }
}

fn interactive_store_track() -> Result<Vec<String>> {
    let strategy_items = vec!["symlink", "copy"];
    let strategy = Select::new(
        &t!("interactive.store_track.select_strategy"),
        strategy_items,
    )
    .prompt_skippable()
    .context(t!("interactive.selection_failed").to_string())?;

    let strategy = match strategy {
        Some(s) => s.to_string(),
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let file = Text::new(&t!("interactive.store_track.file_prompt"))
        .prompt()
        .context(t!("interactive.input_failed").to_string())?;

    if file.is_empty() {
        bail!("{}", t!("interactive.store_track.empty_file"));
    }

    Ok(vec![
        "store".to_string(),
        "track".to_string(),
        "-s".to_string(),
        strategy,
        file,
    ])
}

fn interactive_store_untrack() -> Result<Vec<String>> {
    let store = require_store();
    if let Ok(store) = store {
        let entries = read_manifest(&store)?;
        if !entries.is_empty() {
            let file_list: Vec<String> = entries.iter().map(|e| e.filepath.clone()).collect();
            let items_ref: Vec<&str> = file_list.iter().map(|s| s.as_str()).collect();
            let selected = Select::new(&t!("interactive.store_untrack.select_file"), items_ref)
                .prompt_skippable()
                .context(t!("interactive.selection_failed").to_string())?;

            return match selected {
                Some(s) => Ok(vec![
                    "store".to_string(),
                    "untrack".to_string(),
                    s.to_string(),
                ]),
                None => bail!("{}", t!("interactive.cancelled")),
            };
        }
    }

    // store 未初期化 or 追跡ファイルなしの場合はテキスト入力にフォールバック
    let file = Text::new(&t!("interactive.store_untrack.file_prompt"))
        .prompt()
        .context(t!("interactive.input_failed").to_string())?;

    if file.is_empty() {
        bail!("{}", t!("interactive.store_untrack.empty_file"));
    }

    Ok(vec!["store".to_string(), "untrack".to_string(), file])
}

fn interactive_repos() -> Result<Vec<String>> {
    let repos_items: Vec<String> = vec![
        format!("add       {}", t!("interactive.repos_menu.add")),
        format!("list      {}", t!("interactive.repos_menu.list")),
        format!("rm        {}", t!("interactive.repos_menu.rm")),
    ];

    let items_ref: Vec<&str> = repos_items.iter().map(|s| s.as_str()).collect();
    let selected = Select::new(&t!("interactive.repos_select"), items_ref)
        .prompt_skippable()
        .context(t!("interactive.selection_failed").to_string())?;

    let selected = match selected {
        Some(s) => s,
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let cmd = selected.split_whitespace().next().unwrap_or("");

    match cmd {
        "add" => interactive_repos_add(),
        "list" => Ok(vec!["repos".to_string(), "list".to_string()]),
        "rm" => interactive_repos_rm(),
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    }
}

fn interactive_repos_add() -> Result<Vec<String>> {
    let path_input = Text::new(&t!("interactive.repos_add.path_prompt"))
        .with_help_message(&t!("interactive.repos_add.path_help"))
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let name_input = Text::new(&t!("interactive.repos_add.name_prompt"))
        .with_help_message(&t!("interactive.repos_add.name_help"))
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let mut args = vec!["repos".to_string(), "add".to_string()];
    if !path_input.is_empty() {
        args.push(path_input);
    }
    if !name_input.is_empty() {
        args.push("--name".to_string());
        args.push(name_input);
    }
    Ok(args)
}

fn interactive_repos_rm() -> Result<Vec<String>> {
    if let Ok(config) = load_config()
        && !config.repos.is_empty()
    {
        let names: Vec<String> = config.repos.keys().cloned().collect();
        let items_ref: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
        let selected = Select::new(&t!("interactive.repos_rm.select_repo"), items_ref)
            .prompt_skippable()
            .context(t!("interactive.selection_failed").to_string())?;

        return match selected {
            Some(s) => Ok(vec!["repos".to_string(), "rm".to_string(), s.to_string()]),
            None => bail!("{}", t!("interactive.cancelled")),
        };
    }

    // config 読み込み失敗 or リポジトリ未登録の場合はテキスト入力にフォールバック
    let name = Text::new(&t!("interactive.repos_rm.name_prompt"))
        .prompt()
        .context(t!("interactive.input_failed").to_string())?;

    if name.is_empty() {
        bail!("{}", t!("interactive.repos_rm.empty_name"));
    }

    Ok(vec!["repos".to_string(), "rm".to_string(), name])
}

/// WsCommand に新バリアントが追加されるとここでコンパイルエラーになる。
/// 対話メニュー (interactive_mode) も更新すること。
#[allow(dead_code)]
fn _ensure_all_commands_in_interactive(cmd: &WsCommand) -> &'static str {
    match cmd {
        WsCommand::Clone(_) => "clone",
        WsCommand::New(_) => "new",
        WsCommand::Rm(_) => "rm",
        WsCommand::List(_) => "list",
        WsCommand::Status(_) => "status",
        WsCommand::Store(_) => "store",
        WsCommand::Repos(_) => "repos",
        WsCommand::I(_) => "i",
    }
}
