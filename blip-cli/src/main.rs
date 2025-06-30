use chat;
use clap::{arg, Parser, Subcommand};
use mirror::{RequestType};

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
        /// bot contract
        #[arg(short, long)]
        tenant: String,

        /// flow identifier
        #[arg(short, long)]
        bot: String,
    },

    /// mirror an application locally
    Mirror {
        /// bot contract
        #[arg(short, long)]
        tenant: String,

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

        /// mirror only resources
        #[arg(short, long)]
        resources: bool,
    },

    /// list local bots
    List {}
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Chat { tenant, bot }) => {
            if !tenant.is_empty() && !bot.is_empty() {
                chat::init(tenant, bot);
            }
        },
        Some(Commands::Mirror { 
            bot,
            tenant,
            working_flow, 
            global_actions, 
            configurations,
            resources }
        ) => {
            if !tenant.is_empty() && !bot.is_empty() {
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
                
                if *resources {
                    request_type.push(RequestType::Resources);
                }

                mirror::clone(tenant, bot, &request_type);
            }
        },
        Some(Commands::List { }) => {
            mirror::scanner::list_identifiers().expect("list of identifiers");
        }
        None => {}
    }
}
