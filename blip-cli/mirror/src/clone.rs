use file_handler::deserialize;
use http::{ProxyRequests};
use serde::{Deserialize, Serialize};
use ui::{loader, printer, types::Color};
use domain::http::MirrorRequests;

use crate::{RequestType};

pub fn clone(tenant: &str, identifier: &str, tier: &str, request_type: &Vec<RequestType>) {
    let client = ProxyRequests::new();
    
    print_start_msg(identifier);
    
    if !request_type.is_empty() {
        if request_type.contains(&RequestType::Router) {
            client.get_resources(tenant, identifier);

            let json = client.get_router_children(tenant, identifier, tier);
            let children = deserialize::<Vec<RouterChild>>(&json).expect("deserialized router children");

            let mut already_downloaded_blip_functions = false;
            for child in children {
                print_start_msg(&child.short_name);
                client.get_working_flow(tenant, &child.short_name);
                client.get_global_actions(tenant, &child.short_name);
                client.get_config_variables(tenant, &child.short_name);
                client.get_resources(tenant, &child.short_name);
                if !already_downloaded_blip_functions {
                    client.get_blip_functions(tenant, &child.short_name);
                    already_downloaded_blip_functions = true;
                }
            }
        }
        else {
            for rt in request_type {
                match rt {
                    RequestType::WorkingFlow => client.get_working_flow(tenant, identifier),
                    RequestType::GlobalAction => client.get_global_actions(tenant, identifier),
                    RequestType::Configurations => client.get_config_variables(tenant, identifier),
                    RequestType::Resources => client.get_resources(tenant, identifier),
                    RequestType::BlipFunction => client.get_blip_functions(tenant, identifier),
                    _ => {}
                };
            };
        }
    }
}

fn print_start_msg(identifier: &str) {
    let start_msg = &format!("\nIniciando download das configurações do bot '{}'\n", &identifier);
    printer::println(&start_msg, Color::Yellow);
    loader::start(1);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouterChild {
    #[serde(rename = "shortName")]
    pub short_name: String
}