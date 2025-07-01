use chat;
use clap::{arg, Args, Parser, Subcommand};
use mirror::{RequestType};
use tester::types::TestTemplate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// simulate a chat
    Chat {
        #[command(flatten)]
        commong_args: CommonArgs
    },

    /// mirror an application locally
    Mirror {
        #[command(flatten)]
        commong_args: CommonArgs,

        /// mirror only working flow
        #[arg(short, long)]
        working_flow: bool,

        /// mirror only global actions
        #[arg(short, long)]
        global_actions: bool,

        /// mirror only config variables
        #[arg(short, long)]
        configurations: bool,

        /// mirror only blip functions
        #[arg(short, long)]
        blip_functions: bool,

        /// mirror only resources
        #[arg(short, long)]
        resources: bool,
    },

    /// list local bots
    List {},

    /// automate flow tests
    Test(TestCommand)
}

#[derive(Args, Debug)]
pub struct TestCommand {
    #[command(subcommand)]
    pub action: TestSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum TestSubcommands {
    Create {
        #[command(flatten)]
        commong_args: CommonArgs,
    },
    Run {
        #[command(flatten)]
        commong_args: CommonArgs,

        /// test file name
        #[arg(short, long)]
        file_name: String,
    }
}

#[derive(Args, Debug)]
pub struct CommonArgs {
    /// bot contract
    #[arg(short, long)]
    pub tenant: String,

    /// flow identifier
    #[arg(short, long)]
    pub bot: String,
}

impl CommonArgs {
    pub fn is_valid(&self) -> bool {
        !self.tenant.is_empty() && !self.bot.is_empty()
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Chat { commong_args}) => {
            if commong_args.is_valid() {
                chat::init(&commong_args.tenant, &commong_args.bot);
            }
        },
        Some(Commands::Mirror { 
            commong_args,
            working_flow, 
            global_actions, 
            configurations,
            blip_functions,
            resources 
        }) => {
            if commong_args.is_valid() {
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

                if *blip_functions {
                    request_type.push(RequestType::BlipFunction);
                }

                mirror::clone(&commong_args.tenant, &commong_args.bot, &request_type);
            }
        },
        Some(Commands::List { }) => {
            mirror::scanner::list_identifiers().expect("list of identifiers");
        },
        Some(Commands::Test(test)) => match &test.action {
            TestSubcommands::Create { commong_args } => {
                if commong_args.is_valid() {
                    TestTemplate::create_file(&commong_args.tenant, &commong_args.bot);
                }
            },
            TestSubcommands::Run { commong_args: _, file_name: _ } => todo!(),
        },
        None => {}
    }
}
