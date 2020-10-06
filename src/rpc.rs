use anyhow::Result;
use jsonrpsee::{
    common::{to_value as to_json_value, Params},
    Client,
};
use sp_core::storage::StorageKey;

use crate::primitives::Hash;

#[derive(Clone)]
pub struct Rpc {
    client: Client,
}

impl Rpc {
    pub async fn new(url: &str) -> Result<Self> {
        let client = if url.starts_with("ws://") || url.starts_with("wss://") {
            jsonrpsee::ws_client(url).await?
        } else {
            jsonrpsee::http_client(url)
        };
        Ok(Self { client })
    }

    pub async fn get_keys(
        &self,
        storage_key: StorageKey,
        hash: Option<Hash>,
    ) -> Result<Vec<StorageKey>> {
        let params = Params::Array(vec![to_json_value(storage_key)?, to_json_value(hash)?]);
        let data = self.client.request("state_getKeys", params).await?;
        Ok(data)
    }

    pub async fn get_accounts(&self, hash: Option<Hash>) -> Result<Vec<String>> {
        let storage_key = StorageKey(hex::decode(
            "26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9",
        )?);
        let data = self.get_keys(storage_key, hash).await?;

        let keys = data
            .into_iter()
            .map(|x| hex::encode(&x.0))
            .collect::<Vec<_>>();

        // System Account + hash = 96 chars
        let accounts = keys
            .into_iter()
            .map(|x| format!("0x{}", &x[96..]))
            .collect::<Vec<_>>();
        println!("{:#?}", accounts);
        Ok(accounts)
    }
}
