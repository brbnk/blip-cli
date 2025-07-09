use clap::Subcommand;

use crate::types::cmds::subcommands::get::{Context, Key, Thread};

#[derive(Subcommand, Debug)]
pub enum GetSubcommands {
    Key(Key),
    Context(Context),
    Thread(Thread)
}