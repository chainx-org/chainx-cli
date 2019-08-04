mod rpc;

use std::io;

use structopt::clap::{AppSettings, Shell};
use structopt::StructOpt;
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
    /// Generates completion scripts for your shell.
    #[structopt(name = "completions")]
    Completions {
        /// The shell to generate the script for
        #[structopt(value_name = "SHELL")]
        shell: Shell,
    },

    /// Rpc subcommand.
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
            Command::Completions { shell } => {
                Command::clap().gen_completions_to("xli", shell, &mut io::stdout());
            }
            Command::Rpc(rpc) => rpc.dispatch(transport)?,
        }
        Ok(())
    }
}
