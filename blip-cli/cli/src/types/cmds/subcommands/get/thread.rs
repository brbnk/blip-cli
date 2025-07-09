use clap::Args;
use domain::{cli::Getter, constants, http::ProxyRequests};
use http::ProxyHttpClient;

#[derive(Args, Debug)]
pub struct Thread {
    #[arg(short, long)]
    bot: String,

    #[arg(short, long)]
    contact: String
}

impl Getter for Thread {
    fn get(&self) {
        let proxy_client = ProxyHttpClient::new(
            &format!("{}/api/Proxy", constants::PROXY_SERVER_BASEURL),
            None, 
            Some(self.bot.to_owned()),
            None);

        proxy_client.get_thread(&self.contact);
    }
}
