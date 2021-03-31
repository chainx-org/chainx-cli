use std::{fs::File, io::Read, path::Path};

use anyhow::{anyhow, Result};
use sp_core::crypto::{Pair, Public, Ss58Codec};
use sp_keyring::AccountKeyring;
use sp_runtime::traits::{IdentifyAccount, Verify};
use subxt::ClientBuilder;

use crate::runtime::{
    primitives::{AccountId, BlockNumber, Hash, Signature},
    ChainXClient, ChainXRuntime,
};

pub fn read_code<P: AsRef<Path>>(code_path: P) -> Result<Vec<u8>> {
    let mut file = File::open(code_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

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

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Builds a ChainX runtime specific client.
pub async fn build_client<U: Into<String>>(url: U) -> Result<ChainXClient> {
    Ok(ClientBuilder::<ChainXRuntime>::new()
        .set_url(url)
        .skip_type_sizes_check()
        .build()
        .await?)
}

pub async fn block_hash(
    client: &ChainXClient,
    block_number: Option<BlockNumber>,
) -> Result<Option<Hash>> {
    if let Some(number) = block_number {
        Ok(client.block_hash(Some(number.into())).await?)
    } else {
        Ok(None)
    }
}
