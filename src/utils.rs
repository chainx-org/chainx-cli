use anyhow::{anyhow, Result};
use sp_core::crypto::Ss58Codec;
use sp_keyring::AccountKeyring;
use subxt::ClientBuilder;

use crate::runtime::{primitives::AccountId, ChainXClient, ChainXRuntime};

/// Parses AccountId from String, also supports passing the test accounts directly.
pub fn parse_account(address: &str) -> Result<AccountId> {
    match String::from(address).to_lowercase().as_str() {
        "alice" => Ok(AccountKeyring::Alice.to_account_id()),
        "bob" => Ok(AccountKeyring::Bob.to_account_id()),
        "charlie" => Ok(AccountKeyring::Charlie.to_account_id()),
        "dave" => Ok(AccountKeyring::Dave.to_account_id()),
        "eve" => Ok(AccountKeyring::Eve.to_account_id()),
        "ferdie" => Ok(AccountKeyring::Ferdie.to_account_id()),
        "one" => Ok(AccountKeyring::One.to_account_id()),
        "two" => Ok(AccountKeyring::Two.to_account_id()),
        _ => Ok(AccountId::from_string(address)
            .map_err(|err| anyhow!("Failed to parse account address: {:?}", err))?),
    }
}

/// Builds a ChainX runtime specific client.
pub async fn build_client(url: String) -> Result<ChainXClient> {
    Ok(ClientBuilder::<ChainXRuntime>::new()
        .set_url(&url)
        .build()
        .await?)
}
