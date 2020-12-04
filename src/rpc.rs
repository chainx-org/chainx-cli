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
    xpallets::{
        xassets::AssetType,
        xmining_asset::{AssetLedger, MinerLedger, MiningWeight},
        xstaking::{NominatorLedger, Unbonded, ValidatorLedger, ValidatorProfile, VoteWeight},
    },
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

    #[allow(unused)]
    pub async fn get_accounts(&self, hash: Option<Hash>) -> Result<Vec<String>> {
        let prefix = storage_prefix_for("System", "Account");
        let data = self.get_keys(StorageKey(prefix), hash).await?;

        let keys = data
            .into_iter()
            .map(|x| hex::encode(x.0))
            .collect::<Vec<_>>();

        // System Account (32 bytes hex) + hash (16 bytes hex) = 48 bytes hex
        let accounts = keys
            .into_iter()
            .map(|x| format!("0x{}", &x[STORAGE_PREFIX_LEN + BLAKE_HASH_LEN..]))
            .collect::<Vec<_>>();
        Ok(accounts)
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

    pub async fn get_accounts_info(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, AccountInfo<ChainXRuntime>>> {
        let prefix = storage_prefix_for("System", "Account");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;

        let mut result = BTreeMap::new();
        for (key, value) in data {
            let pubkey = &hex::encode(&key.0)[STORAGE_PREFIX_LEN + BLAKE_HASH_LEN..];
            let account_id = pubkey
                .parse::<AccountId>()
                .map_err(|err| anyhow!("{}", err))?;

            let account_info: AccountInfo<ChainXRuntime> = Decode::decode(&mut value.0.as_slice())?;

            result.insert(account_id, account_info);
        }
        Ok(result)
    }

    pub async fn get_asset_balance(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, BTreeMap<AssetId, BTreeMap<AssetType, Balance>>>> {
        let prefix = storage_prefix_for("XAssets", "AssetBalance");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;
        let mut assets =
            BTreeMap::<AccountId, BTreeMap<AssetId, BTreeMap<AssetType, Balance>>>::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key1_key1_hashed_key2_key2 = &key[STORAGE_PREFIX_LEN..];

            let hashed_key1_key1 = &hashed_key1_key1_hashed_key2_key2[..BLAKE_HASH_LEN + 64];
            let key1 = &hashed_key1_key1[BLAKE_HASH_LEN..];
            let account = key1
                .parse::<AccountId>()
                .map_err(|err| anyhow!("{}", err))?;

            let hashed_key2_key2 = &hashed_key1_key1_hashed_key2_key2[hashed_key1_key1.len()..];
            let key2 = &hashed_key2_key2[TWOX_HASH_LEN..];
            let mut asset_id = [0u8; 4];
            asset_id.copy_from_slice(hex::decode(key2)?.as_slice());

            let asset_balance: BTreeMap<AssetType, Balance> =
                Decode::decode(&mut value.0.as_slice())?;

            let entry = assets.entry(account).or_default();
            entry.insert(AssetId::from_le_bytes(asset_id), asset_balance);
        }
        Ok(assets)
    }

    pub async fn get_total_asset_balance(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AssetId, BTreeMap<AssetType, Balance>>> {
        let prefix = storage_prefix_for("XAssets", "TotalAssetBalance");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;
        let mut total_asset_balance = BTreeMap::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key_key = &key[STORAGE_PREFIX_LEN..];
            let key = &hashed_key_key[TWOX_HASH_LEN..];
            let mut asset_id = [0u8; 4];
            asset_id.copy_from_slice(hex::decode(key)?.as_slice());

            let asset_balance: BTreeMap<AssetType, Balance> =
                Decode::decode(&mut value.0.as_slice())?;

            total_asset_balance.insert(AssetId::from_le_bytes(asset_id), asset_balance);
        }
        Ok(total_asset_balance)
    }

    pub async fn get_asset_ledgers(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AssetId, AssetLedger<MiningWeight, BlockNumber>>> {
        let prefix = storage_prefix_for("XMiningAsset", "AssetLedgers");
        let storage_key = StorageKey(prefix);
        let data = self.get_pairs(storage_key, hash).await?;

        let mut asset_ledgers = BTreeMap::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key_key = &key[STORAGE_PREFIX_LEN..];
            let key = &hashed_key_key[TWOX_HASH_LEN..];
            let mut asset_id = [0u8; 4];
            asset_id.copy_from_slice(hex::decode(key)?.as_slice());

            let asset_ledger: AssetLedger<MiningWeight, BlockNumber> =
                Decode::decode(&mut value.0.as_slice())?;

            asset_ledgers.insert(AssetId::from_le_bytes(asset_id), asset_ledger);
        }
        Ok(asset_ledgers)
    }

    pub async fn get_miner_ledgers(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, BTreeMap<AssetId, MinerLedger<MiningWeight, BlockNumber>>>>
    {
        let prefix = storage_prefix_for("XMiningAsset", "MinerLedgers");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;

        let mut miner_ledgers =
            BTreeMap::<AccountId, BTreeMap<AssetId, MinerLedger<MiningWeight, BlockNumber>>>::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key1_key1_hashed_key2_key2 = &key[STORAGE_PREFIX_LEN..];

            let hashed_key1_key1 = &hashed_key1_key1_hashed_key2_key2[..TWOX_HASH_LEN + 64];
            let key1 = &hashed_key1_key1[TWOX_HASH_LEN..];
            let account = key1
                .parse::<AccountId>()
                .map_err(|err| anyhow!("{}", err))?;

            let hashed_key2_key2 = &hashed_key1_key1_hashed_key2_key2[hashed_key1_key1.len()..];
            let key2 = &hashed_key2_key2[TWOX_HASH_LEN..];
            let mut asset_id = [0u8; 4];
            asset_id.copy_from_slice(hex::decode(key2)?.as_slice());

            let miner_ledger: MinerLedger<MiningWeight, BlockNumber> =
                Decode::decode(&mut value.0.as_slice())?;

            let entry = miner_ledgers.entry(account).or_default();
            entry.insert(AssetId::from_le_bytes(asset_id), miner_ledger);
        }
        Ok(miner_ledgers)
    }

    pub async fn get_vesting_account(&self, hash: Option<Hash>) -> Result<AccountId> {
        let prefix = storage_prefix_for("XStaking", "VestingAccount");
        let data = self.get_pairs(StorageKey(prefix.clone()), hash).await?;
        let mut vesting_account = Default::default();
        for (_key, value) in data {
            vesting_account = Decode::decode(&mut value.0.as_slice())?;
        }
        Ok(vesting_account)
    }

    pub async fn get_validators(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, ValidatorProfile<BlockNumber>>> {
        let prefix = storage_prefix_for("XStaking", "Validators");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;
        let mut validator_profiles = BTreeMap::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key_key = &key[STORAGE_PREFIX_LEN..];
            let key = &hashed_key_key[TWOX_HASH_LEN..];
            let validator = key.parse::<AccountId>().map_err(|err| anyhow!("{}", err))?;

            let validator_profile: ValidatorProfile<BlockNumber> =
                Decode::decode(&mut value.0.as_slice())?;

            validator_profiles.insert(validator, validator_profile);
        }
        Ok(validator_profiles)
    }

    pub async fn get_nominations(
        &self,
        hash: Option<Hash>,
    ) -> Result<
        BTreeMap<AccountId, BTreeMap<AccountId, NominatorLedger<Balance, VoteWeight, BlockNumber>>>,
    > {
        let prefix = storage_prefix_for("XStaking", "Nominations");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;
        let mut nominations = BTreeMap::<
            AccountId,
            BTreeMap<AccountId, NominatorLedger<Balance, VoteWeight, BlockNumber>>,
        >::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key1_key1_hashed_key2_key2 = &key[STORAGE_PREFIX_LEN..];

            let hashed_key1_key1 = &hashed_key1_key1_hashed_key2_key2[..TWOX_HASH_LEN + 64];
            let key1 = &hashed_key1_key1[TWOX_HASH_LEN..];
            let nominator = key1
                .parse::<AccountId>()
                .map_err(|err| anyhow!("{}", err))?;

            let hashed_key2_key2 = &hashed_key1_key1_hashed_key2_key2[hashed_key1_key1.len()..];
            let key2 = &hashed_key2_key2[TWOX_HASH_LEN..];
            let nominee = key2
                .parse::<AccountId>()
                .map_err(|err| anyhow!("{}", err))?;

            let nominator_ledger: NominatorLedger<Balance, VoteWeight, BlockNumber> =
                Decode::decode(&mut value.0.as_slice())?;

            let entry = nominations.entry(nominator).or_default();
            entry.insert(nominee, nominator_ledger);
        }
        Ok(nominations)
    }

    pub async fn get_validator_ledgers(
        &self,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, ValidatorLedger<Balance, VoteWeight, BlockNumber>>> {
        let prefix = storage_prefix_for("XStaking", "ValidatorLedgers");
        let data = self.get_pairs(StorageKey(prefix), hash).await?;
        let mut validator_ledgers = BTreeMap::new();
        for (key, value) in data {
            let key = hex::encode(&key.0);
            let hashed_key_key = &key[STORAGE_PREFIX_LEN..];
            let key = &hashed_key_key[TWOX_HASH_LEN..];
            let validator = key.parse::<AccountId>().map_err(|err| anyhow!("{}", err))?;

            let validator_ledger: ValidatorLedger<Balance, VoteWeight, BlockNumber> =
                Decode::decode(&mut value.0.as_slice())?;

            validator_ledgers.insert(validator, validator_ledger);
        }
        Ok(validator_ledgers)
    }

    pub async fn get_staking_dividend(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, Balance>> {
        let params = Params::Array(vec![to_json_value(who)?, to_json_value(hash)?]);
        let data: BTreeMap<AccountId, String> = self
            .client
            .request("xstaking_getDividendByAccount", params)
            .await?;
        Ok(data
            .into_iter()
            .map(|(v, d)| {
                (
                    v,
                    d.parse::<Balance>()
                        .expect("Parse Balance from string failed"),
                )
            })
            .collect())
    }

    pub async fn get_nominations_rpc(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> Result<BTreeMap<AccountId, NominatorLedger<Balance, Balance, BlockNumber>>> {
        let params = Params::Array(vec![to_json_value(who)?, to_json_value(hash)?]);
        let data: BTreeMap<AccountId, NominatorLedger<String, String, BlockNumber>> = self
            .client
            .request("xstaking_getNominationByAccount", params)
            .await?;
        Ok(data
            .into_iter()
            .map(|(who, ledger)| {
                (
                    who,
                    NominatorLedger::<Balance, Balance, BlockNumber> {
                        nomination: ledger
                            .nomination
                            .parse::<Balance>()
                            .expect("Parse Balance from string failed"),
                        last_vote_weight: ledger
                            .last_vote_weight
                            .parse::<Balance>()
                            .expect("Parse Balance from string failed"),
                        unbonded_chunks: ledger
                            .unbonded_chunks
                            .into_iter()
                            .map(|unlocking| Unbonded {
                                value: unlocking
                                    .value
                                    .parse::<Balance>()
                                    .expect("Parse Balance from string failed"),
                                locked_until: unlocking.locked_until,
                            })
                            .collect(),
                        last_vote_weight_update: ledger.last_vote_weight_update,
                    },
                )
            })
            .collect())
    }
}
