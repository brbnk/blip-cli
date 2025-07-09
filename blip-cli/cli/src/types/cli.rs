use clap::Parser;

use crate::types::Commands;

use domain::cli::Runnable;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn build() -> Self {
        Self::parse()
    }

    pub fn run(&self) {
        if let Some(commands) = &self.command {
            match commands {
                Commands::Chat(chat ) => chat.run(),
                Commands::Mirror(mirror) => mirror.run(),
                Commands::List {} => mirror::scanner::list_identifiers().expect("list of identifiers"),
                Commands::Test(test) => test.run(),
                Commands::Get(get_command) => get_command.run(),
            }
        }
    }
}
