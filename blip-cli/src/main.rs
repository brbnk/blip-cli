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
        bot: String,
    },

    /// mirror an application locally
    Mirror {
        #[arg(short, long)]
        bot: String
    }
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Chat { bot }) => {
            if !bot.is_empty() {
                chat::init(bot);
            }
        },
        Some(Commands::Mirror { bot }) => {
            if !bot.is_empty() {
                mirror::clone_application(bot);
            }
        },
        None => {}
    }
}
