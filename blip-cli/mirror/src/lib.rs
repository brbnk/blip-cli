use http::{HttpClient};
use types::http::{BuilderConfigs, BuilderFlow, BuilderGlobalActions, Resources, BlipFunctionsResult};
use ui::{loader, printer::print_success_message};

mod auth;
pub mod scanner;

pub enum RequestType {
    WorkingFlow,
    GlobalAction,
    Configurations,
    Resources,
    BlipFunction
}

pub fn clone(tenant: &str, identifier: &str, request_type: &Vec<RequestType>) {
    if let Some(token) = auth::get_token() {
        let client = HttpClient::new("http://localhost:5107/api/Proxy", &token, tenant, identifier);
        
        println!("\nIniciando download das configurações do bot '{}'", &client.identifier);
        loader::start(1);
        
        if !request_type.is_empty() {
            for rt in request_type {
                match rt {
                    RequestType::WorkingFlow => request_builder_working_flow(&client),
                    RequestType::GlobalAction => request_builder_global_actions(&client),
                    RequestType::Configurations => request_builder_configurations(&client),
                    RequestType::Resources => request_resources(&client),
                    RequestType::BlipFunction => request_blip_functions(&client)
                };
            };
        }
    }
}

fn request_builder_working_flow(client: &HttpClient) {
    let builder_flow: BuilderFlow = client
        .get(&format!("/working-flow?identifier={}", &client.identifier))
        .expect("builder flow");

    builder_flow.save(&client.tenant, &client.identifier).expect("flow.json created");
    print_success_message("Fluxo");
}

fn request_builder_global_actions(client: &HttpClient) {
    let builder_global_actions: BuilderGlobalActions = client
        .get(&format!("/global-actions?identifier={}", &client.identifier))
        .expect("builder global actions");

    builder_global_actions.save(&client.tenant, &client.identifier).expect("global_actions.json created");
    print_success_message("Ações Globais");
}

fn request_builder_configurations(client: &HttpClient) {
    let builder_configs: BuilderConfigs = client
        .get(&format!("/configs?identifier={}", &client.identifier))
        .expect("builder configuration variables");

    builder_configs.save(&client.tenant, &client.identifier).expect("configs.json created");
    print_success_message("Variáveis de configuração");
}

fn request_resources(client: &HttpClient) {
    let builder_configs: Resources = client
        .get(&format!("/resources?identifier={}", &client.identifier))
        .expect("application resources");

    builder_configs.save(&client.tenant, &client.identifier).expect("resources.json created");
    print_success_message("Recursos");
}

fn request_blip_functions(client: &HttpClient) {
    let blip_functions: BlipFunctionsResult = client
        .get(&format!("/blip-functions?identifier={}", &client.identifier))
        .expect("tenant blip functions");

    blip_functions.save().expect("blip-functions.json created");
    print_success_message("Blip functions");
}