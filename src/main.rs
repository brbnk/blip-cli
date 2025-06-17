use flow_parser::parse;

fn main() {
    let flow = parse(String::from("./flow.json")).expect("Falha ao realizar o parse do fluxo!");
    let start = flow.get_onboarding_state().expect("Bloco de início não encontrado!");
    println!("{:#?}", start);

    let fallback = flow.get_state(&start.default_output.state_id);
    println!("{:#?}", fallback);
}
