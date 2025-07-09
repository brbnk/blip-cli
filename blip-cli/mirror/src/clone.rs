use domain::{constants, http::ProxyRequests};
use http::{ProxyHttpClient};
use ui::{loader, printer, types::Color};

use crate::{auth, RequestType};

pub fn clone(tenant: &str, identifier: &str, tier: &str, request_type: &Vec<RequestType>) {
    if let Some(token) = auth::get_token() {
        let proxy_client = ProxyHttpClient::new(
            &format!("{}/api/Proxy", constants::PROXY_SERVER_BASEURL), 
            &token, 
            tenant, 
            identifier,
            tier);
        
        let start_msg = &format!("\nIniciando download das configurações do bot '{}'\n", &proxy_client.identifier);
        printer::println(&start_msg, Color::Yellow);
        loader::start(1);
        
        if !request_type.is_empty() {
            for rt in request_type {
                match rt {
                    RequestType::WorkingFlow => proxy_client.get_builder_flow(),
                    RequestType::GlobalAction => proxy_client.get_builder_global_actions(),
                    RequestType::Configurations => proxy_client.get_builder_configurations(),
                    RequestType::Resources => proxy_client.get_builder_resources(),
                    RequestType::BlipFunction => proxy_client.get_blip_functions(),
                    RequestType::Router => proxy_client.get_router_chidlren()
                };
            };
        }
    }
}