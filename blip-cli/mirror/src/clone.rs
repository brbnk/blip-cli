use http::{ProxyRequests};
use ui::{loader, printer, types::Color};
use domain::http::MirrorRequests;

use crate::{RequestType};

pub fn clone(tenant: &str, identifier: &str, tier: &str, request_type: &Vec<RequestType>) {
    let client = ProxyRequests::new();
    
    let start_msg = &format!("\nIniciando download das configurações do bot '{}'\n", &identifier);
    printer::println(&start_msg, Color::Yellow);
    loader::start(1);
    
    if !request_type.is_empty() {
        for rt in request_type {
            match rt {
                RequestType::WorkingFlow => client.get_working_flow(tenant, identifier),
                RequestType::GlobalAction => client.get_global_actions(tenant, identifier),
                RequestType::Configurations => client.get_config_variables(tenant, identifier),
                RequestType::Resources => client.get_resources(tenant, identifier),
                RequestType::BlipFunction => client.get_blip_functions(tenant, identifier),
                RequestType::Router => client.get_router_children(tenant, identifier, tier)
            };
        };
    }
}