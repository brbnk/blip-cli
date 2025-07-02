use domain::{
    constants,
    traits::{file_handler::Writer, http::ProxyRequests},
};
use file_handler::types::DataFile;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::de::DeserializeOwned;
use ui::printer::print_success_message;

use crate::types::{
    BlipFunctionsResult, BuilderConfigs, BuilderFlow, BuilderGlobalActions, Resources,
};

pub struct ProxyHttpClient {
    client: Client,
    base_url: String,
    token: String,
    pub tenant: String,
    pub identifier: String,
}

impl ProxyHttpClient {
    pub fn new(base_url: &str, token: &str, tenant: &str, identifier: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
            tenant: tenant.to_string(),
            identifier: identifier.to_string(),
        }
    }

    pub fn get<T>(&self, path: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(format!("{}{}", &self.base_url, path))
            .headers(self.get_headers()?)
            .send()?
            .json::<T>()?;

        Ok(response)
    }

    fn get_headers(&self) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut header_map = HeaderMap::new();

        header_map.insert("token", HeaderValue::from_str(&self.token)?);

        Ok(header_map)
    }
}

impl ProxyRequests for ProxyHttpClient {
    fn get_builder_flow(&self) {
        let response: BuilderFlow = self
            .get(&format!("/working-flow?identifier={}", &self.identifier))
            .expect("");

        let data_file = DataFile {
            tenant: self.tenant.clone(),
            bot_id: Some(self.identifier.clone()),
            file_name: constants::FLOW_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.application).expect("flow json")),
        };

        data_file.write().expect("create flow file");
        print_success_message("Fluxo");
    }

    fn get_builder_resources(&self) {
        let response: Resources = self
            .get(&format!("/resources?identifier={}", &self.identifier))
            .expect("application resources");

        let data_file = DataFile {
            tenant: self.tenant.clone(),
            bot_id: Some(self.identifier.clone()),
            file_name: constants::RESOURCES_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.resources).expect("resources json")),
        };

        data_file.write().expect("resources file");
        print_success_message("Recursos");
    }

    fn get_builder_configurations(&self) {
        let response: BuilderConfigs = self
            .get(&format!("/configs?identifier={}", &self.identifier))
            .expect("builder configuration variables");

        let data_file = DataFile {
            tenant: self.tenant.clone(),
            bot_id: Some(self.identifier.clone()),
            file_name: constants::CONFIGS_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.configurations).expect("configurations json")),
        };

        data_file.write().expect("configurations file");
        print_success_message("Variáveis de configuração");
    }

    fn get_builder_global_actions(&self) {
        let response: BuilderGlobalActions = self
            .get(&format!("/global-actions?identifier={}", &self.identifier))
            .expect("builder global actions");

        let data_file = DataFile {
            tenant: self.tenant.clone(),
            bot_id: Some(self.identifier.clone()),
            file_name: constants::GLOBAL_ACTIONS_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.global_actions).expect("global action json")),
        };

        data_file.write().expect("create global actions file");
        print_success_message("Ações Globais");
    }

    fn get_blip_functions(&self) {
        let response: BlipFunctionsResult = self
            .get(&format!("/blip-functions?identifier={}", &self.identifier))
            .expect("tenant blip functions");

        let data_file = DataFile {
            tenant: self.tenant.clone(),
            bot_id: None,
            file_name: constants::BLIP_FUNCTION_FILE_NAME.to_string(),
            content: Some(serde_json::to_string_pretty(&response.functions).expect("blip function json")),
        };

        data_file.write().expect("blip functions file");
        print_success_message("Blip functions");
    }
}
