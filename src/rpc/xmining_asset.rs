use super::*;

use crate::runtime::xpallets::xmining_asset::{AssetLedger, MinerLedger, MiningWeight};

impl Rpc {
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
}
