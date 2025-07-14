use clap::Args;

#[derive(Args, Debug)]
pub struct CommonArgs {
    /// contract
    #[arg(long)]
    pub tenant: String,

    /// bot identifier
    #[arg(long)]
    pub bot: String,
}

impl CommonArgs {
    pub fn is_valid(&self) -> bool {
        !self.tenant.is_empty() && !self.bot.is_empty()
    }
}