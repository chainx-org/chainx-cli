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

use xpallet_mining_staking::{NominatorLedger, ValidatorLedger};

use crate::primitives::{AccountIndex, Balance, BlockNumber, Hash};
use crate::types::{AccountData, AccountInfo};
use crate::utils::{as_account_id_ed25519, Ed25519Public};

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

    pub async fn get_pairs(
        &self,
        storage_key: StorageKey,
        hash: Option<Hash>,
    ) -> Result<Vec<(StorageKey, StorageData)>> {
        let params = Params::Array(vec![to_json_value(storage_key)?, to_json_value(hash)?]);
        let data = self.client.request("state_getPairs", params).await?;
        Ok(data)
    }

    pub async fn get_accounts(&self, hash: Option<Hash>) -> Result<Vec<String>> {
        let prefix = storage_prefix_for("System", "Account");
        let storage_key = StorageKey(prefix);
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
        Ok(accounts)
    }

    pub async fn get_accounts_info(
        &self,
        hash: Option<Hash>,
    ) -> Result<
        Vec<(
            Ed25519Public,
            AccountInfo<AccountIndex, AccountData<Balance>>,
        )>,
    > {
        let prefix = storage_prefix_for("System", "Account");
        let storage_key = StorageKey(prefix);
        let data = self.get_pairs(storage_key, hash).await?;
        let ENDOWED = vec![
            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
            "be5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25f",
            "fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e",
        ];
        let ENDOWED = vec![];
        let result = data
            .into_iter()
            .filter_map(|(key, value)| {
                let decoded_account_info: Result<AccountInfo<AccountIndex, AccountData<Balance>>> =
                    Decode::decode(&mut value.0.as_slice())
                        .map_err(|e| anyhow!("failed to decode {:?}, error:{:?}", value, e));
                decoded_account_info.ok().and_then(|account_info| {
                    let pubkey_str = &hex::encode(&key.0)[96..];
                    if ENDOWED.contains(&pubkey_str) {
                        println!("----- Endowned account:{:#?}", pubkey_str);
                        None
                    } else {
                        // FIXME: Alice is sr25519 account
                        as_account_id_ed25519(pubkey_str)
                            .ok()
                            .map(|account_id| (account_id, account_info))
                    }
                })
            })
            .collect::<Vec<_>>();
        Ok(result)
    }

    pub async fn get_nominations(
        &self,
        hash: Option<Hash>,
    ) -> Result<
        BTreeMap<Ed25519Public, BTreeMap<Ed25519Public, NominatorLedger<Balance, BlockNumber>>>,
    > {
        let prefix = storage_prefix_for("XStaking", "Nominations");
        let hex_prefix_len = hex::encode(&prefix).len();
        let storage_key = StorageKey(prefix);
        let data = self.get_pairs(storage_key, hash).await?;
        let mut nominations: BTreeMap<
            Ed25519Public,
            BTreeMap<Ed25519Public, NominatorLedger<Balance, BlockNumber>>,
        > = BTreeMap::new();
        for (key, value) in data {
            let decoded_nominator_ledger: Result<NominatorLedger<Balance, BlockNumber>> =
                Decode::decode(&mut value.0.as_slice())
                    .map_err(|e| anyhow!("failed to decode {:?}, error:{:?}", value, e));
            let key = hex::encode(&key.0);
            let hashed_key1_key1_hashed_key2_key2 = &key[hex_prefix_len..];
            let hashed_key1_key1 =
                &hashed_key1_key1_hashed_key2_key2[..hashed_key1_key1_hashed_key2_key2.len() / 2];
            let hashed_key2_key2 =
                &hashed_key1_key1_hashed_key2_key2[hashed_key1_key1_hashed_key2_key2.len() / 2..];
            let key1 = &hashed_key1_key1[TWOX_HASH_LEN..];
            let key2 = &hashed_key2_key2[TWOX_HASH_LEN..];
            let nominator = as_account_id_ed25519(key1)?;
            let nominee = as_account_id_ed25519(key2)?;
            let votes = nominations.entry(nominator).or_default();
            let nominator_ledger = decoded_nominator_ledger.unwrap();
            votes.insert(nominee, nominator_ledger);
        }
        Ok(nominations)
    }

    pub async fn get_validator_ledgers(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<Ed25519Public, ValidatorLedger<Balance, BlockNumber>>> {
        let prefix = storage_prefix_for("XStaking", "ValidatorLedgers");
        let storage_key = StorageKey(prefix);
        let data = self.get_pairs(storage_key, hash).await?;
        let mut validator_ledgers = BTreeMap::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key_key = &key[64..];
            let key = &hashed_key_key[TWOX_HASH_LEN..];
            let validator = as_account_id_ed25519(key)?;

            let validator_ledger: ValidatorLedger<Balance, BlockNumber> =
                Decode::decode(&mut value.0.as_slice())
                    .map_err(|e| anyhow!("failed to decode {:?}, error:{:?}", value, e))?;
            validator_ledgers.insert(validator, validator_ledger);
        }
        Ok(validator_ledgers)
    }
}

const TWOX_HASH_LEN: usize = 16;

fn storage_prefix_for(module: &str, storage_name: &str) -> Vec<u8> {
    let mut storage_prefix = twox_128(module.as_bytes()).to_vec();
    storage_prefix.extend_from_slice(&twox_128(storage_name.as_bytes()));
    storage_prefix
}

// a2162bba46c28c6fa06a97bac4ad9efd15a12573b165647c961348db57def9a8
// a2162bba46c28c6fa06a97bac4ad9efd15a12573b165647c961348db57def9a8 fff1db419264c4d0d279e25916d31b790fb1eed84489c7c8750e7a4dbdc23162565c648e10614776c2aaca9b81677beef6337ca4247f4825277da0581c5bd83d0c61b7991360fd71ed00a65767c73ffa
