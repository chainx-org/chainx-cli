use super::*;

impl Rpc {
    pub async fn get_accounts(&self, hash: Option<Hash>) -> Result<Vec<String>> {
        let prefix = storage_prefix_for("System", "Account");
        let data = self.get_keys(StorageKey(prefix), hash).await?;

        // System Account (32 bytes hex) + hash (16 bytes hex) = 48 bytes hex
        let accounts = data
            .into_iter()
            .map(|x| hex::encode(x.0))
            .map(|x| format!("0x{}", &x[STORAGE_PREFIX_LEN + BLAKE_HASH_LEN..]))
            .collect::<Vec<_>>();
        Ok(accounts)
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
}
