use clap::Args;
use domain::traits::{cli::Runnable, file_handler::Writer};
use file_handler::types::TestTemplateFile;
use tester::types::TestTemplate;

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Create {
    #[command(flatten)]
    commong_args: CommonArgs,
}

impl Runnable for Create {
    fn run(&self) {
        if self.commong_args.is_valid() {
            let test_file_template = TestTemplateFile {
                tenant: self.commong_args.tenant.clone(),
                bot_id: self.commong_args.bot.clone(),
                content: Some(
                    serde_json::to_string_pretty(&TestTemplate::new()).expect("test template"),
                ),
            };

            test_file_template
                .write()
                .expect("create test file");
        }
    }
}
