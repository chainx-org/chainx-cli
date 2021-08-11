pub mod system;
pub mod xassets;
pub mod xmining_asset;
pub mod xstaking;

use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use codec::Decode;
use jsonrpsee::{
    common::{to_value as to_json_value, Params},
    Client,
};
use sp_core::{
    storage::{StorageData, StorageKey},
    twox_128,
};
use subxt::system::AccountInfo;

use crate::runtime::{
    primitives::{AccountId, AssetId, Balance, BlockNumber, Hash},
    ChainXRuntime,
};

const TWOX_HASH_LEN: usize = 16; // 8 bytes hex
const BLAKE_HASH_LEN: usize = 32; // 16 bytes hex
const STORAGE_PREFIX_LEN: usize = 64; // 32 bytes hex

fn storage_prefix_for(module: &str, storage_name: &str) -> Vec<u8> {
    let mut storage_prefix = twox_128(module.as_bytes()).to_vec();
    storage_prefix.extend_from_slice(&twox_128(storage_name.as_bytes()));
    storage_prefix
}

#[derive(Clone)]
pub struct Rpc {
    client: Client,
}

impl Rpc {
    pub async fn new<U: AsRef<str>>(url: U) -> Result<Self> {
        let client = jsonrpsee::ws_client(url.as_ref()).await?;
        Ok(Self { client })
    }

    pub async fn genesis_hash(&self) -> Result<Hash> {
        let params = Params::Array(vec![to_json_value(0)?]);
        let hash = self.client.request("chain_getBlockHash", params).await?;
        Ok(hash)
    }

    #[allow(unused)]
    pub async fn get_keys(&self, key: StorageKey, hash: Option<Hash>) -> Result<Vec<StorageKey>> {
        let params = Params::Array(vec![to_json_value(key)?, to_json_value(hash)?]);
        let data = self.client.request("state_getKeys", params).await?;
        Ok(data)
    }

    pub async fn get_pairs(
        &self,
        key: StorageKey,
        hash: Option<Hash>,
    ) -> Result<Vec<(StorageKey, StorageData)>> {
        let params = Params::Array(vec![to_json_value(key)?, to_json_value(hash)?]);
        let data = self.client.request("state_getPairs", params).await?;
        Ok(data)
    }
}
