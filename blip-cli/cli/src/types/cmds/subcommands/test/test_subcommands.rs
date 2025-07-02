use clap::Subcommand;

use crate::types::cmds::subcommands::test::{Create, Run};

#[derive(Subcommand, Debug)]
pub enum TestSubcommands {
    Create(Create),
    Run(Run)
}