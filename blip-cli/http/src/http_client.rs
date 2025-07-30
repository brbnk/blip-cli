use std::{error::Error};

use reqwest::{blocking::{Client, Response}, header::{HeaderMap, HeaderValue}};

use crate::types::ProxyResponse;

pub struct HttpClient {
    base_url: String,
    headers: HeaderMap
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        Self { 
            base_url: base_url.to_string() ,
            headers: HeaderMap::new()
        }
    }

    pub fn add_header<V: AsRef<str>>(&mut self, key: &'static str, value: V) {
        self.headers.insert(key, HeaderValue::from_str(value.as_ref()).unwrap());
    }

    pub fn send(&self, path: &str) -> Result<Response, Box<dyn Error>> {
        let client = Client::new();

        Ok(client
            .get(format!("{}{}", &self.base_url, path))
            .headers(self.headers.clone())
            .send()?)
    }

    pub fn get(&self, path: &str) -> Result<ProxyResponse, Box<dyn Error>>
    {
        let response = self
            .send(path)?
            .json::<ProxyResponse>()?;

        Ok(response)
    }

    pub fn get_raw(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let response = self.send(path)?.text()?;
        Ok(serde_json::to_string(&response)?)
    }
}