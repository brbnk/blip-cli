use clap::{Args};
use domain::cli::{Getter, Runnable};

use crate::types::cmds::subcommands::{get::GetSubcommands};

#[derive(Args, Debug)]
pub struct GetCommand {
    #[command(subcommand)]
    pub action: GetSubcommands,
}

impl Runnable for GetCommand {
    fn run(&self) {
        match &self.action {
            GetSubcommands::Key(key) => key.get(),
            GetSubcommands::Context(context) => context.get(),
            GetSubcommands::Thread(thread) => thread.get(),
        }
    }
}