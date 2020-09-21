use sp_keyring::AccountKeyring;

use substrate_subxt::{
    sp_core::{crypto::Ss58Codec, sr25519},
    Client, ClientBuilder, PairSigner,
};

use chainx_runtime::AccountId;

use crate::{app::BuiltinAccounts, runtime::ChainXRuntime};

pub type Sr25519Signer = PairSigner<ChainXRuntime, sr25519::Pair>;

pub type ChainXClient = Client<ChainXRuntime>;

/// Parses AccountId from String, also supports passing the test accounts directly.
pub fn parse_account(address: &str) -> Result<AccountId, String> {
    match String::from(address).to_lowercase().as_str() {
        "alice" => Ok(AccountKeyring::Alice.to_account_id()),
        "bob" => Ok(AccountKeyring::Bob.to_account_id()),
        "charlie" => Ok(AccountKeyring::Charlie.to_account_id()),
        "dave" => Ok(AccountKeyring::Dave.to_account_id()),
        "eve" => Ok(AccountKeyring::Eve.to_account_id()),
        "ferdie" => Ok(AccountKeyring::Ferdie.to_account_id()),
        "one" => Ok(AccountKeyring::One.to_account_id()),
        "two" => Ok(AccountKeyring::Two.to_account_id()),
        _ => AccountId::from_string(address)
            .map_err(|err| format!("Failed to parse account address: {:?}", err)),
    }
}

/// Builds a ChainX runtime specific client.
pub async fn build_client(url: String) -> Result<ChainXClient, Box<dyn std::error::Error>> {
    Ok(ClientBuilder::<ChainXRuntime>::new()
        .set_url(&url)
        .build()
        .await?)
}

pub fn as_signer(account: BuiltinAccounts) -> Sr25519Signer {
    let account_keyring: AccountKeyring = account.into();
    PairSigner::new(account_keyring.pair())
}
