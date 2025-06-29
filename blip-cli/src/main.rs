use chat;
use clap::{arg, Parser, Subcommand};
use mirror::{clone, RequestType};

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
        /// bot identifier
        #[arg(short, long)]
        bot: String,

        /// mirror only working flow
        #[arg(short, long)]
        working_flow: bool,

        /// mirror only global actions
        #[arg(short, long)]
        global_actions: bool,

        /// mirror only config variables
        #[arg(short, long)]
        configurations: bool,
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
        Some(Commands::Mirror { bot, working_flow, global_actions, configurations }) => {
            if !bot.is_empty() {
                let mut request_type: Vec<RequestType> = Vec::new();

                if *working_flow {
                    request_type.push(RequestType::WorkingFlow);
                }
                
                if *global_actions {
                    request_type.push(RequestType::GlobalAction);
                }
                
                if *configurations {
                    request_type.push(RequestType::Configurations);
                }

                clone(bot, &request_type);
            }
        },
        None => {}
    }
}
