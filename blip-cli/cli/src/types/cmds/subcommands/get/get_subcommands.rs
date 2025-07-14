use clap::Subcommand;

use crate::types::cmds::subcommands::get::{Context, Key, Thread};

#[derive(Subcommand, Debug)]
pub enum GetSubcommands {
    /// bot auth key
    Key(Key),

    /// user context value
    Context(Context),

    /// last user messages
    Thread(Thread)
}