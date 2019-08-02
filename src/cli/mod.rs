mod rpc;

use structopt::{clap::AppSettings, StructOpt};
use web3::BatchTransport;

use crate::error::Result;
use crate::transport::ChainXTransport;

/// Initialize command from the env args
pub fn init() -> Command {
    Command::from_args()
}

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

impl Command {
    pub fn dispatch<T>(self, transport: ChainXTransport<T>) -> Result<()>
    where
        T: BatchTransport + 'static,
    {
        match self {
            Command::Rpc(rpc) => rpc.dispatch(transport)?,
        }
        Ok(())
    }
}
