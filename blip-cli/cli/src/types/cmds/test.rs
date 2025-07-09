use clap::{Args};
use domain::cli::Runnable;

use crate::types::{cmds::subcommands::test::TestSubcommands};

#[derive(Args, Debug)]
pub struct TestCommand {
    #[command(subcommand)]
    pub action: TestSubcommands,
}

impl Runnable for TestCommand {
    fn run(&self) {
        match &self.action {
            TestSubcommands::Create(create) => create.run(),
            TestSubcommands::Run(run) => run.run(),
        }
    }
}