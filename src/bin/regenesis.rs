//! Exports the neccessary state for the regenesis purpose.
//!
//! Used for the ChainX 2.0 regenesis.

use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::generic::{Block, SignedBlock};
use subxt::system::System;

use chainx_cli::{
    block_hash, build_client,
    rpc::Rpc,
    runtime::{
        primitives::{AccountId, Balance, BlockNumber, Hash},
        xpallets::xstaking::ReferralId,
        ChainXClient, ChainXRuntime,
    },
};

#[derive(StructOpt, Debug)]
#[structopt(author, about, no_version)]
struct App {
    /// The websocket url of ChainX node.
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    pub url: String,

    /// The start block of the balance history.
    #[structopt(long)]
    pub block_number: Option<BlockNumber>,

    /// Ss58 Address version of the network.
    ///
    /// 44 for ChainX mainnet, 42 for Substrate.
    #[structopt(long, default_value = "44")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,
}

pub type ChainBlock<T> = SignedBlock<Block<<T as System>::Header, <T as System>::Extrinsic>>;

async fn latest_block_number(client: &ChainXClient) -> Result<BlockNumber> {
    let latest_block: ChainBlock<ChainXRuntime> = client
        .block(None::<<ChainXRuntime as System>::Hash>)
        .await?
        .expect("Failed to fetch the latest block");
    Ok(latest_block.block.header.number)
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

pub struct RegenesisBuilder {
    rpc: Rpc,
    at: Option<Hash>,
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
    pub validator: AccountId,
    pub referral_id: ReferralId,
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

impl RegenesisBuilder {
    pub fn new(rpc: Rpc, at: Option<Hash>) -> Self {
        Self { rpc, at }
    }

    /// Collect all the accounts on the chain along with their PCX balance info.
    async fn collect_accounts(&self) -> Result<(Vec<FreeBalanceInfo>, u128)> {
        let account_info = self.rpc.get_accounts_info(self.at).await?;

        let mut total_issuance = 0u128;

        let exported_accounts: Vec<FreeBalanceInfo> = account_info
            .into_iter()
            .filter_map(|(who, info)| {
                let total = info.data.free + info.data.reserved;

                total_issuance += total;

                if total > 0 {
                    Some(FreeBalanceInfo { who, free: total })
                } else {
                    None
                }
            })
            .collect();

        Ok((exported_accounts, total_issuance))
    }

    async fn collect_xbtc_accounts(&self) -> Result<(Vec<FreeBalanceInfo>, u128)> {
        let asset_balance = self.rpc.get_asset_balance(self.at).await?;

        const XBTC_ASSET_ID: u32 = 1;

        let mut total_issuance = 0u128;

        let xbtc_accounts: Vec<FreeBalanceInfo> = asset_balance
            .into_iter()
            .filter_map(|(who, mut asset_info)| {
                asset_info.retain(|&k, _| k == XBTC_ASSET_ID);
                if let Some(xbtc_asset) = asset_info.get(&XBTC_ASSET_ID) {
                    let xbtc_amount = xbtc_asset.values().sum();
                    if xbtc_amount > 0 {
                        total_issuance += xbtc_amount;
                        Some(FreeBalanceInfo {
                            who,
                            free: xbtc_amount,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok((xbtc_accounts, total_issuance))
    }

    async fn collect_xstaking(&self) -> Result<XStakingParams> {
        let nominations = self.rpc.get_nominations(self.at).await?;

        let mut sum_of_nominators = 0u128;

        // Extract the amount of each nomination record.
        let nominators = nominations
            .into_iter()
            .filter_map(|(k, v)| {
                let nominations = v
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
                        nominator: k,
                        nominations,
                    })
                }
            })
            .collect::<Vec<_>>();

        // Extract the referral id of each validator.
        let validators = self.rpc.get_validators(self.at).await?;

        let get_referral_id = |who: &AccountId| {
            let validator_profile = validators
                .get(who)
                .unwrap_or_else(|| panic!("ValidatorProfile does not exist for {}", who));

            validator_profile.referral_id.clone()
        };

        let mut sum_of_validators = 0u128;
        // Extract the total nomination of each validator, for verification purpose.
        let validator_ledgers = self.rpc.get_validator_ledgers(self.at).await?;
        let validators = validator_ledgers
            .into_iter()
            .map(|(k, v)| {
                sum_of_validators += v.total_nomination;
                ValidatorInfo {
                    referral_id: get_referral_id(&k),
                    validator: k,
                    total_nomination: v.total_nomination,
                }
            })
            .collect::<Vec<_>>();

        println!("sum_of_nominators: {}", sum_of_nominators);
        println!("sum_of_validators: {}", sum_of_validators);
        assert_eq!(sum_of_validators, sum_of_nominators);

        Ok(XStakingParams {
            validators,
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

    let app = App::from_args();

    sp_core::crypto::set_default_ss58_version(Ss58AddressFormat::ChainXAccount);

    let client = build_client(app.url.clone()).await?;

    let rpc = Rpc::new(app.url).await?;

    let block_number = if let Some(number) = app.block_number {
        number
    } else {
        latest_block_number(&client).await?
    };

    let at = block_hash(&client, Some(block_number)).await?;

    let full_params = RegenesisBuilder::new(rpc, at).build().await?;

    let output_filename = format!("{}_regenesis_params.json", block_number);

    save_state(output_filename, &full_params)?;

    Ok(())
}
