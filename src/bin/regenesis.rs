//! Exports the entire state of balances at a certain block.
//!
//! Used for the SherpaX genesis.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::{
    generic::{Block, SignedBlock},
    traits::AccountIdConversion,
    ModuleId,
};
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

fn save_state<B, P, V>(block_number: B, output_filename: P, state_value: &V) -> Result<()>
where
    B: Display,
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

#[derive(Debug, Serialize, Deserialize)]
struct PcxInfo {
    free: Balance,
    account_id: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
struct XbtcInfo {
    free: Balance,
    account_id: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidatorInfo {
    referral_id: ReferralId,
    total_nomination: Balance,
}

pub struct RegenesisBuilder {
    rpc: Rpc,
    at: Option<Hash>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleNominatorLedger {
    nominee: AccountId,
    nomination: Balance,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleValidatorLedger {
    validator: AccountId,
    referral_id: ReferralId,
    total_nomination: Balance,
}

#[derive(Debug, Serialize, Deserialize)]
struct StakingParams {
    validators: Vec<SingleValidatorLedger>,
    nominations: BTreeMap<AccountId, Vec<SingleNominatorLedger>>,
}

impl RegenesisBuilder {
    pub fn new(rpc: Rpc, at: Option<Hash>) -> Self {
        Self { rpc, at }
    }

    /// Collect all the accounts on the chain along with their PCX balance info.
    async fn collect_accounts(&self) -> Result<(Vec<PcxInfo>, u128)> {
        let account_info = self.rpc.get_accounts_info(self.at).await?;

        let mut total_issuance = 0u128;

        let exported_accounts: Vec<PcxInfo> = account_info
            .into_iter()
            .map(|(id, info)| {
                let total = info.data.free + info.data.reserved;

                total_issuance += total;

                PcxInfo {
                    account_id: id,
                    free: total,
                }
            })
            .collect();

        Ok((exported_accounts, total_issuance))
    }

    async fn collect_xbtc_accounts(&self) -> Result<Vec<XbtcInfo>> {
        let asset_balance = self.rpc.get_asset_balance(self.at).await?;

        const XBTC_ASSET_ID: u32 = 1;

        let xbtc_accounts: Vec<XbtcInfo> = asset_balance
            .into_iter()
            .filter_map(|(account_id, mut asset_info)| {
                asset_info.retain(|&k, _| k == XBTC_ASSET_ID);
                if let Some(xbtc_asset) = asset_info.get(&XBTC_ASSET_ID) {
                    let xbtc_amount = xbtc_asset.values().sum();
                    Some(XbtcInfo {
                        account_id,
                        free: xbtc_amount,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(xbtc_accounts)
    }

    async fn collect_xstaking(&self) -> Result<StakingParams> {
        let nominations = self.rpc.get_nominations(self.at).await?;

        // Extract the amount of each nomination record.
        let converted_nominator_ledgers = nominations
            .into_iter()
            .map(|(k, v)| {
                let v = v
                    .into_iter()
                    .map(|(nominee, nominator_ledger)| SingleNominatorLedger {
                        nominee,
                        nomination: nominator_ledger.nomination,
                    })
                    .collect::<Vec<_>>();
                (k, v)
            })
            .collect::<BTreeMap<_, _>>();

        // Extract the referral id of each validator.
        let validators = self.rpc.get_validators(self.at).await?;

        let get_referral_id = |who: &AccountId| {
            let validator_profile = validators
                .get(who)
                .unwrap_or_else(|| panic!("ValidatorProfile does not exists for {}", who));

            validator_profile.referral_id.clone()
        };

        // Extract the total nomination of each validator, for verification purpose.
        let validator_ledgers = self.rpc.get_validator_ledgers(self.at).await?;
        let convert_validators = validator_ledgers
            .into_iter()
            .map(|(k, v)| SingleValidatorLedger {
                referral_id: get_referral_id(&k),
                validator: k,
                total_nomination: v.total_nomination,
            })
            .collect::<Vec<_>>();

        Ok(StakingParams {
            validators: convert_validators,
            nominations: converted_nominator_ledgers,
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

    let builder = RegenesisBuilder::new(rpc, at);

    // println!("{:#?}", builder.collect_xbtc_accounts().await?);

    // for xbtc_account in builder.collect_xbtc_accounts().await? {
        // println!("account_id: {}", xbtc_account.account_id);
    // }

    println!("{:#?}", builder.collect_xstaking().await?);

    // save_state(block_number, "ksx_accounts", &ksx_accounts)?;
    // save_state(block_number, "dust_accounts", &dust_accounts)?;

    Ok(())
}
