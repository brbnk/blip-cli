use serde::de::DeserializeOwned;
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};

pub struct HttpClient {
    client: Client,
    base_url: String,
    token: String,
    pub tenant: String,
    pub identifier: String
}

impl HttpClient {
    pub fn new(base_url: &str, token: &str, tenant: &str, identifier: &str) -> HttpClient {
        HttpClient { 
            client: Client::new(), 
            base_url: base_url.to_string(),
            token: token.to_string(),
            tenant: tenant.to_string(),
            identifier: identifier.to_string()
        }
    }

    pub fn get<T>(&self, path: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned 
    {
        let response = self.client
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