use clap::Args;
use contexts::{store, test};
use domain::traits::cli::Runnable;
use tester::types::TestTemplate;
use chat::params::ChatParams;
use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Run {
    #[command(flatten)]
    commong_args: CommonArgs
}

impl Runnable for Run {
    fn run(&self) { 
        let files = TestTemplate::read_files(&self.commong_args.tenant, &self.commong_args.bot);
        
        for file in files {
            if let Ok(inputs) = serde_json::to_string(&file.inputs) {
                test::enter_mode();
                test::set_inputs(&inputs);
                test::reset_end_signal();
                chat::init(ChatParams {
                    tenant: self.commong_args.tenant.to_string(),
                    bot: self.commong_args.bot.to_string()
                });
                store::reset();
            }
        }
    }
}