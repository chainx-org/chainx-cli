pub mod balances;
pub mod session;
pub mod sudo;
pub mod system;
pub mod xassets;
pub mod xmining_asset;
pub mod xstaking;

use anyhow::{anyhow, Result};
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use structopt::{clap::arg_enum, StructOpt};
use subxt::PairSigner;

use crate::runtime::ChainXSigner;

#[derive(StructOpt, Debug)]
pub enum Cmd {
    Balances(balances::Balances),
    Session(session::Session),
    Sudo(sudo::Sudo),
    System(system::System),

    #[structopt(name = "xassets")]
    XAssets(xassets::XAssets),
    #[structopt(name = "xmining_asset")]
    XMiningAsset(xmining_asset::XMingAsset),
    #[structopt(name = "xstaking")]
    XStaking(xstaking::XStaking),

    #[cfg(feature = "sc-cli")]
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

impl From<BuiltinAccounts> for AccountKeyring {
    fn from(builtin_account: BuiltinAccounts) -> Self {
        match builtin_account {
            BuiltinAccounts::Alice => Self::Alice,
            BuiltinAccounts::Bob => Self::Bob,
            BuiltinAccounts::Charlie => Self::Charlie,
            BuiltinAccounts::Dave => Self::Dave,
            BuiltinAccounts::Eve => Self::Eve,
            BuiltinAccounts::Ferdie => Self::Ferdie,
            BuiltinAccounts::One => Self::One,
            BuiltinAccounts::Two => Self::Two,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "chainx-cli", author, about, no_version)]
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

    /// The websocket url of ChainX node.
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    pub url: String,

    /// Ss58 Address version of the network.
    ///
    /// 44 for ChainX mainnet, 42 for Substrate.
    #[structopt(long, default_value = "44")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,

    #[structopt(subcommand)]
    pub command: Cmd,
}

fn as_sr25519_signer(uri: &str) -> Result<ChainXSigner> {
    sp_core::sr25519::Pair::from_phrase(&uri, None)
        .map(|(pair, _seed)| PairSigner::new(pair))
        .map_err(|err| anyhow!("Failed to generate sr25519 Pair from uri: {:?}", err))
}

impl App {
    pub fn init() -> Self {
        App::from_args()
    }

    pub async fn run(self) -> Result<()> {
        sp_core::crypto::set_default_ss58_version(self.ss58_prefix);

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
            Cmd::XAssets(xassets) => xassets.run(self.url, signer).await?,
            Cmd::XMiningAsset(xmining_asset) => xmining_asset.run(self.url, signer).await?,
            Cmd::XStaking(xstaking) => xstaking.run(self.url, signer).await?,
            #[cfg(feature = "sc-cli")]
            Cmd::InspectKey => {
                if let Some(ref uri) = self.get_uri() {
                    sc_cli::utils::print_from_uri::<sp_core::sr25519::Pair>(
                        uri,
                        None,
                        Some(self.ss58_prefix),
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

    fn builtin_signer(&self) -> ChainXSigner {
        let signer = self.signer.clone().unwrap_or(BuiltinAccounts::Alice);
        let signer: AccountKeyring = signer.into();
        PairSigner::new(signer.pair())
    }
}
