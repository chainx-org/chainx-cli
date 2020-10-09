pub mod balances;
pub mod session;
pub mod sudo;
pub mod system;
pub mod xstaking;

use anyhow::{anyhow, Result};
use sp_core::{crypto::Ss58AddressFormat, Pair};
use sp_keyring::AccountKeyring;
use structopt::{clap::arg_enum, StructOpt};
use subxt::PairSigner;

use crate::utils::Sr25519Signer;

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

    /// Verify the 2.0 genesis is correct against the state exported from 1.0.
    #[structopt(name = "verify")]
    Verify,
    #[structopt(name = "inspect-key")]
    InspectKey,
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
    /// Builtin test accounts.
    #[structopt(long, possible_values = &BuiltinAccounts::variants(), case_insensitive = true)]
    pub signer: Option<BuiltinAccounts>,

    /// A Key URI used as a signer.
    ///
    /// Maybe a secret seed, secret URI(with derivation paths and password), SS58 or public URI.
    /// You can also use an environment variable URI=[URI] for this purpose.
    #[structopt(long)]
    pub uri: Option<String>,

    #[structopt(long, default_value = "ws://127.0.0.1:9944")]
    pub url: String,

    #[structopt(long)]
    pub network: Option<Ss58AddressFormat>,

    #[structopt(subcommand)]
    pub command: Cmd,
}

fn as_sr25519_signer(uri: &str) -> Result<Sr25519Signer> {
    sp_core::sr25519::Pair::from_phrase(&uri, None)
        .map(|(pair, _seed)| PairSigner::new(pair))
        .map_err(|err| anyhow!("Failed to generate sr25519 Pair from uri: {:?}", err))
}

impl App {
    pub fn init() -> Self {
        App::from_args()
    }

    pub async fn run(self) -> Result<()> {
        let signer = if let Some(ref uri) = self.get_uri() {
            as_sr25519_signer(uri)?
        } else {
            self.builtin_signer()
        };
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
                let accounts = rpc.get_accounts(Some(*genesis_hash)).await?;
                println!("{:#?}", rpc.get_accounts_info(Some(*genesis_hash)).await?);
                let nominations = rpc.get_nominations(Some(*genesis_hash)).await?;
                println!(
                    "{:#?}",
                    rpc.get_validator_ledgers(Some(*genesis_hash)).await?
                );
            }
            Cmd::InspectKey => {
                if let Some(ref uri) = self.get_uri() {
                    sc_cli::utils::print_from_uri::<sp_core::sr25519::Pair>(
                        uri,
                        None,
                        self.network,
                        sc_cli::OutputType::Text,
                    );
                }
            }
        }
        Ok(())
    }

    fn get_uri(&self) -> Option<String> {
        if let Some(ref uri) = self.uri {
            Some(uri.into())
        } else if let Ok(ref uri) = std::env::var("URI") {
            Some(uri.into())
        } else {
            None
        }
    }

    fn builtin_signer(&self) -> Sr25519Signer {
        let signer = self
            .signer
            .clone()
            .unwrap_or_else(|| BuiltinAccounts::Alice);
        let signer: AccountKeyring = signer.into();
        PairSigner::new(signer.pair())
    }
}
