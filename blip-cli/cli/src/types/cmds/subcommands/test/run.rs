use std::collections::HashMap;

use crate::types::{CommonArgs};
use chat::{custom_actions::Settings, params::ChatParams};

use clap::Args;
use contexts::{MANAGER_POOL, store, system};
use domain::traits::cli::Runnable;
use file_handler::deserialize;
use serde_json::Value;
use tester::types::{TestTemplate, asserts::AssertType};
use ui::{
    printer::{self, b},
    types::Color,
};

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
                printer::println(&format!("\nTEST CASE #{} 🧪\n", i + 1), Color::White);
                self.execute_test(f);
                println!();
            });
    }
}

impl Run {
    fn execute_test(&self, test_file: &TestTemplate) {
        if let Ok(inputs) = serde_json::to_string(&test_file.inputs) {
            printer::println(
                &format!("{}\n{}\n", b("DESCRIÇÃO"), test_file.description),
                Color::White,
            );

            system::enter_test_mode();
            system::set_test_inputs(&inputs);
            system::reset_end_test_signal();

            self.handle_mocks(&test_file.mocks);
            self.show_inputs(&test_file.inputs);
            
            // Run chat in test mode and collect events
            chat::init(ChatParams {
                tenant: self.commong_args.tenant.to_string(),
                bot: self.commong_args.bot.to_string(),
            });

            self.handle_asserts(&test_file);

            store::reset();
        }
    }

    fn handle_mocks(&self, mocks: &HashMap<String, Value>) {
        printer::println(&format!("{}", b("MOCKS")), Color::White);
        mocks.iter().for_each(|(k, v)| {
            let value = serde_json::to_string(v).expect("serialized value");
            let v_parsed = value.trim_matches('"');
            MANAGER_POOL.context.set(&k, &v_parsed);
            println!("- {}: {}", &k, &v_parsed)
        });
    }

    fn show_inputs(&self, inputs: &Vec<String>) {
        printer::println(&format!("\n{}", b("INPUTS")), Color::White);
        inputs.iter().for_each(|i| println!("- {i}"));
    }

    fn collect_events(&self) -> Vec<Settings> {
        match MANAGER_POOL.event.get(&system::get_master_state()) {
            Some(events_str) => deserialize::<Vec<String>>(&events_str)
                .expect("events")
                .iter()
                .map(|s| deserialize::<Settings>(s).expect("action deserialized"))
                .collect::<Vec<Settings>>(),
            None => panic!("Test events were expected"),
        }
    }

    fn handle_asserts(&self, test_file: &TestTemplate) {
        let events = self.collect_events();
        printer::println(&format!("\n{}", b("ASSERTS")), Color::White);
        for assert in &test_file.asserts {
            match assert {
                AssertType::Tracking { inner } => inner.assert(&events, &test_file),
                AssertType::Variable { inner } => inner.assert(&events, &test_file),
                AssertType::Redirect { inner: _ } => {}
                AssertType::SendMessage { inner: _ } => {}
                AssertType::Script { inner } => inner.assert(&events, &test_file),
            }
        }
    }
}
