use clap::Subcommand;

use crate::types::cmds::subcommands::test::{Create, Run};

#[derive(Subcommand, Debug)]
pub enum TestSubcommands {
    /// create a template test file
    Create(Create),

    /// run tests
    Run(Run)
}