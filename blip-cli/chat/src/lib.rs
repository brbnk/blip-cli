mod types;

pub use types::{
    params, 
    custom_actions,
    actions,
    content_actions,
    execute_conditions
};

use contexts::{system};
use crate::types::{Flow, params::ChatParams};

pub fn init(params: ChatParams) {
    system::set_tenant(&params.tenant);
    system::set_master_state(&params.bot);

    let mut is_first_input = true;

    loop {
        let flow = Flow::deserialize(&system::get_master_state());
        
        let mut state = flow
            .get_onboarding_state()
            .expect("Bloco de início não encontrado!"); 
    
        loop {
            if !system::is_test_mode() {
                ui::printer::print_state_title(&state.title);
            }
            
            state.handle_global_leaving_actions(is_first_input);
            state.handle_custom_entering_actions();
            state.handle_content_actions(is_first_input);

            if system::is_reset_end_test_signal() {
                break;
            }

            state.handle_custom_leaving_actions();
            state.save_previous();
        
            let default_output = state.get_default_output();
            state = flow.get_state(match state.handle_condition_outputs() {
                Some(destination) => destination,
                None => &default_output,
            }).expect("Bloco não encontrado");
    
            state.save_current();
            is_first_input = false;

            if !system::is_test_mode() {
                println!();
            }
        }

        if system::is_reset_end_test_signal() {
            break;
        }

        // context::set_master_state(identifier);
    }
}