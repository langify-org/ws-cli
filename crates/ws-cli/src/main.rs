mod interactive;

use anyhow::Result;
use rust_i18n::t;

use ws_core::cli::{ReposCommand, StoreCommand, WsCommand};

rust_i18n::i18n!("../../locales", fallback = "en");

fn run(ws: ws_core::cli::Ws) -> Result<()> {
    match ws.command {
        WsCommand::New(cmd) => ws_core::commands::worktree::cmd_new(&cmd),
        WsCommand::Rm(cmd) => ws_core::commands::worktree::cmd_rm(&cmd),
        WsCommand::Open(cmd) => ws_core::commands::open::cmd_open(&cmd),
        WsCommand::Status(_) => {
            let ctx = ws_core::context::AppContext::build()?;
            ws_core::commands::status::cmd_status(&ctx)
        }
        WsCommand::Interactive(_) => interactive::interactive_mode(),
        WsCommand::Completions(cmd) => {
            ws_core::commands::completions::cmd_completions(&cmd);
            Ok(())
        }
        WsCommand::Repos(cmd) => match cmd.command {
            ReposCommand::Clone(c) => ws_core::commands::worktree::cmd_clone(&c),
            ReposCommand::Add(c) => ws_core::commands::repos::cmd_repos_add(&c),
            ReposCommand::List(_) => {
                let ctx = ws_core::context::AppContext::build()?;
                ws_core::commands::repos::cmd_repos_list(&ctx)
            }
            ReposCommand::Rm(c) => ws_core::commands::repos::cmd_repos_rm(&c),
        },
        WsCommand::Store(cmd) => match cmd.command {
            StoreCommand::Track(c) => ws_core::commands::store::cmd_store_track(&c),
            StoreCommand::Status(_) => ws_core::commands::store::cmd_store_status(),
            StoreCommand::Push(c) => ws_core::commands::store::cmd_store_push(&c),
            StoreCommand::Pull(c) => ws_core::commands::store::cmd_store_pull(&c),
            StoreCommand::Untrack(c) => ws_core::commands::store::cmd_store_untrack(&c),
        },
    }
}

fn main() -> std::process::ExitCode {
    ws_core::detect_and_set_locale();
    let ws = ws_core::cli::parse_with_i18n();
    match run(ws) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            anstream::eprintln!(
                "{}",
                ws_core::ui::styled(
                    ws_core::ui::STYLE_ERROR_BOLD,
                    &t!("error.top", detail = format!("{:#}", e))
                )
            );
            std::process::ExitCode::FAILURE
        }
    }
}
