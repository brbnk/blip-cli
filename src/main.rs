use flow_parser::parse;

fn main() {
    // let mut user_context: HashMap<String, String> = HashMap::new();
    let flow = parse(String::from("./flow.json")).expect("Falha ao realizar o parse do fluxo!");
    let mut state = flow.get_onboarding_state().expect("Bloco de início não encontrado!");
    
    loop {
        // Global entering actions

        // Entering custom actions

        // Content actions
        state.handle_content_actions();

        // Leaving custom actions
        state = flow
            .get_state(&state.condition_outputs[0].destination)
            .expect("Bloco não encontrado");
        
        // Default output

        // Global leaving actions
    }
}
