#[cfg(feature = "internal")]
mod root;
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
pub enum Command {
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

    /// Root subcommand
    #[cfg(feature = "internal")]
    #[structopt(name = "root")]
    #[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
    Root(root::RootCommand),
}

impl Command {
    pub fn dispatch(self, url: &str) -> Result<()> {
        use Command::*;
        match self {
            Completions { shell } => Self::gen_shell_completion(shell),
            Rpc(rpc) => rpc.dispatch(url)?,
            Root(root) => root.dispatch(url)?,
        }
        Ok(())
    }

    fn gen_shell_completion(shell: clap::Shell) {
        Self::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
    }
}
