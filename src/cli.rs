use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use rust_i18n::t;

#[derive(Parser)]
#[command(name = "ws", version)]
pub(crate) struct Ws {
    #[command(subcommand)]
    pub command: WsCommand,
}

#[derive(Subcommand)]
pub(crate) enum WsCommand {
    Clone(CloneCmd),
    New(NewCmd),
    Rm(RmCmd),
    List(ListCmd),
    Status(StatusCmd),
    Store(StoreCmd),
    I(InteractiveCmd),
}

#[derive(Parser)]
pub(crate) struct CloneCmd {
    pub url: Option<String>,
}

#[derive(Parser)]
pub(crate) struct NewCmd {
    pub name: Option<String>,

    #[arg(short = 'd', long)]
    pub directory: Option<String>,

    #[arg(long)]
    pub branch: Option<String>,

    #[arg(long = "from")]
    pub from: Option<String>,
}

#[derive(Parser)]
pub(crate) struct RmCmd {
    pub directory: String,

    #[arg(short = 'f', long)]
    pub force: bool,
}

#[derive(Parser)]
pub(crate) struct ListCmd {}

#[derive(Parser)]
pub(crate) struct InteractiveCmd {}

#[derive(Parser)]
pub(crate) struct StatusCmd {}

#[derive(Parser)]
pub(crate) struct StoreCmd {
    #[command(subcommand)]
    pub command: StoreCommand,
}

#[derive(Subcommand)]
pub(crate) enum StoreCommand {
    Track(StoreTrackCmd),
    Status(StoreStatusCmd),
    Push(StorePushCmd),
    Pull(StorePullCmd),
    Untrack(StoreUntrackCmd),
}

#[derive(Parser)]
pub(crate) struct StoreTrackCmd {
    #[arg(short = 's', long)]
    pub strategy: String,

    pub file: String,
}

#[derive(Parser)]
pub(crate) struct StoreStatusCmd {}

#[derive(Parser)]
pub(crate) struct StorePushCmd {
    pub file: Option<String>,
}

#[derive(Parser)]
pub(crate) struct StorePullCmd {
    pub file: Option<String>,

    #[arg(short = 'f', long)]
    pub force: bool,
}

#[derive(Parser)]
pub(crate) struct StoreUntrackCmd {
    pub file: String,
}

/// derive で定義した Command にランタイムで i18n ヘルプを上書きしてパース
pub(crate) fn parse_with_i18n() -> Ws {
    let cmd = Ws::command()
        .about(t!("cli.about").to_string())
        .mut_subcommand("clone", |s| {
            s.about(t!("cli.clone.about").to_string())
                .mut_arg("url", |a| a.help(t!("cli.clone.url").to_string()))
        })
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
        .mut_subcommand("list", |s| {
            s.about(t!("cli.list.about").to_string())
        })
        .mut_subcommand("status", |s| {
            s.about(t!("cli.status.about").to_string())
        })
        .mut_subcommand("i", |s| {
            s.about(t!("cli.i.about").to_string())
        })
        .mut_subcommand("store", |s| {
            s.about(t!("cli.store.about").to_string())
                .mut_subcommand("track", |ss| {
                    ss.about(t!("cli.store.track.about").to_string())
                        .mut_arg("strategy", |a| {
                            a.help(t!("cli.store.track.strategy").to_string())
                        })
                        .mut_arg("file", |a| {
                            a.help(t!("cli.store.track.file").to_string())
                        })
                })
                .mut_subcommand("status", |ss| {
                    ss.about(t!("cli.store.status.about").to_string())
                })
                .mut_subcommand("push", |ss| {
                    ss.about(t!("cli.store.push.about").to_string())
                        .mut_arg("file", |a| {
                            a.help(t!("cli.store.push.file").to_string())
                        })
                })
                .mut_subcommand("pull", |ss| {
                    ss.about(t!("cli.store.pull.about").to_string())
                        .mut_arg("file", |a| {
                            a.help(t!("cli.store.pull.file").to_string())
                        })
                        .mut_arg("force", |a| {
                            a.help(t!("cli.store.pull.force").to_string())
                        })
                })
                .mut_subcommand("untrack", |ss| {
                    ss.about(t!("cli.store.untrack.about").to_string())
                        .mut_arg("file", |a| {
                            a.help(t!("cli.store.untrack.file").to_string())
                        })
                })
        });

    let matches = cmd.get_matches();
    Ws::from_arg_matches(&matches).expect("CLI parse error")
}
