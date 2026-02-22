mod cli;
mod commands;
mod git;
mod interactive;
mod store;

use anyhow::Result;
use rust_i18n::t;

use cli::{SharedCommand, WsCommand};

rust_i18n::i18n!("locales", fallback = "en");

fn detect_and_set_locale() {
    let locale = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LANG"))
        .ok()
        .and_then(|v| {
            let v = v.trim().to_string();
            if v.is_empty() || v == "C" || v == "POSIX" {
                None
            } else {
                Some(v)
            }
        })
        .unwrap_or_else(|| {
            sys_locale::get_locale().unwrap_or_else(|| "en".to_string())
        });

    let normalized = if locale.starts_with("ja") {
        "ja"
    } else if locale.starts_with("zh") {
        "zh-CN"
    } else {
        "en"
    };
    rust_i18n::set_locale(normalized);
}

fn run(ws: cli::Ws) -> Result<()> {
    match ws.command {
        WsCommand::Clone(cmd) => commands::worktree::cmd_clone(&cmd),
        WsCommand::New(cmd) => commands::worktree::cmd_new(&cmd),
        WsCommand::Rm(cmd) => commands::worktree::cmd_rm(&cmd),
        WsCommand::List(_) => commands::worktree::cmd_list(),
        WsCommand::Status(_) => commands::status::cmd_status(),
        WsCommand::I(_) => interactive::interactive_mode(),
        WsCommand::Shared(cmd) => match cmd.command {
            SharedCommand::Track(c) => commands::shared::cmd_shared_track(&c),
            SharedCommand::Status(_) => commands::shared::cmd_shared_status(),
            SharedCommand::Push(c) => commands::shared::cmd_shared_push(&c),
            SharedCommand::Pull(c) => commands::shared::cmd_shared_pull(&c),
        },
    }
}

fn main() {
    detect_and_set_locale();
    let ws = cli::parse_with_i18n();
    if let Err(e) = run(ws) {
        eprintln!("{}", t!("error.top", detail = format!("{:#}", e)));
        std::process::exit(1);
    }
}
