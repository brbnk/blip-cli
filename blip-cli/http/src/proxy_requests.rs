use std::{process::{exit}};

use domain::{
    constants, file_handler::Writer, http::{ApplicationRequests, ContextRequests, MirrorRequests, ThreadsRequests}
};

use file_handler::{deserialize, types::DataFile};
use serde_json::Value;
use ui::{printer::{print_success_message, println}, types::Color};

use crate::{auth, types::{BlipFunctionsResult, ProxyResponse}, HttpClient};

pub struct ProxyRequests {
    client: HttpClient,
}

impl ProxyRequests {
    pub fn new() -> Self {
        if let Some(token) = auth::get_token() {
            let mut client = HttpClient::new(&format!("{}", constants::PROXY_SERVER_BASEURL));

            client.add_header(constants::TOKEN_HEADER, &token);

            Self { client: client }
        }
        else {
            exit(0)
        }
    }

    pub fn request(&self, endpoint: &str) -> Result<ProxyResponse, String> {
        let response = self.client
            .get(&endpoint)
            .expect("proxy server response");

        if !response.sucesss {
            return Err(format!("Erro na requisição: {}", &endpoint));
        }

        return Ok(response);
    }
}

impl ApplicationRequests for ProxyRequests {
    fn get_auth_key(&self, identifier: &str) {
        let endpoint = format!("/v1/application/key?identifier={}", identifier);

        let response= self
            .request(&endpoint)
            .expect("auth key");

        let key = serde_json::to_string(&response.data).unwrap_or_default();

        println(&format!("\n{}\n", key.trim_matches('"')), Color::White);
    }
}

impl MirrorRequests for ProxyRequests {
    fn get_working_flow(&self, tenant: &str, identifier: &str) {
        let endpoint = format!("/v1/mirror/working-flow?identifier={}", identifier);

        let response = self
            .request(&endpoint)
            .expect("working flow");

        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: Some(identifier.to_string()),
            file_name: constants::FLOW_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.data).expect("working flow")) 
        };

        file.write().expect("write woring flow file");

        print_success_message("Working Flow");
    }
    
    fn get_resources(&self, tenant: &str, identifier: &str) {
        let endpoint = format!("/v1/resource/all?identifier={}", identifier);

        let response = self
            .request(&endpoint)
            .expect("resources");

        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: Some(identifier.to_string()),
            file_name: constants::RESOURCES_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.data).expect("resources")) 
        };

        file.write().expect("write resources file");

        print_success_message("Resources");
    }
    
    fn get_config_variables(&self, tenant: &str, identifier: &str) {
        let endpoint = format!("/v1/mirror/builder-configs?identifier={}", identifier);

        let response = self
            .request(&endpoint)
            .expect("builder configuration variables");

        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: Some(identifier.to_string()),
            file_name: constants::CONFIGS_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.data).expect("builder configuration variables")) 
        };

        file.write().expect("write builder configuration variables file");

        print_success_message("Builder Configs");
    }
    
    fn get_global_actions(&self, tenant: &str, identifier: &str) {
        let endpoint = format!("/v1/mirror/global-actions?identifier={}", identifier);

        let response = self
            .request(&endpoint)
            .expect("global actions");

        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: Some(identifier.to_string()),
            file_name: constants::GLOBAL_ACTIONS_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.data).expect("global actions")) 
        };

        file.write().expect("write create global actions file");

        print_success_message("Global Actions");
    }
    
    fn get_blip_functions(&self, tenant: &str, identifier: &str) {
        let endpoint = format!("/v1/mirror/blip-functions?identifier={}", identifier);

        let response = self
            .request(&endpoint)
            .expect("blip functions");

        let data = serde_json::to_string_pretty(&response.data).expect("blip functions");
        let parsed: BlipFunctionsResult = deserialize(&data).expect("deserialized response");

        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: None,
            file_name: constants::BLIP_FUNCTION_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&parsed.functions).expect("blip functions")) 
        };

        file.write().expect("write blip functions file");

        print_success_message("Blip Functions");
    }
    
    fn get_router_children(&self, tenant: &str, identifier: &str, tier: &str) {
        let endpoint = format!("/v1/mirror/router-children?identifier={}&tier={}", identifier, tier);

        let response = self
            .request(&endpoint)
            .expect("router children");

        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: Some(identifier.to_string()),
            file_name: constants::ROUTER_CHILDREN_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.data).expect("router children")) 
        };

        file.write().expect("write router children file");

        print_success_message("Router Children");
    }
}

impl ContextRequests for ProxyRequests {
    fn get_context(&self, identifier: &str, contact: &str, context: &str) {
        let endpoint = format!(
            "/v1/context?identifier={}&contact={}&context={}", 
            identifier,
            contact, 
            context);

        let response= self
            .request(&endpoint)
            .expect("context");
        
        let raw = response.data.as_str().unwrap_or("");
        let context: Result<Value, _> = serde_json::from_str(raw);
        
        match context {
            Ok(json) => {
                println(&format!("\n{}\n", serde_json::to_string_pretty(&json).unwrap()), Color::White);
            }
            Err(_) => {
                println(&format!("\n{}\n", raw), Color::White);
            }
        };
    }
}

impl ThreadsRequests for ProxyRequests {
    fn get_thread(&self, identifier: &str, contact: &str) {
        let endpoint = format!(
            "/v1/threads?identifier={}&contact={}", 
            identifier,
            contact);

        let response= self
            .request(&endpoint)
            .expect("thread");

        let raw = serde_json::to_string(&response.data).expect("json");
        
        match serde_json::from_str::<Value>(&raw) {
            Ok(json) => {
                println(&format!("\n{}\n", serde_json::to_string_pretty(&json).unwrap()), Color::White);
            }
            Err(_) => {
                println(&format!("\n{}\n", raw), Color::White);
            }
        };
    }
}