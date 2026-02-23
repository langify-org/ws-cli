use anyhow::{Context, Result, bail};
use inquire::{Confirm, Select, Text};
use rust_i18n::t;

use ws_core::cli::WsCommand;
use ws_core::config::load_config;
use ws_core::git::{find_bare_dir, is_inside_git_worktree};
use ws_core::store::{Strategy, read_manifest, require_store};

pub(crate) fn interactive_mode() -> Result<()> {
    let top_items: Vec<String> = vec![
        format!("new       {}", t!("interactive.menu.new")),
        format!("rm        {}", t!("interactive.menu.rm")),
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

    match cmd {
        "new" => interactive_new(),
        "rm" => interactive_rm(),
        "status" => {
            eprintln!("> ws status");
            let ctx = ws_core::context::AppContext::build()?;
            ws_core::commands::status::cmd_status(&ctx)
        }
        "store" => interactive_store(),
        "repos" => interactive_repos(),
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    }
}

fn interactive_clone() -> Result<()> {
    let url_input = Text::new(&t!("interactive.clone.url_prompt"))
        .with_help_message(&t!("interactive.clone.url_help"))
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let cmd = ws_core::cli::CloneCmd {
        url: if url_input.is_empty() {
            None
        } else {
            Some(url_input.clone())
        },
    };
    eprintln!(
        "> ws repos clone{}",
        if url_input.is_empty() {
            String::new()
        } else {
            format!(" {}", url_input)
        }
    );
    ws_core::commands::worktree::cmd_clone(&cmd)
}

fn interactive_new() -> Result<()> {
    let name = Text::new(&t!("interactive.new.name_prompt"))
        .prompt()
        .context(t!("interactive.input_failed").to_string())?;

    if name.is_empty() {
        bail!("{}", t!("interactive.new.empty_name"));
    }

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

    let from_input = Text::new(&t!("interactive.new.from_prompt"))
        .with_help_message(&t!("interactive.new.from_help"))
        .prompt_skippable()
        .context(t!("interactive.input_failed").to_string())?
        .unwrap_or_default();

    let directory = if !dir_input.is_empty() && dir_input != default_dir {
        Some(dir_input)
    } else {
        None
    };

    let branch = if !branch_input.is_empty() && branch_input != default_branch {
        Some(branch_input)
    } else {
        None
    };

    let from = if !from_input.is_empty() {
        Some(from_input)
    } else {
        None
    };

    let cmd = ws_core::cli::NewCmd {
        name: name.clone(),
        directory,
        branch,
        from,
    };

    let mut cmd_str = format!("ws new {}", name);
    if let Some(ref d) = cmd.directory {
        cmd_str.push_str(&format!(" -d {}", d));
    }
    if let Some(ref b) = cmd.branch {
        cmd_str.push_str(&format!(" --branch {}", b));
    }
    if let Some(ref f) = cmd.from {
        cmd_str.push_str(&format!(" --from {}", f));
    }
    eprintln!("> {}", cmd_str);

    ws_core::commands::worktree::cmd_new(&cmd)
}

fn interactive_rm() -> Result<()> {
    let worktree_list = ws_core::git::git_output(&["worktree", "list"])?;
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

    let cmd = ws_core::cli::RmCmd {
        directory: path.clone(),
        force: false,
    };
    eprintln!("> ws rm {}", path);
    ws_core::commands::worktree::cmd_rm(&cmd)
}

fn interactive_store() -> Result<()> {
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
        "status" => {
            eprintln!("> ws store status");
            ws_core::commands::store::cmd_store_status()
        }
        "push" => {
            let file_input = Text::new(&t!("interactive.store_push.file_prompt"))
                .with_help_message(&t!("interactive.store_push.file_help"))
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or_default();

            let cmd = ws_core::cli::StorePushCmd {
                file: if file_input.is_empty() {
                    None
                } else {
                    Some(file_input.clone())
                },
            };
            eprintln!(
                "> ws store push{}",
                if file_input.is_empty() {
                    String::new()
                } else {
                    format!(" {}", file_input)
                }
            );
            ws_core::commands::store::cmd_store_push(&cmd)
        }
        "pull" => {
            let file_input = Text::new(&t!("interactive.store_pull.file_prompt"))
                .with_help_message(&t!("interactive.store_pull.file_help"))
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or_default();

            let force = Confirm::new(&t!("interactive.store_pull.force_prompt"))
                .with_default(false)
                .prompt_skippable()
                .context(t!("interactive.input_failed").to_string())?
                .unwrap_or(false);

            let cmd = ws_core::cli::StorePullCmd {
                file: if file_input.is_empty() {
                    None
                } else {
                    Some(file_input.clone())
                },
                force,
            };
            eprintln!(
                "> ws store pull{}{}",
                if force { " --force" } else { "" },
                if file_input.is_empty() {
                    String::new()
                } else {
                    format!(" {}", file_input)
                }
            );
            ws_core::commands::store::cmd_store_pull(&cmd)
        }
        "untrack" => interactive_store_untrack(),
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    }
}

fn interactive_store_track() -> Result<()> {
    let strategy_items = vec![Strategy::Symlink, Strategy::Copy];
    let display_items: Vec<&str> = strategy_items.iter().map(|s| s.as_str()).collect();
    let selected = Select::new(
        &t!("interactive.store_track.select_strategy"),
        display_items,
    )
    .prompt_skippable()
    .context(t!("interactive.selection_failed").to_string())?;

    let strategy = match selected {
        Some(s) => strategy_items
            .into_iter()
            .find(|item| item.as_str() == s)
            .expect("selected item must exist in strategy_items"),
        None => bail!("{}", t!("interactive.cancelled")),
    };

    let file = Text::new(&t!("interactive.store_track.file_prompt"))
        .prompt()
        .context(t!("interactive.input_failed").to_string())?;

    if file.is_empty() {
        bail!("{}", t!("interactive.store_track.empty_file"));
    }

    eprintln!("> ws store track -s {} {}", strategy, &file);
    let cmd = ws_core::cli::StoreTrackCmd { strategy, file };
    ws_core::commands::store::cmd_store_track(&cmd)
}

fn interactive_store_untrack() -> Result<()> {
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
                Some(s) => {
                    let cmd = ws_core::cli::StoreUntrackCmd {
                        file: s.to_string(),
                    };
                    eprintln!("> ws store untrack {}", s);
                    ws_core::commands::store::cmd_store_untrack(&cmd)
                }
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

    let cmd = ws_core::cli::StoreUntrackCmd { file: file.clone() };
    eprintln!("> ws store untrack {}", file);
    ws_core::commands::store::cmd_store_untrack(&cmd)
}

fn interactive_repos() -> Result<()> {
    let repos_items: Vec<String> = vec![
        format!("clone     {}", t!("interactive.repos_menu.clone")),
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
        "clone" => interactive_clone(),
        "add" => interactive_repos_add(),
        "list" => {
            eprintln!("> ws repos list");
            let ctx = ws_core::context::AppContext::build()?;
            ws_core::commands::repos::cmd_repos_list(&ctx)
        }
        "rm" => interactive_repos_rm(),
        _ => bail!("{}", t!("interactive.unknown_command", cmd = cmd)),
    }
}

fn interactive_repos_add() -> Result<()> {
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

    let cmd = ws_core::cli::ReposAddCmd {
        path: if path_input.is_empty() {
            None
        } else {
            Some(path_input.clone())
        },
        name: if name_input.is_empty() {
            None
        } else {
            Some(name_input.clone())
        },
    };

    let mut cmd_str = "ws repos add".to_string();
    if !path_input.is_empty() {
        cmd_str.push_str(&format!(" {}", path_input));
    }
    if !name_input.is_empty() {
        cmd_str.push_str(&format!(" --name {}", name_input));
    }
    eprintln!("> {}", cmd_str);

    ws_core::commands::repos::cmd_repos_add(&cmd)
}

fn interactive_repos_rm() -> Result<()> {
    if let Ok(config) = load_config()
        && !config.repos.is_empty()
    {
        let names: Vec<String> = config.repos.keys().cloned().collect();
        let items_ref: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
        let selected = Select::new(&t!("interactive.repos_rm.select_repo"), items_ref)
            .prompt_skippable()
            .context(t!("interactive.selection_failed").to_string())?;

        return match selected {
            Some(s) => {
                let cmd = ws_core::cli::ReposRmCmd {
                    name: s.to_string(),
                };
                eprintln!("> ws repos rm {}", s);
                ws_core::commands::repos::cmd_repos_rm(&cmd)
            }
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

    let cmd = ws_core::cli::ReposRmCmd { name: name.clone() };
    eprintln!("> ws repos rm {}", name);
    ws_core::commands::repos::cmd_repos_rm(&cmd)
}

/// WsCommand に新バリアントが追加されるとここでコンパイルエラーになる。
/// 対話メニュー (interactive_mode) も更新すること。
#[allow(dead_code)]
fn _ensure_all_commands_in_interactive(cmd: &WsCommand) -> &'static str {
    match cmd {
        WsCommand::New(_) => "new",
        WsCommand::Rm(_) => "rm",
        WsCommand::Status(_) => "status",
        WsCommand::Store(_) => "store",
        WsCommand::Repos(_) => "repos",
        WsCommand::Interactive(_) => "interactive",
        WsCommand::Completions(_) => "completions",
    }
}
