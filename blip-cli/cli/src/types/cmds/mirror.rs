use clap::Args;
use domain::cli::Runnable;
use mirror::RequestType;

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Mirror {
    #[command(flatten)]
    commong_args: CommonArgs,

    /// tier contract
    #[arg(short, long)]
    tier: Option<String>,

    /// router applicatino
    #[arg(long)]
    router: bool,

    /// mirror all
    #[arg(short, long)]
    all: bool,

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
}

impl Runnable for Mirror {
    fn run(&self) {
        let tier = match &self.tier {
            Some(t) => t,
            None => "standard"
        };

        if self.commong_args.is_valid() {
            let mut request_type: Vec<RequestType> = Vec::new();

            if self.router {
                request_type.push(RequestType::Router);
            }
            else {
                if self.working_flow || self.all {
                    request_type.push(RequestType::WorkingFlow);
                }
    
                if self.global_actions || self.all {
                    request_type.push(RequestType::GlobalAction);
                }
    
                if self.configurations || self.all {
                    request_type.push(RequestType::Configurations);
                }
    
                if self.resources || self.all {
                    request_type.push(RequestType::Resources);
                }
    
                if self.blip_functions || self.all {
                    request_type.push(RequestType::BlipFunction);
                }
            }

            mirror::clone(&self.commong_args.tenant, &self.commong_args.bot, &tier, &request_type);
        }
    }
}
