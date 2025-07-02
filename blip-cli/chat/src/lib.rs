mod types;

use contexts::context;
use crate::types::Flow;

pub fn init(tenant: &str, identifier: &str) {
    context::set_tenant(tenant);
    context::set_master_state(identifier);

    let flow = Flow::deserialize(&context::get_master_state());

    let mut state = flow
        .get_onboarding_state()
        .expect("Bloco de início não encontrado!");

    let mut is_first_input = true;

    loop {
        ui::printer::print_state_title(&state.title);
        state.handle_global_leaving_actions(is_first_input);
        state.handle_custom_entering_actions();
        state.handle_content_actions(is_first_input);
        state.handle_custom_leaving_actions();
        state.save_previous();
        
        let default_output = state.get_default_output();
        state = flow.get_state(match state.handle_condition_outputs() {
            Some(destination) => destination,
            None => &default_output,
        }).expect("Bloco não encontrado");

        state.save_current();
        is_first_input = false;
        println!();
    }
}