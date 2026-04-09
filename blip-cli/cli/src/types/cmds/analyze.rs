use std::{collections::HashMap, fs, ops::AddAssign};

use chat::{custom_actions::{ActionType, CustomAction, Settings}, types::{Flow, State}};
use clap::{Args};
use domain::{cli::Runnable, constants, file_handler::Reader};
use file_handler::{RouterChild, deserialize, types::DataFile};
use ui::{printer::{colorize, print, print_state_title, println}, types::Color};

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
    fetch: bool,

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

                let subflows = get_bot_subflows(&self.commong_args.tenant, &child.short_name);

                if subflows.is_ok() {
                    for subflow in subflows.unwrap() {
                        for (id, json) in subflow {
                            let mut context_subflow_store = ContextStore::new(&id);
                            
                            for (_, state) in json.flow {
                                context_subflow_store.states += 1;
                                context_subflow_store.update(state);
                            }

                            context_subflow_store.print_result(&self);
                        }
                    }
                }
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

            let subflows = get_bot_subflows(&self.commong_args.tenant, &self.commong_args.bot);

            if subflows.is_ok() {
                for subflow in subflows.unwrap() {
                    for (id, json) in subflow {
                        let mut context_subflow_store = ContextStore::new(&id);
                        
                        for (_, state) in json.flow {
                            context_subflow_store.states += 1;
                            context_subflow_store.update(state);
                        }

                        context_subflow_store.print_result(&self);
                    }
                }
            }
        }
    }
}

struct Context {
    count: i32,
    props: Vec<Props>
}

struct Props {
    state_id: String,
    state_name: String,
    actions: Vec<CustomAction>
}

impl AddAssign<i32> for Context {
    fn add_assign(&mut self, rhs: i32) {
        self.count += rhs;
    }
}

impl Props {
    fn new(state_id: &str, state_name: &str) -> Self {
        Self { 
            state_id: String::from(state_id), 
            state_name: String::from(state_name), 
            actions: Vec::new() 
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Self { 
            count: 0, 
            props: Vec::new()
        }
    }

    pub fn print_result(&self, label: &str, should_display_actions: Option<bool>) {
        println(&format!("{} = {}", label, self.count), Color::White);

        if should_display_actions.unwrap_or(false) {
            for prop in &self.props {
                for ca in &prop.actions {
                    self.print_action(&ca.settings, &prop.state_id, &prop.state_name, ca.title.clone().unwrap_or(String::from("")));
                }
            }
        }
    }

    fn print_action(&self, settings: &Settings, state_id: &String, state_name: &String, action_name: String) {
        match settings {
            Settings::Script(s) => {
                if s.source.contains("fetchAsync") {
                    print("[HTTP] ", Color::Yellow);
                }

                 println(
                    &format!("{}: {} ({}) - {}\n{}: {}\n\n{}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize("Script", Color::Yellow), 
                        s.output_variable, 
                        s.source.replace("\\n", "\n")
                    ), 
                    Color::White);
            },
            Settings::Variable(v) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {}: {}\n    {}: {}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize("Variable", Color::Yellow),
                        v.variable, 
                        colorize("Value", Color::Yellow),
                        v.value.clone().unwrap_or(String::from("EMPTY"))
                    ), 
                    Color::White);
            },
            Settings::ProcessHttp(h) => {
                   println(
                    &format!("    {}: {} ({}) - {}\n    {} {}\n    {}: {}\n    {}: {}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize(&h.method, Color::Yellow), 
                        h.uri,
                        colorize("Status", Color::Yellow),
                        h.status.clone().unwrap_or(String::from("EMPTY")), 
                        colorize("Response", Color::Yellow),
                        h.response.clone().unwrap_or(String::from("EMPTY"))),
                    Color::White);
            },
            Settings::TrackEvent(t) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {}: {}\n    {}: {}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize("Category", Color::Yellow),
                        t.category, 
                        colorize("Action", Color::Yellow),
                        t.action), 
                    Color::White);
            },
            Settings::MergeContact(m) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {} {:#?}\n", 
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize("MergeContact", Color::Yellow), 
                        m), 
                    Color::White);
            },
            Settings::Redirect(r) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {}: {}\n    {}: {:#?}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name, 
                        colorize("Address", Color::Yellow),
                        r.address, 
                        colorize("Context", Color::Yellow),
                        r.context), 
                    Color::White);
            },
            Settings::ScriptV2(sv2) => {
                if sv2.source.contains("fetchAsync") {
                    print("[HTTP] ", Color::Yellow);
                }

                println(
                    &format!("{}: {} ({}) - {}\n{}: {}\n\n{}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize("Script", Color::Yellow), 
                        sv2.output_variable, 
                        sv2.source.replace("\\n", "\n")
                    ), 
                    Color::White);
            },
            Settings::ProcessCommand(p) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {} {}\n    {}: {}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        p.method,
                        p.uri,
                        colorize("Variable", Color::Yellow),
                        p.variable), 
                    Color::White);
            },
            Settings::ExecuteBlipFunction(ebf) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {}\n", 
                    colorize("State", Color::Yellow),
                    state_name,
                    state_id,
                    action_name,
                    ebf.output_variable), 
                        Color::White);
            },
            Settings::ProcessContentAssistant(pca) => {
                println(
                    &format!("    {}: {} ({}) - {}\n    {}: {}\n    {}: {}\n",
                        colorize("State", Color::Yellow),
                        state_name,
                        state_id,
                        action_name,
                        colorize("Variable", Color::Yellow),
                        pca.output_variable, 
                         colorize("Score", Color::Yellow),
                        pca.score), 
                    Color::White);
            },
            Settings::ForwardToDesk(ftd) => {
                println(&format!("    {}: {} ({}) - {}\n    {:#?}\n",
                    colorize("State", Color::Yellow),
                    state_name,
                    state_id,
                    action_name,
                    ftd), 
                Color::White);
            },
            Settings::Agent(a) => {
                println(&format!("    {}: {} ({}) - {}\n    {:#?}\n",
                    colorize("State", Color::Yellow),
                    state_name,
                    state_id,
                    action_name,
                    a.output), 
                Color::White);
            },
        }
    }
}

struct ContextStore {
    bot_id: String,
    states: Context,
    scripts_v1: Context,
    scripts_v2: Context,
    scripts_v2_http: Context,
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
            scripts_v2_http: Context::new(),
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
        self.scripts_v2_http.print_result("ScriptV2_Http", Some(analyze.fetch));
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
            self.increment(eca, &state.id, &state.title);
        }
    
        for lca in state.leaving_custom_actions {
            self.increment(lca, &state.id, &state.title);
        }
    }
    
    fn increment(&mut self, custom_action: CustomAction, state_id: &str, state_title: &str) {
        let mut props = Props::new(state_id, state_title);

        match custom_action.action_type {
            ActionType::ExecuteScript => {
                self.scripts_v1 += 1;
                
                match &custom_action.settings {
                    Settings::Script(s) => {
                        if s.source.contains("fetchAsync") {
                            self.scripts_v2_http += 1;
                            props.actions.push(custom_action);
                            self.scripts_v2_http.props.push(props);
                        }
                        else {
                            props.actions.push(custom_action);
                            self.scripts_v1.props.push(props);
                        }
                    },
                    Settings::ScriptV2(s2) => {
                        if s2.source.contains("fetchAsync") {
                            self.scripts_v2_http += 1;
                            props.actions.push(custom_action);
                            self.scripts_v2_http.props.push(props);
                        }
                        else {
                            props.actions.push(custom_action);
                            self.scripts_v1.props.push(props);
                        }
                    },
                    _ => {}
                }
            },
            ActionType::SetVariable => {
                self.variables += 1;
                props.actions.push(custom_action);
                self.variables.props.push(props);
            },
            ActionType::ProcessHttp => {
                self.http += 1;
                props.actions.push(custom_action);
                self.http.props.push(props);
            },
            ActionType::MergeContact => {
                self.merge_contacts += 1;
                props.actions.push(custom_action);
                self.merge_contacts.props.push(props);
            },
            ActionType::Redirect => {
                self.redirects += 1;
                props.actions.push(custom_action);
                self.redirects.props.push(props);
            },
            ActionType::ExecuteScriptV2 => {
                self.scripts_v2 += 1;

                match &custom_action.settings {
                    Settings::Script(s) => {
                        if s.source.contains("fetchAsync") {
                            self.scripts_v2_http += 1;
                            props.actions.push(custom_action);
                            self.scripts_v2_http.props.push(props);
                        }
                        else {
                            props.actions.push(custom_action);
                            self.scripts_v2.props.push(props);
                        }
                    },
                    Settings::ScriptV2(s2) => {
                        if s2.source.contains("fetchAsync") {
                            self.scripts_v2_http += 1;
                            props.actions.push(custom_action);
                            self.scripts_v2_http.props.push(props);
                        }
                        else {
                            props.actions.push(custom_action);
                            self.scripts_v2.props.push(props);
                        }
                    },
                    _ => {}
                }
            },
            ActionType::ProcessCommand => {
                self.commands += 1;
                props.actions.push(custom_action);
                self.commands.props.push(props);
            },
            ActionType::ExecuteBlipFunction => {
                self.blip_function += 1;
                props.actions.push(custom_action);
                self.blip_function.props.push(props);
            },
            ActionType::ProcessContentAssistant => {},
            ActionType::TrackEvent => {
                self.trackings += 1;
                props.actions.push(custom_action);
                self.trackings.props.push(props);
            },
            ActionType::ForwardToAgent => {
                self.agents += 1;
                props.actions.push(custom_action);
                self.agents.props.push(props);
            },
            ActionType::ForwardToDesk => {
                self.desk += 1;
                props.actions.push(custom_action);
                self.desk.props.push(props);
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

fn get_bot_subflows(tenant: &str, bot_id: &str) -> Result<Vec<HashMap<String, Flow>>, String> {
    let folder = file_handler::resolve_path(Some(&format!("{}/{}/{}", "data", &tenant, &bot_id)));

    let mut subflows: Vec<HashMap<String, Flow>> = Vec::new();
    
    for entry in fs::read_dir(folder).expect("") {
        let mut map: HashMap<String, Flow> = HashMap::new();
        let entry = entry.expect("");
        let metadata = entry.metadata().expect("");

        if metadata.is_file() {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if file_name.starts_with("subflow_") {
                let file = DataFile {
                    tenant: String::from(tenant),
                    bot_id: Some(bot_id.to_string()),
                    file_name: String::from(file_name.clone()),
                    content: None
                };

                let json = file.read();
                
                if json.is_ok() {
                    let flow = deserialize::<Flow>(&json.unwrap()).expect("flow");
                    map.insert(file_name.to_string(), flow);
                    subflows.push(map);
                }
            }
        }
    }

    Ok(subflows)
}