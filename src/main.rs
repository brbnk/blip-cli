use contexts::{context};
use types::{flow::Flow};

fn main() {
  context::set_master_state("teste638760290597490566");
  
  let flow = Flow::deserialize(&context::get_master_state());

  let mut state = flow
    .get_onboarding_state()
    .expect("Bloco de início não encontrado!");
    
  loop {
    state.print_state_title();

    // Entering custom actions
    state.handle_custom_entering_actions();

    // Content actions
    state.handle_content_actions();

    // Leaving custom actions
    state.handle_custom_leaving_actions();

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
