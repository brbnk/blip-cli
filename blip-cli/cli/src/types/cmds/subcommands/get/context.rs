use clap::Args;
use domain::{cli::Getter, http::ContextRequests};
use http::ProxyRequests;

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
        let client = ProxyRequests::new();
        client.get_context(&self.bot, &self.contact, &self.variable);
    }
}
