use clap::CommandFactory;
use clap_complete::generate;

use crate::cli::{CompletionsCmd, Ws};

pub fn cmd_completions(cmd: &CompletionsCmd) {
    let mut cli = Ws::command();
    generate(cmd.shell, &mut cli, "ws", &mut std::io::stdout());
}
