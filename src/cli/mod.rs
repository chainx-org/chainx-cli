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
    name = "xli",
    author = "koushiro <koushiro.cqx@gmail.com>",
    about = "A ChainX command-line tool"
)]
#[structopt(raw(setting = "AppSettings::DisableHelpSubcommand"))]
pub struct Command {
    #[structopt(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
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

impl SubCommand {
    pub fn dispatch<T>(self, transport: ChainXTransport<T>) -> Result<()>
    where
        T: BatchTransport + 'static,
    {
        match self {
            SubCommand::Completions { shell } => {
                SubCommand::clap().gen_completions_to("xli", shell, &mut io::stdout());
            }
            SubCommand::Rpc(rpc) => rpc.dispatch(transport)?,
        }
        Ok(())
    }
}
