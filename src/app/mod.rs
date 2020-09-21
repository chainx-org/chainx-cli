pub mod balances;
pub mod sudo;
pub mod system;
pub mod xstaking;

use structopt::clap::arg_enum;
use structopt::StructOpt;

use sp_keyring::AccountKeyring;

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
    #[structopt(long = "signer", possible_values = &BuiltinAccounts::variants(), case_insensitive = true)]
    pub signer: Option<BuiltinAccounts>,

    #[structopt(long = "url", default_value = "ws://127.0.0.1:9944")]
    pub url: String,

    #[structopt(subcommand)]
    pub command: Cmd,
}

impl App {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let signer = crate::utils::as_signer(self.signer.unwrap_or(BuiltinAccounts::Alice));
        match self.command {
            Cmd::Balances(balances) => balances.run(self.url, signer).await?,
            Cmd::System(system) => system.run(self.url, signer).await?,
            Cmd::XStaking(xstaking) => xstaking.run(self.url, signer).await?,
            Cmd::Sudo(sudo) => sudo.run(self.url, signer).await?,
        }
        Ok(())
    }
}
