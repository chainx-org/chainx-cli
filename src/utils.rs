use anyhow::{anyhow, Result};
use sp_keyring::AccountKeyring;
use substrate_subxt::{
    sp_core::{crypto::Ss58Codec, sr25519},
    Client, ClientBuilder, PairSigner,
};

use crate::{primitives::AccountId, runtime::ChainXRuntime};

pub use sp_core::ed25519::Public as Ed25519Public;

pub type Sr25519Signer = PairSigner<ChainXRuntime, sr25519::Pair>;

pub type ChainXClient = Client<ChainXRuntime>;

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

pub fn to_u8_32(pubkey_str: &str) -> Result<[u8; 32]> {
    use hex::FromHex;
    let trimed = if pubkey_str.starts_with("0x") {
        &pubkey_str[2..]
    } else {
        pubkey_str
    };

    let raw: [u8; 32] = if let Ok(raw) = <[u8; 32] as FromHex>::from_hex(trimed) {
        raw
    } else {
        return Err(anyhow!("Failed to hex [u8, 32]"));
    };

    Ok(raw)
}

pub fn as_account_id_ed25519(pubkey_str: &str) -> Result<Ed25519Public> {
    Ok(Ed25519Public::from_raw(to_u8_32(pubkey_str)?))
}

#[test]
fn test_account() {
    use sp_core::ed25519::Public;

    let account = Public::from_raw(
        to_u8_32("0x33bcfcfbed81b3f8bc2a6041cd62f969cd83cdeb7118a05d06fb5e3d2c27dfff").unwrap(),
    );
    println!("{:?}", account);
}
