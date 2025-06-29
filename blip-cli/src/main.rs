use chat;
use clap::{arg, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// simulate a chat
    Chat {
        /// flow identifier
        #[arg(short, long)]
        identifier: String,
    },

    /// mirror an application locally
    Mirror {
        #[arg(short, long)]
        identifier: String
    }
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Chat { identifier }) => {
            if !identifier.is_empty() {
                chat::init(identifier);
            }
        },
        Some(Commands::Mirror { identifier }) => {
            if !identifier.is_empty() {
                mirror::clone_application(identifier);
            }
        },
        None => {}
    }
}
