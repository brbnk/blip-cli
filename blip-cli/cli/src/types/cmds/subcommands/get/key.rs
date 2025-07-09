use clap::Args;
use domain::{cli::Getter, constants, http::ProxyRequests};
use http::ProxyHttpClient;

#[derive(Args, Debug)]
pub struct Key {
     #[arg(short, long)]
    identifier: String,
}

impl Getter for Key {
    fn get(&self) {
        let proxy_client = ProxyHttpClient::new(
            &format!("{}/api/Proxy", constants::PROXY_SERVER_BASEURL),
            None, 
            Some(self.identifier.to_owned()),
            None);

        proxy_client.get_key();
    }
}
