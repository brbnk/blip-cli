use clap::Args;
use domain::{cli::Getter, constants, http::ProxyRequests};
use http::ProxyHttpClient;

#[derive(Args, Debug)]
pub struct Context {
    /// bot identifier
    #[arg(short, long)]
    bot: String,

    /// contact identity
    #[arg(short, long)]
    contact: String,

    /// variable
    #[arg(short, long)]
    variable: String,
}

impl Getter for Context {
    fn get(&self) {
        let proxy_client = ProxyHttpClient::new(
            &format!("{}/api/Proxy", constants::PROXY_SERVER_BASEURL),
            None, 
            Some(self.bot.to_owned()),
            None);

        proxy_client.get_context(&self.contact, &self.variable);
    }
}
