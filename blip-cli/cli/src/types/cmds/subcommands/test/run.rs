use crate::types::CommonArgs;
use chat::{
    custom_actions::Settings,
    params::ChatParams,
};

use clap::Args;
use contexts::{MANAGER_POOL, store, system};
use domain::traits::cli::Runnable;
use file_handler::deserialize;
use tester::types::{TestTemplate, asserts::AssertType};
use ui::{printer::{self, b}, types::Color};

#[derive(Args, Debug)]
pub struct Run {
    #[command(flatten)]
    commong_args: CommonArgs,
}

impl Runnable for Run {
    fn run(&self) {
        TestTemplate::read_files(&self.commong_args.tenant, &self.commong_args.bot)
            .iter()
            .enumerate()
            .for_each(|(i, f)| {
                printer::println(&format!("\n{}", "=".repeat(80)), Color::BrightBlack);
                printer::println(&format!("\nTEST CASE #{} 🧪\n", i+1), Color::White);
                self.execute_test(f);
                println!();
    }       );
    }
}

impl Run {
    fn execute_test(&self, test_file: &TestTemplate) {
        if let Ok(inputs) = serde_json::to_string(&test_file.inputs) {
            printer::println(&format!("{}\n{}\n", b("DESCRIÇÃO"), test_file.description), Color::White);
            system::enter_test_mode();
            system::set_test_inputs(&inputs);
            system::reset_end_test_signal();

            printer::println(&format!("{}", b("INPUTS")), Color::White);
            test_file.inputs.iter().for_each(|i| println!("- {i}"));

            chat::init(ChatParams {
                tenant: self.commong_args.tenant.to_string(),
                bot: self.commong_args.bot.to_string(),
            });

            let events = match MANAGER_POOL.event.get(&system::get_master_state()) {
                Some(events_str) => deserialize::<Vec<String>>(&events_str)
                    .expect("events")
                    .iter()
                    .map(|s| deserialize::<Settings>(s).expect("action deserialized"))
                    .collect::<Vec<Settings>>(),
                None => panic!("Test events were expected"),
            };

            printer::println(&format!("\n{}", b("ASSERTS")), Color::White);
            for assert in &test_file.asserts {
                match assert {
                    AssertType::Tracking { inner } => inner.assert(&events, Some(&test_file.specs)),
                    AssertType::Variable { inner } => inner.assert(&events, Some(&test_file.specs)),
                    AssertType::Redirect { inner: _ } => {},
                    AssertType::SendMessage { inner: _ } => {},
                    AssertType::Script { inner } => inner.assert(&events, Some(&test_file.specs)),
                }
            }

            store::reset();
            MANAGER_POOL.event.reset();
        }
    }
}
