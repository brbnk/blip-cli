use chat::{custom_actions::ActionType, types::{Flow, State}};
use clap::{Args};
use domain::{cli::Runnable, constants, file_handler::Reader};
use file_handler::{RouterChild, deserialize, types::DataFile};
use ui::{printer::{print_state_title, println}, types::Color};

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Analyze {
    #[command(flatten)]
    commong_args: CommonArgs,

    #[arg(short, long)]
    router: bool,

    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    scripts: bool,

    #[arg(short, long)]
    http: bool,

    #[arg(short, long)]
    command: bool,

    #[arg(short, long)]
    variable: bool,

    #[arg(short, long)]
    tracking: bool
}

impl Runnable for Analyze {
    fn run(&self) {
        if self.router {
            let router_children = get_router_children(
                &self.commong_args.tenant, &self.commong_args.bot);
                
            for child in router_children {
                let mut context_store = create_context_store(&child.short_name);

                let bot_flow = get_bot_flow(&self.commong_args.tenant, &child.short_name);
                
                match bot_flow {
                    Ok(json) => {
                        for (_, state) in json.flow {
                            context_store.states += 1;
                            update_store(state, &mut context_store);
                        }
                    },
                    Err(_) => {},
                }

                print_result(context_store);
            }
        }
        else {
            let bot_flow = get_bot_flow(&self.commong_args.tenant, &self.commong_args.bot);
            
            let mut context_store = create_context_store(&self.commong_args.bot);
            
            match bot_flow {
                Ok(json) => {
                    for (_, state) in json.flow {
                        context_store.states += 1;
                        update_store(state, &mut context_store);
                    }
                }
                Err(_) => {},
            }
            
            print_result(context_store);
        }
    }
}

struct ContextStore {
    bot_id: String,
    states: i32,
    scripts_v1: i32,
    scripts_v2: i32,
    http: i32,
    commands: i32,
    trackings: i32,
    variables: i32,
    redirects: i32,
    blip_function: i32,
    merge_contacts: i32,
    agents: i32,
    desk: i32
}

fn create_context_store(bot_id: &str) -> ContextStore {
    ContextStore { 
        bot_id: String::from(bot_id),
        states: 0,
        scripts_v1: 0,
        scripts_v2: 0,
        http: 0,
        commands: 0,
        trackings: 0,
        variables: 0,
        redirects: 0,
        blip_function: 0,
        merge_contacts: 0,
        agents: 0,
        desk: 0,
    }
}

fn print_result(context: ContextStore) {
    print_state_title(&context.bot_id);
    println(&format!("States = {}", context.states), Color::White);
    println(&format!("ScriptsV1 = {}", context.scripts_v1), Color::White);
    println(&format!("ScriptsV2 = {}", context.scripts_v2), Color::White);
    println(&format!("ProcessHttp = {}", context.http), Color::White);
    println(&format!("ProcessCommands = {}", context.commands), Color::White);
    println(&format!("Trackings = {}", context.trackings), Color::White);
    println(&format!("Variables = {}", context.variables), Color::White);
    println(&format!("Redirects = {}", context.redirects), Color::White);
    println(&format!("BlipFunctions = {}", context.blip_function), Color::White);
    println(&format!("MergeContacts = {}", context.merge_contacts), Color::White);
    println(&format!("Agents = {}", context.agents), Color::White);
    println(&format!("Desk = {}", context.desk), Color::White);
    println!();
}

fn update_store(state: State, context_store: &mut ContextStore) {
    for eca in state.entering_custom_actions {
        increment_counter(eca.action_type, context_store);
    }

    for lca in state.leaving_custom_actions {
        increment_counter(lca.action_type, context_store);
    }
}

fn increment_counter(action_type: ActionType, context_store: &mut ContextStore) {
    match action_type {
        ActionType::ExecuteScript => context_store.scripts_v1 += 1,
        ActionType::SetVariable => context_store.variables += 1,
        ActionType::ProcessHttp => context_store.http += 1,
        ActionType::MergeContact => context_store.merge_contacts += 1,
        ActionType::Redirect => context_store.redirects += 1,
        ActionType::ExecuteScriptV2 => context_store.scripts_v2 += 1,
        ActionType::ProcessCommand => context_store.commands += 1,
        ActionType::ExecuteBlipFunction => context_store.blip_function += 1,
        ActionType::ProcessContentAssistant => {},
        ActionType::TrackEvent => context_store.trackings += 1,
        ActionType::ForwardToAgent => context_store.agents += 1,
        ActionType::ForwardToDesk => context_store.desk += 1,
    }
}

fn get_router_children(tenant: &str, bot_id: &str) -> Vec<RouterChild> {
    let file = DataFile {
        tenant: String::from(tenant),
        bot_id: Some(bot_id.to_string()),
        file_name: String::from(constants::ROUTER_CHILDREN_FILE_NAME),
        content: None
    };

    let json = file.read();

    match json {
        Ok(content) => deserialize::<Vec<RouterChild>>(&content).expect("deserialized router child"),
        Err(_) => panic!(),
    }
}

fn get_bot_flow(tenant: &str, bot_id: &str) -> Result<Flow, String> {
    let file = DataFile {
        tenant: String::from(tenant),
        bot_id: Some(bot_id.to_string()),
        file_name: String::from(constants::FLOW_FILE_NAME),
        content: None
    };

    let json = file.read();

    match json {
        Ok(content) => deserialize::<Flow>(&content),
        Err(err) => Err(err),
    }
}