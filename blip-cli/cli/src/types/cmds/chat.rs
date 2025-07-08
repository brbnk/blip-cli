use std::collections::HashMap;

use chat::params::ChatParams;
use clap::Args;
use contexts::MANAGER_POOL;
use domain::{constants, traits::cli::Runnable, traits::file_handler::Reader};
use file_handler::{deserialize, types::DataFile};
use serde_json::Value;

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

            let file = DataFile {
                tenant: tenant.to_string(),
                bot_id: Some(bot_id.to_string()),
                file_name: constants::MOCKS_FILE_NAME.to_string(),
                content: None,
            };

            let file = &file.read();
            match file {
                Ok(content) => {
                    let values = deserialize::<HashMap<String, Value>>(content);
                    match values {
                        Ok(v) => {
                            v.iter().for_each(|(key, value)| {
                                MANAGER_POOL.mocks.set(
                                    key,
                                    &serde_json::to_string(value).expect("serialized object"),
                                )
                            });
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }

            chat::init(ChatParams {
                tenant: tenant.to_string(),
                bot: bot_id.to_string(),
            });
        }
    }
}
