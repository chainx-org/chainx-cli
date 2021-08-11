use super::*;

use crate::runtime::xpallets::xassets::AssetType;

impl Rpc {
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
}
