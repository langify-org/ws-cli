use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use clap_complete::Shell;
use rust_i18n::t;

use crate::store::Strategy;

#[derive(Parser)]
#[command(name = "ws", version)]
pub struct Ws {
    #[command(subcommand)]
    pub command: WsCommand,
}

#[derive(Subcommand)]
pub enum WsCommand {
    New(NewCmd),
    Rm(RmCmd),
    Status(StatusCmd),
    Store(StoreCmd),
    Repos(ReposCmd),
    #[command(alias = "i")]
    Interactive(InteractiveCmd),
    Completions(CompletionsCmd),
}

#[derive(Parser)]
pub struct CloneCmd {
    pub url: Option<String>,
}

#[derive(Parser)]
pub struct NewCmd {
    pub name: String,

    #[arg(short = 'd', long)]
    pub directory: Option<String>,

    #[arg(long)]
    pub branch: Option<String>,

    #[arg(long = "from")]
    pub from: Option<String>,
}

#[derive(Parser)]
pub struct RmCmd {
    pub directory: String,

    #[arg(short = 'f', long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct InteractiveCmd {}

#[derive(Parser)]
pub struct CompletionsCmd {
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Parser)]
pub struct StatusCmd {}

#[derive(Parser)]
pub struct StoreCmd {
    #[command(subcommand)]
    pub command: StoreCommand,
}

#[derive(Subcommand)]
pub enum StoreCommand {
    Track(StoreTrackCmd),
    Status(StoreStatusCmd),
    Push(StorePushCmd),
    Pull(StorePullCmd),
    Untrack(StoreUntrackCmd),
}

#[derive(Parser)]
pub struct StoreTrackCmd {
    #[arg(short = 's', long, value_enum)]
    pub strategy: Strategy,

    pub file: String,
}

#[derive(Parser)]
pub struct StoreStatusCmd {}

#[derive(Parser)]
pub struct StorePushCmd {
    pub file: Option<String>,
}

#[derive(Parser)]
pub struct StorePullCmd {
    pub file: Option<String>,

    #[arg(short = 'f', long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct StoreUntrackCmd {
    pub file: String,
}

#[derive(Parser)]
pub struct ReposCmd {
    #[command(subcommand)]
    pub command: ReposCommand,
}

#[derive(Subcommand)]
pub enum ReposCommand {
    Clone(CloneCmd),
    Add(ReposAddCmd),
    List(ReposListCmd),
    Rm(ReposRmCmd),
}

#[derive(Parser)]
pub struct ReposAddCmd {
    pub path: Option<String>,

    #[arg(long)]
    pub name: Option<String>,
}

#[derive(Parser)]
pub struct ReposListCmd {}

#[derive(Parser)]
pub struct ReposRmCmd {
    pub name: String,
}

/// derive で定義した Command にランタイムで i18n ヘルプを上書きしてパース
pub fn parse_with_i18n() -> Ws {
    let cmd = Ws::command()
        .about(t!("cli.about").to_string())
        .mut_subcommand("new", |s| {
            s.about(t!("cli.new.about").to_string())
                .mut_arg("name", |a| a.help(t!("cli.new.name").to_string()))
                .mut_arg("directory", |a| a.help(t!("cli.new.directory").to_string()))
                .mut_arg("branch", |a| a.help(t!("cli.new.branch").to_string()))
                .mut_arg("from", |a| a.help(t!("cli.new.from").to_string()))
        })
        .mut_subcommand("rm", |s| {
            s.about(t!("cli.rm.about").to_string())
                .mut_arg("directory", |a| a.help(t!("cli.rm.directory").to_string()))
                .mut_arg("force", |a| a.help(t!("cli.rm.force").to_string()))
        })
        .mut_subcommand("status", |s| s.about(t!("cli.status.about").to_string()))
        .mut_subcommand("interactive", |s| {
            s.about(t!("cli.interactive.about").to_string())
        })
        .mut_subcommand("completions", |s| {
            s.about(t!("cli.completions.about").to_string())
                .mut_arg("shell", |a| a.help(t!("cli.completions.shell").to_string()))
        })
        .mut_subcommand("store", |s| {
            s.about(t!("cli.store.about").to_string())
                .mut_subcommand("track", |ss| {
                    ss.about(t!("cli.store.track.about").to_string())
                        .mut_arg("strategy", |a| {
                            a.help(t!("cli.store.track.strategy").to_string())
                        })
                        .mut_arg("file", |a| a.help(t!("cli.store.track.file").to_string()))
                })
                .mut_subcommand("status", |ss| {
                    ss.about(t!("cli.store.status.about").to_string())
                })
                .mut_subcommand("push", |ss| {
                    ss.about(t!("cli.store.push.about").to_string())
                        .mut_arg("file", |a| a.help(t!("cli.store.push.file").to_string()))
                })
                .mut_subcommand("pull", |ss| {
                    ss.about(t!("cli.store.pull.about").to_string())
                        .mut_arg("file", |a| a.help(t!("cli.store.pull.file").to_string()))
                        .mut_arg("force", |a| a.help(t!("cli.store.pull.force").to_string()))
                })
                .mut_subcommand("untrack", |ss| {
                    ss.about(t!("cli.store.untrack.about").to_string())
                        .mut_arg("file", |a| a.help(t!("cli.store.untrack.file").to_string()))
                })
        })
        .mut_subcommand("repos", |s| {
            s.about(t!("cli.repos.about").to_string())
                .mut_subcommand("clone", |ss| {
                    ss.about(t!("cli.repos.clone.about").to_string())
                        .mut_arg("url", |a| a.help(t!("cli.repos.clone.url").to_string()))
                })
                .mut_subcommand("add", |ss| {
                    ss.about(t!("cli.repos.add.about").to_string())
                        .mut_arg("path", |a| a.help(t!("cli.repos.add.path").to_string()))
                        .mut_arg("name", |a| a.help(t!("cli.repos.add.name").to_string()))
                })
                .mut_subcommand("list", |ss| {
                    ss.about(t!("cli.repos.list.about").to_string())
                })
                .mut_subcommand("rm", |ss| {
                    ss.about(t!("cli.repos.rm.about").to_string())
                        .mut_arg("name", |a| a.help(t!("cli.repos.rm.name").to_string()))
                })
        });

    let matches = cmd.get_matches();
    Ws::from_arg_matches(&matches).expect("CLI parse error")
}
