use clap::Args;
use domain::{cli::Getter, http::ApplicationRequests};
use http::ProxyRequests;

// use http::ProxyHttpClient;

#[derive(Args, Debug)]
pub struct Key {
    /// bot identifier
    #[arg(short, long)]
    bot: String,
}

impl Getter for Key {
    fn get(&self) {
        let client = ProxyRequests::new();
        client.get_auth_key(&self.bot);
    }
}
