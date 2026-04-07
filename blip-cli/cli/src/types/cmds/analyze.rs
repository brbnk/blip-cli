use std::ops::AddAssign;

use chat::{custom_actions::{ActionType, CustomAction, Settings}, types::{Flow, State}};
use clap::{Args};
use domain::{cli::Runnable, constants, file_handler::Reader};
use file_handler::{RouterChild, deserialize, types::DataFile};
use ui::{printer::{print_state_title, println}, types::Color};

use crate::types::{CommonArgs};

#[derive(Args, Debug)]
pub struct Analyze {
    #[command(flatten)]
    commong_args: CommonArgs,

    #[arg(short, long)]
    router: bool,

    #[arg(short, long)]
    scripts: bool,

    #[arg(short, long)]
    http: bool,

    #[arg(short, long)]
    command: bool,

    #[arg(short, long)]
    variable: bool,

    #[arg(short, long)]
    tracking: bool,
    
    #[arg(short, long)]
    redirect: bool,

    #[arg(short, long)]
    blip_function: bool,

    #[arg(short, long)]
    merge_contacts: bool,

    #[arg(short, long)]
    agents: bool,

    #[arg(short, long)]
    desk: bool
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

                context_store.print_result(&self);
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
            
            context_store.print_result(&self);
        }
    }
}

struct Context {
    count: i32,
    actions: Vec<CustomAction>
}

impl AddAssign<i32> for Context {
    fn add_assign(&mut self, rhs: i32) {
        self.count += rhs;
    }
}

impl Context {

}

impl Context {
    pub fn new() -> Self {
        Self { 
            count: 0, 
            actions: Vec::new() 
        }
    }

    pub fn print_result(&self, label: &str, should_display_actions: Option<bool>) {
        println(&format!("{} = {}", label, self.count), Color::White);

        if should_display_actions.unwrap_or(false) {
            for ca in &self.actions {
                self.print_action(&ca.settings);
            }
        }
    }

    fn print_action(&self, settings: &Settings) {
        match settings {
            Settings::Script(s) => {
                println(&format!("SCRIPT: {}\n\n{}", s.output_variable, s.source.replace("\\n", "\n")), Color::White);
            },
            Settings::Variable(v) => {
                println(&format!("    Variable: {}\n    Value: {}\n", v.variable, v.value.clone().unwrap_or(String::from("EMPTY"))), Color::White);
            },
            Settings::ProcessHttp(h) => {
                println(
                    &format!("    {} {}\n      Status: {}\n      Response: {}\n", 
                    h.method, 
                    h.uri, 
                    h.status.clone().unwrap_or(String::from("EMPTY")), 
                    h.response.clone().unwrap_or(String::from("EMPTY"))), Color::White);
            },
            Settings::TrackEvent(t) => {
                println(&format!("    Category: {}\n    Action: {}\n", t.category, t.action), Color::White);
            },
            Settings::MergeContact(m) => {
                println(&format!("    {:#?}\n", m), Color::White);
            },
            Settings::Redirect(r) => {
                println(&format!("    Address: {}\n    {:#?}", r.address, r.context), Color::White);
            },
            Settings::ScriptV2(sv2) => {
                println(&format!(" SCRIPT: {}\n\n{}", sv2.output_variable, sv2.source.replace("\\n", "\n")), Color::White);
            },
            Settings::ProcessCommand(p) => {
                println(&format!("    {} {}\n    Variable: {}\n", p.method, p.uri, p.variable), Color::White);
            },
            Settings::ExecuteBlipFunction(ebf) => {
                println(&format!("    {}\n", ebf.output_variable), Color::White);
            },
            Settings::ProcessContentAssistant(pca) => {
                println(&format!("    Variable: {}\n    Score: {}\n", pca.output_variable, pca.score), Color::White);
            },
            Settings::ForwardToDesk(ftd) => {
                println(&format!("    {:#?}\n", ftd), Color::White);
            },
            Settings::Agent(a) => {
                println(&format!("    {:#?}\n", a.output), Color::White);
            },
        }
    }
}

struct ContextStore {
    bot_id: String,
    states: Context,
    scripts_v1: Context,
    scripts_v2: Context,
    http: Context,
    commands: Context,
    trackings: Context,
    variables: Context,
    redirects: Context,
    blip_function: Context,
    merge_contacts: Context,
    agents: Context,
    desk: Context
}

impl ContextStore {
    pub fn new(bot_id: &str) -> Self {
        ContextStore { 
            bot_id: String::from(bot_id),
            states: Context::new(),
            scripts_v1: Context::new(),
            scripts_v2: Context::new(),
            http: Context::new(),
            commands: Context::new(),
            trackings: Context::new(),
            variables: Context::new(),
            redirects: Context::new(),
            blip_function: Context::new(),
            merge_contacts: Context::new(),
            agents: Context::new(),
            desk: Context::new(),
        }
    }

    pub fn print_result(&self, analyze: &Analyze) {
        print_state_title(&self.bot_id);
        self.states.print_result("States", None);
        self.scripts_v1.print_result("ScriptV1", Some(analyze.scripts));
        self.scripts_v2.print_result("ScriptV2", Some(analyze.scripts));
        self.http.print_result("ProcessHttp", Some(analyze.http));
        self.commands.print_result("ProcessCommand", Some(analyze.command));
        self.trackings.print_result("Trackings",Some(analyze.tracking));
        self.variables.print_result("Variables", Some(analyze.variable));
        self.redirects.print_result("Redirects", Some(analyze.redirect));
        self.blip_function.print_result("BlipFunctions", Some(analyze.blip_function));
        self.merge_contacts.print_result("MergeContacts", Some(analyze.merge_contacts));
        self.agents.print_result("Agents", Some(analyze.agents));
        self.desk.print_result("Desk", Some(analyze.desk));
        println!();
    }
    
    pub fn update(&mut self, state: State) {
        for eca in state.entering_custom_actions {
            self.increment(eca);
        }
    
        for lca in state.leaving_custom_actions {
            self.increment(lca);
        }
    }
    
    fn increment(&mut self, custom_action: CustomAction) {
        match custom_action.action_type {
            ActionType::ExecuteScript => {
                self.scripts_v1 += 1;
                self.scripts_v1.actions.push(custom_action);
            },
            ActionType::SetVariable => {
                self.variables += 1;
                self.variables.actions.push(custom_action);
            },
            ActionType::ProcessHttp => {
                self.http += 1;
                self.http.actions.push(custom_action);
            },
            ActionType::MergeContact => {
                self.merge_contacts += 1;
                self.merge_contacts.actions.push(custom_action);
            },
            ActionType::Redirect => {
                self.redirects += 1;
                self.redirects.actions.push(custom_action);
            },
            ActionType::ExecuteScriptV2 => {
                self.scripts_v2 += 1;
                self.scripts_v2.actions.push(custom_action);
            },
            ActionType::ProcessCommand => {
                self.commands += 1;
                self.commands.actions.push(custom_action);
            },
            ActionType::ExecuteBlipFunction => {
                self.blip_function += 1;
                self.blip_function.actions.push(custom_action);
            },
            ActionType::ProcessContentAssistant => {},
            ActionType::TrackEvent => {
                self.trackings += 1;
                self.trackings.actions.push(custom_action);
            },
            ActionType::ForwardToAgent => {
                self.agents += 1;
                self.agents.actions.push(custom_action);
            },
            ActionType::ForwardToDesk => {
                self.desk += 1;
                self.desk.actions.push(custom_action);
            }
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