use http::{HttpClient};
use types::http::Application;

mod auth;

pub fn clone_application(identifier: &str) {
    if let Some(token) = auth::get_token() {
        let client = HttpClient::new("http://localhost:5107", &token);

        let endpoint = format!("/api/Proxy/data?identifier={}", identifier);
        
        let response: Application = client
            .get(&endpoint)
            .expect("/api/Proxy/data response");

        response.save_flow(identifier).expect("flow.json created");
        response.save_global_actions(identifier).expect("global_actions.json created");
        response.save_configurations(identifier).expect("configs.json created")
    }
}