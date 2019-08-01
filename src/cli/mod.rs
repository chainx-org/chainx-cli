mod rpc;

use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "chainx-cli",
    author = "koushiro <koushiro.cqx@gmail.com>",
    about = "A ChainX command-line tool"
)]
#[structopt(raw(setting = "AppSettings::DisableHelpSubcommand"))]
pub enum Command {
    /// Rpc subcommand
    #[structopt(name = "rpc")]
    #[structopt(raw(setting = "AppSettings::DisableHelpSubcommand"))]
    Rpc(rpc::RpcCommand),
}

/// Initialize command from the env args
pub fn init() -> Command {
    Command::from_args()
}
