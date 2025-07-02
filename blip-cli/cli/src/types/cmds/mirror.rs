use clap::Args;
use domain::traits::cli::Runnable;
use mirror::RequestType;

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Mirror {
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
}

impl Runnable for Mirror {
    fn run(&self) {
        if self.commong_args.is_valid() {
            let mut request_type: Vec<RequestType> = Vec::new();

            if self.working_flow {
                request_type.push(RequestType::WorkingFlow);
            }

            if self.global_actions {
                request_type.push(RequestType::GlobalAction);
            }

            if self.configurations {
                request_type.push(RequestType::Configurations);
            }

            if self.resources {
                request_type.push(RequestType::Resources);
            }

            if self.blip_functions {
                request_type.push(RequestType::BlipFunction);
            }

            mirror::clone(&self.commong_args.tenant, &self.commong_args.bot, &request_type);
        }
    }
}
