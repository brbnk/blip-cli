use flow_parser::parse;

fn main() {
  let flow = parse(String::from("./flow.json")).expect("Falha ao realizar o parse do fluxo!");

  let mut state = flow
    .get_onboarding_state()
    .expect("Bloco de início não encontrado!");
    
  loop {
    state.print_state_title();

    // Global entering actions

    // Entering custom actions

    // Content actions
    state.handle_content_actions();

    // Leaving custom actions

    // Global leaving actions

    // Condition outputs
    let destination = match state.handle_condition_outputs() {
      Some(destination) => destination,
      None => state.get_default_output(),
    };
    
    state = flow
      .get_state(destination)
      .expect("Bloco não encontrado");
    
    println!("");
  }
}
