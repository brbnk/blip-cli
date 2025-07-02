use clap::Args;
use domain::traits::cli::Runnable;

use crate::types::CommonArgs;

#[derive(Args, Debug)]
pub struct Run {
    #[command(flatten)]
    commong_args: CommonArgs,

    /// test file name
    #[arg(short, long)]
    file_name: String,
}

impl Runnable for Run {
    fn run(&self) {
        todo!()
    }
}