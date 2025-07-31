mod types;

use file_handler::deserialize;
pub use types::{
    params, 
    custom_actions,
    actions,
    content_actions,
    execute_conditions
};

use contexts::system;
use crate::types::{actions::Redirect, params::ChatParams, Flow, Router};

pub fn init(params: ChatParams) {
    let mut is_first_input = true;

    let mut master_state= params.bot.clone();
    if params.router {
        let router = Router::new(&params.tenant, &params.bot);
        master_state = match router.get_default_child() {
            Some(default) => default.short_name,
            None => panic!("default child on router was not found"),
        };
    }
    
    system::set_tenant(&params.tenant);
    system::set_master_state(&master_state);

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

            if system::is_reset_end_test_signal() || system::is_redirect_signal() {
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

        if params.router {
            let router = Router::new(&params.tenant, &params.bot);
            let serialized = system::get_redirect().expect("serialized redirect event");
            let redirect: Redirect = deserialize(&serialized).expect("deserialized redirect event");
            
            let destination = router.get_child_by_service(&redirect.address);
            match destination {
                Some(child) => system::set_master_state(&child.short_name),
                None => panic!("router service destination not found"),
            }

            system::clear_redirect();
            
            let context = match &redirect.context {
                Some(c) => c.value.clone(),
                None => "no_context".to_owned(),
            };

            system::set_redirect_transition_signal(&context);
        }
    }
}