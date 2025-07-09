use chat::params::ChatParams;
use clap::Args;
use contexts::{MocksManager};
use domain::{constants, cli::Runnable};

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Chat {
    #[command(flatten)]
    commong_args: CommonArgs,
}

impl Runnable for Chat {
    fn run(&self) {
        if self.commong_args.is_valid() {
            let tenant = &self.commong_args.tenant;
            let bot_id = &self.commong_args.bot;

            MocksManager::init(tenant, bot_id, constants::MOCKS_FILE_NAME);

            chat::init(ChatParams {
                tenant: tenant.to_string(),
                bot: bot_id.to_string(),
            });
        }
    }
}
