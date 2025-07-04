use chat::params::ChatParams;
use clap::Args;
use domain::traits::cli::Runnable;

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Chat {
    #[command(flatten)]
    commong_args: CommonArgs,
}

impl Runnable for Chat {
    fn run(&self) {
        if self.commong_args.is_valid() {
            chat::init(ChatParams {
                tenant: self.commong_args.tenant.to_string(),
                bot: self.commong_args.bot.to_string()
            });
        }
    }
}
