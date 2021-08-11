use super::*;

use crate::runtime::xpallets::xstaking::{
    NominatorLedger, Unbonded, ValidatorLedger, ValidatorProfile, VoteWeight,
};

impl Rpc {
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
