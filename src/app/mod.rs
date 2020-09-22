pub mod balances;
pub mod sudo;
pub mod system;
pub mod xstaking;

use anyhow::Result;
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use substrate_subxt::PairSigner;

#[derive(StructOpt, Debug)]
pub enum Cmd {
    #[structopt(name = "balances")]
    Balances(balances::Balances),
    #[structopt(name = "system")]
    System(system::System),
    #[structopt(name = "sudo")]
    Sudo(sudo::Sudo),

    #[structopt(name = "xstaking")]
    XStaking(xstaking::XStaking),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "chainx-cli", no_version)]
pub struct App {
    #[structopt(long)]
    pub signer: Option<AccountKeyring>,

    #[structopt(long, default_value = "ws://127.0.0.1:9944")]
    pub url: String,

    #[structopt(subcommand)]
    pub command: Cmd,
}

impl App {
    pub fn init() -> Self {
        App::from_args()
    }

    pub async fn run(self) -> Result<()> {
        let account = self.signer.unwrap_or_else(|| AccountKeyring::Alice);
        let signer = PairSigner::new(account.pair());
        match self.command {
            Cmd::Balances(balances) => balances.run(self.url, signer).await?,
            Cmd::System(system) => system.run(self.url, signer).await?,
            Cmd::XStaking(xstaking) => xstaking.run(self.url, signer).await?,
            Cmd::Sudo(sudo) => sudo.run(self.url, signer).await?,
        }
        Ok(())
    }
}
