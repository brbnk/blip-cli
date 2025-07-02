use clap::Subcommand;
use crate::types::cmds::{Chat, Mirror, TestCommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// simulate a chat
    Chat(Chat),

    /// mirror an application locally
    Mirror(Mirror),

    /// list local bots
    List {},

    /// automate flow tests
    Test(TestCommand)
}