// mod call;
mod rpc;
// mod storage;

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
pub enum Command {
    /// Generates completion scripts for your shell.
    Completions {
        /// The shell to generate the script for
        #[structopt(value_name = "SHELL")]
        shell: clap::Shell,
    },
    /// Rpc subcommand.
    #[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
    Rpc(rpc::RpcCommand),
    /*
    /// Storage subcommand.
    #[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
    Storage(storage::StorageCommand),
    /// Call subcommand.
    #[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
    Call(call::CallCommand),
    */
}

impl Command {
    pub async fn dispatch(self, url: &str) -> Result<()> {
        use Command::*;
        match self {
            Completions { shell } => Self::gen_shell_completion(shell),
            Rpc(rpc) => rpc.dispatch(url).await?,
            /* Storage(storage) => storage.dispatch(url)?,
            Call(call) => call.dispatch(url)?,*/
        }
        Ok(())
    }

    fn gen_shell_completion(shell: clap::Shell) {
        Self::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
    }
}
