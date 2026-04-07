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
                let mut context_store = ContextStore::new(&child.short_name);

                let bot_flow = get_bot_flow(&self.commong_args.tenant, &child.short_name);
                
                match bot_flow {
                    Ok(json) => {
                        for (_, state) in json.flow {
                            context_store.states += 1;
                            context_store.update(state);
                        }
                    },
                    Err(_) => {},
                }

                context_store.print_result();
            }
        }
        else {
            let bot_flow = get_bot_flow(&self.commong_args.tenant, &self.commong_args.bot);
            
            let mut context_store = ContextStore::new(&self.commong_args.bot);
            
            match bot_flow {
                Ok(json) => {
                    for (_, state) in json.flow {
                        context_store.states += 1;
                        context_store.update(state);
                    }
                }
                Err(_) => {},
            }
            
            context_store.print_result();
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

impl ContextStore {
    pub fn new(bot_id: &str) -> Self {
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

    pub fn print_result(&self) {
        print_state_title(&self.bot_id);
        println(&format!("States = {}", self.states), Color::White);
        println(&format!("ScriptsV1 = {}", self.scripts_v1), Color::White);
        println(&format!("ScriptsV2 = {}", self.scripts_v2), Color::White);
        println(&format!("ProcessHttp = {}", self.http), Color::White);
        println(&format!("ProcessCommands = {}", self.commands), Color::White);
        println(&format!("Trackings = {}", self.trackings), Color::White);
        println(&format!("Variables = {}", self.variables), Color::White);
        println(&format!("Redirects = {}", self.redirects), Color::White);
        println(&format!("BlipFunctions = {}", self.blip_function), Color::White);
        println(&format!("MergeContacts = {}", self.merge_contacts), Color::White);
        println(&format!("Agents = {}", self.agents), Color::White);
        println(&format!("Desk = {}", self.desk), Color::White);
        println!();
    }
    
    pub fn update(&mut self, state: State) {
        for eca in state.entering_custom_actions {
            self.increment(eca.action_type);
        }
    
        for lca in state.leaving_custom_actions {
            self.increment(lca.action_type);
        }
    }
    
    fn increment(&mut self, action_type: ActionType) {
        match action_type {
            ActionType::ExecuteScript => self.scripts_v1 += 1,
            ActionType::SetVariable => self.variables += 1,
            ActionType::ProcessHttp => self.http += 1,
            ActionType::MergeContact => self.merge_contacts += 1,
            ActionType::Redirect => self.redirects += 1,
            ActionType::ExecuteScriptV2 => self.scripts_v2 += 1,
            ActionType::ProcessCommand => self.commands += 1,
            ActionType::ExecuteBlipFunction => self.blip_function += 1,
            ActionType::ProcessContentAssistant => {},
            ActionType::TrackEvent => self.trackings += 1,
            ActionType::ForwardToAgent => self.agents += 1,
            ActionType::ForwardToDesk => self.desk += 1,
        }
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