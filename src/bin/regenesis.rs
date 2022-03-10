//! Exports the neccessary state for the regenesis purpose.
//!
//! Used for the ChainX v4.0.0 regenesis.

use anyhow::Result;
use codec::Decode;
use serde::{Deserialize, Serialize};
use sp_core::storage::StorageKey;
use std::collections::BTreeMap;
use std::path::Path;
use structopt::StructOpt;
use subxt::{BlockNumber, ClientBuilder, Config};

use crate::chainx_v4::{
    runtime_types::xpallet_mining_staking::types::NominatorLedger, DefaultConfig, RuntimeApi,
};

const MAX_PAG_SIZE: u32 = 1000;
const TWOX_HASH_LEN: usize = 8; // 8 bytes
const BLAKE_HASH_LEN: usize = 16; // 16 bytes
const STORAGE_PREFIX_LEN: usize = 32; // 32 bytes
const PUBKEY_LEN: usize = 32; // 32 bytes
const ASSET_ID_LEN: usize = 4; // 4 bytes

type Balance = u128;
type VoteWeight = u128;
type Hash = <DefaultConfig as Config>::Hash;
type AccountId = <DefaultConfig as Config>::AccountId;

#[subxt::subxt(
    runtime_metadata_path = "chainx_v4_dev_metadata.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod chainx_v4 {}

/// Arguments required for creating and sending an extrinsic to a sherpax node
#[derive(Clone, Debug, StructOpt)]
pub(crate) struct ExtrinsicOpts {
    /// Websockets url of a chainx v4 node
    #[structopt(name = "url", long, default_value = "ws://localhost:8087")]
    url: String,
    /// The specified block number.
    #[structopt(long)]
    block_number: Option<u32>,
    /// Sort validators in ascending order by total_nomination
    #[structopt(long)]
    sort: bool,
}

fn save_state<P, V>(output_filename: P, state_value: &V) -> Result<()>
where
    P: AsRef<Path>,
    V: ?Sized + serde::Serialize,
{
    let mut output = std::env::current_dir()?;
    output.push(output_filename);
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(output)?;
    Ok(serde_json::to_writer_pretty(file, state_value)?)
}

// Can be used for representing the free balance of PCX and XBTC.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FreeBalanceInfo {
    pub free: Balance,
    pub who: AccountId,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Nomination {
    pub nominee: AccountId,
    pub nomination: Balance,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NominatorInfo {
    pub nominator: AccountId,
    pub nominations: Vec<Nomination>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub who: AccountId,
    pub referral_id: String,
    pub total_nomination: Balance,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct XStakingParams {
    pub validators: Vec<ValidatorInfo>,
    pub nominators: Vec<NominatorInfo>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FullParams {
    pub balances: Vec<FreeBalanceInfo>,
    pub xassets: Vec<FreeBalanceInfo>,
    pub xstaking: XStakingParams,
}

pub struct RegenesisBuilder {
    api: RuntimeApi<DefaultConfig>,
    at: Option<Hash>,
    sort: bool,
}

fn to_account(key: &StorageKey, hash_len: usize) -> Result<AccountId> {
    assert_eq!(key.0.len(), STORAGE_PREFIX_LEN + hash_len + PUBKEY_LEN);

    let pubkey = &key.0[STORAGE_PREFIX_LEN + hash_len..];

    let account_id: AccountId = Decode::decode(&mut pubkey.clone())?;

    Ok(account_id)
}

fn to_account_and_asset_id(key: &StorageKey) -> Result<(AccountId, u32)> {
    assert_eq!(
        key.0.len(),
        STORAGE_PREFIX_LEN + BLAKE_HASH_LEN + PUBKEY_LEN + TWOX_HASH_LEN + ASSET_ID_LEN
    );

    let hashed_key1_key1_hashed_key2_key2 = &key.0[STORAGE_PREFIX_LEN..];
    let hashed_key1_key1 = &hashed_key1_key1_hashed_key2_key2[..BLAKE_HASH_LEN + PUBKEY_LEN];
    let key1 = &hashed_key1_key1[BLAKE_HASH_LEN..];

    let account_id: AccountId = Decode::decode(&mut key1.clone())?;

    let hashed_key2_key2 = &hashed_key1_key1_hashed_key2_key2[hashed_key1_key1.len()..];
    let key2 = &hashed_key2_key2[TWOX_HASH_LEN..];
    let mut raw_asset_id = [0u8; ASSET_ID_LEN];
    raw_asset_id.copy_from_slice(key2);

    let asset_id: u32 = Decode::decode(&mut raw_asset_id.as_slice())?;

    Ok((account_id, asset_id))
}

fn to_account_and_account(key: &StorageKey) -> Result<(AccountId, AccountId)> {
    assert_eq!(
        key.0.len(),
        STORAGE_PREFIX_LEN + TWOX_HASH_LEN + PUBKEY_LEN + TWOX_HASH_LEN + PUBKEY_LEN
    );

    let hashed_key1_key1_hashed_key2_key2 = &key.0[STORAGE_PREFIX_LEN..];
    let hashed_key1_key1 = &hashed_key1_key1_hashed_key2_key2[..TWOX_HASH_LEN + PUBKEY_LEN];
    let key1 = &hashed_key1_key1[TWOX_HASH_LEN..];

    let nominator: AccountId = Decode::decode(&mut key1.clone())?;

    let hashed_key2_key2 = &hashed_key1_key1_hashed_key2_key2[hashed_key1_key1.len()..];
    let key2 = &hashed_key2_key2[TWOX_HASH_LEN..];
    let nominee: AccountId = Decode::decode(&mut key2.clone())?;

    Ok((nominator, nominee))
}

impl RegenesisBuilder {
    pub fn new(api: RuntimeApi<DefaultConfig>, at: Option<Hash>, sort: bool) -> Self {
        Self { api, at, sort }
    }

    /// Collect all the accounts on the chain along with their PCX balance info.
    async fn collect_accounts(&self) -> Result<(Vec<FreeBalanceInfo>, u128)> {
        let mut account_info = self.api.storage().system().account_iter(self.at).await?;

        let mut total_issuance = 0u128;
        let mut exported_accounts = Vec::<FreeBalanceInfo>::new();

        while let Some((key1, info)) = account_info.next().await? {
            let total = info.data.free + info.data.reserved;
            total_issuance += total;
            if total > 0 {
                exported_accounts.push(FreeBalanceInfo {
                    who: to_account(&key1, BLAKE_HASH_LEN)?,
                    free: total,
                })
            }
        }

        Ok((exported_accounts, total_issuance))
    }

    async fn collect_xbtc_accounts(&self) -> Result<(Vec<FreeBalanceInfo>, u128)> {
        let mut asset_balance = self
            .api
            .storage()
            .x_assets()
            .asset_balance_iter(self.at)
            .await?;

        const XBTC_ASSET_ID: u32 = 1;

        let mut total_issuance = 0u128;
        let mut xbtc_accounts = Vec::<FreeBalanceInfo>::new();

        while let Some((key1_key2, asset_info)) = asset_balance.next().await? {
            let (account, asset_id) = to_account_and_asset_id(&key1_key2)?;
            if asset_id == XBTC_ASSET_ID {
                let xbtc_amount = asset_info.iter().map(|(_, b)| b).sum::<Balance>();

                if xbtc_amount > 0 {
                    total_issuance += xbtc_amount;
                    xbtc_accounts.push(FreeBalanceInfo {
                        who: account,
                        free: xbtc_amount,
                    })
                }
            }
        }

        Ok((xbtc_accounts, total_issuance))
    }

    async fn collect_xstaking(&self) -> Result<XStakingParams> {
        // Extract the amount of each nomination record.

        let mut nominations = self
            .api
            .storage()
            .x_staking()
            .nominations_iter(self.at)
            .await?;

        let mut sum_of_nominators = 0u128;
        let mut nominations_map = BTreeMap::<
            AccountId,
            BTreeMap<AccountId, NominatorLedger<Balance, VoteWeight, u32>>,
        >::new();

        while let Some((key1_key2, nominator_ledger)) = nominations.next().await? {
            let (nominator, nominee) = to_account_and_account(&key1_key2)?;
            let entry = nominations_map.entry(nominator).or_default();
            entry.insert(nominee, nominator_ledger);
        }

        let nominators = nominations_map
            .into_iter()
            .filter_map(|(nominator, nominee_map)| {
                let nominations = nominee_map
                    .into_iter()
                    .filter_map(|(nominee, nominator_ledger)| {
                        if nominator_ledger.nomination > 0 {
                            sum_of_nominators += nominator_ledger.nomination;
                            Some(Nomination {
                                nominee,
                                nomination: nominator_ledger.nomination,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                if nominations.is_empty() {
                    None
                } else {
                    Some(NominatorInfo {
                        nominator,
                        nominations,
                    })
                }
            })
            .collect::<Vec<_>>();

        // Extract the total nomination of each validator, for verification purpose.
        // Use Validators storage as it's possible that some validator does
        // not have the ValidatorLedger storage item due to no one has ever voted him.

        let mut validator_ledgers = self
            .api
            .storage()
            .x_staking()
            .validator_ledgers_iter(self.at)
            .await?;

        let mut sum_of_validators = 0u128;
        let mut validators_map = BTreeMap::<AccountId, Balance>::new();

        // Retrieve ValidatorLedgers storage
        while let Some((key1, info)) = validator_ledgers.next().await? {
            sum_of_validators += info.total_nomination;

            let account = to_account(&key1, TWOX_HASH_LEN)?;

            assert!(validators_map
                .insert(account, info.total_nomination)
                .is_none())
        }

        let mut validators = self
            .api
            .storage()
            .x_staking()
            .validators_iter(self.at)
            .await?;

        let mut validators_count = 0usize;
        let mut validator_infos = Vec::<ValidatorInfo>::new();

        // Retrieve Validators storage
        while let Some((key1, profile)) = validators.next().await? {
            validators_count += 1;

            let account = to_account(&key1, TWOX_HASH_LEN)?;
            let referral_id = String::from_utf8_lossy(&profile.referral_id).to_string();
            let total_nomination = validators_map.get(&account).map(|b| *b).unwrap_or_default();

            validator_infos.push(ValidatorInfo {
                referral_id,
                who: account,
                total_nomination,
            });
        }

        // Sort validators in ascending order by total_nomination
        if self.sort {
            validator_infos.sort_by(|v1, v2| v2.total_nomination.cmp(&v1.total_nomination));
        }

        assert_eq!(validator_infos.len(), validators_count);

        println!("sum_of_nominators: {}", sum_of_nominators);
        println!("sum_of_validators: {}", sum_of_validators);
        assert_eq!(sum_of_validators, sum_of_nominators);

        Ok(XStakingParams {
            validators: validator_infos,
            nominators,
        })
    }

    async fn build(&self) -> Result<FullParams> {
        // TODO: verify
        let (balances, total_pcx_issuance) = self.collect_accounts().await?;
        println!("total issuance of PCX: {}", total_pcx_issuance);

        let (xassets, total_xbtc_issuance) = self.collect_xbtc_accounts().await?;
        println!("total issuance of XBTC: {}", total_xbtc_issuance);

        let xstaking = self.collect_xstaking().await?;

        Ok(FullParams {
            balances,
            xassets,
            xstaking,
        })
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();
    sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::ChainXAccount);

    let extrinsic_opts = ExtrinsicOpts::from_args();

    let api = ClientBuilder::new()
        .set_url(extrinsic_opts.url)
        .set_page_size(MAX_PAG_SIZE)
        .build()
        .await?
        .to_runtime_api::<chainx_v4::RuntimeApi<chainx_v4::DefaultConfig>>();

    let block_number = {
        if let Some(number) = extrinsic_opts.block_number {
            number
        } else {
            api.client
                .rpc()
                .block(None)
                .await?
                .expect("Failed to fetch the latest block")
                .block
                .header
                .number
        }
    };

    let block_hash = api
        .client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?;

    let full_params = RegenesisBuilder::new(api, block_hash, extrinsic_opts.sort)
        .build()
        .await?;

    let output_filename = format!("{}_regenesis_params.json", block_number);

    save_state(output_filename, &full_params)?;

    Ok(())
}
