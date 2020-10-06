pub mod balances;
pub mod session;
pub mod sudo;
pub mod system;
pub mod xstaking;

use anyhow::Result;
use sp_keyring::AccountKeyring;
use structopt::{clap::arg_enum, StructOpt};
use substrate_subxt::PairSigner;

use crate::rpc::Rpc;

#[derive(StructOpt, Debug)]
pub enum Cmd {
    #[structopt(name = "balances")]
    Balances(balances::Balances),
    #[structopt(name = "session")]
    Session(session::Session),
    #[structopt(name = "sudo")]
    Sudo(sudo::Sudo),
    #[structopt(name = "system")]
    System(system::System),

    #[structopt(name = "xstaking")]
    XStaking(xstaking::XStaking),

    /// Verify the genesis is correct with respect to the 1.0 exported state.
    #[structopt(name = "verify")]
    Verify,
}

arg_enum! {
  #[derive(Clone, Debug)]
  pub enum BuiltinAccounts {
      Alice,
      Bob,
      Charlie,
      Dave,
      Eve,
      Ferdie,
      One,
      Two,
  }
}

impl Into<AccountKeyring> for BuiltinAccounts {
    fn into(self) -> AccountKeyring {
        match self {
            Self::Alice => AccountKeyring::Alice,
            Self::Bob => AccountKeyring::Bob,
            Self::Charlie => AccountKeyring::Charlie,
            Self::Dave => AccountKeyring::Dave,
            Self::Eve => AccountKeyring::Eve,
            Self::Ferdie => AccountKeyring::Ferdie,
            Self::One => AccountKeyring::One,
            Self::Two => AccountKeyring::Two,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "chainx-cli", no_version)]
pub struct App {
    #[structopt(long, possible_values = &BuiltinAccounts::variants(), case_insensitive = true)]
    pub signer: Option<BuiltinAccounts>,

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
        let signer = self
            .signer
            .clone()
            .unwrap_or_else(|| BuiltinAccounts::Alice);
        let signer: AccountKeyring = signer.into();
        let signer = PairSigner::new(signer.pair());
        match self.command {
            Cmd::Balances(balances) => balances.run(self.url, signer).await?,
            Cmd::Session(session) => session.run(self.url, signer).await?,
            Cmd::Sudo(sudo) => sudo.run(self.url, signer).await?,
            Cmd::System(system) => system.run(self.url, signer).await?,
            Cmd::XStaking(xstaking) => xstaking.run(self.url, signer).await?,
            Cmd::Verify => {
                let client = crate::utils::build_client(self.url.clone()).await?;
                let genesis_hash = client.genesis();
                println!("genesis hash:{:?}", genesis_hash);
                let rpc = Rpc::new(&self.url).await?;
                // System Account + hash = 96 chars
                let accounts = rpc.get_accounts(Some(*genesis_hash)).await?;
                println!("{:#?}", accounts);
            }
        }
        Ok(())
    }
}
