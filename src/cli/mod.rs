mod rpc;

use structopt::clap;
use structopt::StructOpt;

use crate::error::Result;

/// Initialize command from the env args
pub fn init() -> Command {
    Command::from_args()
}

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
#[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
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
        shell: clap::Shell,
    },

    /// Rpc subcommand.
    #[structopt(name = "rpc")]
    #[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
    Rpc(rpc::RpcCommand),
}

impl SubCommand {
    pub fn dispatch(self, rpc_url: &str) -> Result<()> {
        match self {
            SubCommand::Completions { shell } => {
                SubCommand::clap().gen_completions_to("xli", shell, &mut std::io::stdout());
            }
            SubCommand::Rpc(rpc) => rpc.dispatch(rpc_url)?,
        }
        Ok(())
    }
}
