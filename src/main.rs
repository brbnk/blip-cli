use contexts::context;
use types::flow::Flow;

fn main() {
    context::set_master_state("teste638760290597490566");

    let flow = Flow::deserialize(&context::get_master_state());

    let mut state = flow
        .get_onboarding_state()
        .expect("Bloco de início não encontrado!");

    let mut is_first_input = true;

    loop {
        state.print_state_title();
        state.handle_global_leaving_actions(is_first_input);
        state.handle_custom_entering_actions();
        state.handle_content_actions(is_first_input);
        state.handle_custom_leaving_actions();
        state.save_previous();

        state = flow.get_state(match state.handle_condition_outputs() {
            Some(destination) => destination,
            None => state.get_default_output(),
        }).expect("Bloco não encontrado");

        state.save_current();
        is_first_input = false;
        println!();
    }
}
