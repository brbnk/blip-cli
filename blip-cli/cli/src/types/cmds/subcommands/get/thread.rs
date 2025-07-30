use clap::Args;
use domain::{cli::Getter, http::ThreadsRequests};
use http::ProxyRequests;

#[derive(Args, Debug)]
pub struct Thread {
    /// bot identifier
    #[arg(short, long)]
    bot: String,

    /// contact identity
    #[arg(short, long)]
    contact: String
}

impl Getter for Thread {
    fn get(&self) {
        let client = ProxyRequests::new();
        client.get_thread(&self.bot, &self.contact);
    }
}
