mod genesis;
mod rpc;

use std::path::PathBuf;

use anyhow::Result;
use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat};
use structopt::StructOpt;

use chainx_cli::runtime::xpallets::xassets::AssetType;

use self::{
    genesis::{
        read_genesis_json, FreeBalanceInfo, Nomination, NominatorInfo, ValidatorInfo, XBtcMiner,
    },
    rpc::Rpc,
};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "chainx-verify",
    author,
    about = "For verifying genesis params",
    no_version
)]
pub struct Config {
    /// Specify the path of genesis json file
    #[structopt(long)]
    genesis: PathBuf,
    /// Specify the WebSocket url of ChainX node.
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    chainx_url: String,
}

impl Config {
    pub fn init() -> Config {
        Config::from_args()
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::init();

    let genesis = read_genesis_json(config.genesis)?;

    set_default_ss58_version(Ss58AddressFormat::ChainXAccount);

    let rpc = Rpc::new(config.chainx_url).await?;

    let genesis_hash = rpc.genesis_hash().await?;
    println!("Genesis Hash: {:?}", genesis_hash);

    let pots_cnt = genesis.balances.wellknown_accounts.legacy_pots.len();
    println!("legacy pot count: {}", pots_cnt);

    println!("================================================================================");
    println!("========================= Verify PCX Balance ===================================");
    println!("================================================================================");

    let account_info = rpc.get_accounts_info(Some(genesis_hash)).await?;

    let mut missing_cnt = 0;
    for FreeBalanceInfo { who, free } in genesis.balances.free_balances {
        if let Some(account_info) = account_info.get(&who) {
            assert_eq!(account_info.data.free + account_info.data.reserved, free);
        } else {
            missing_cnt += 1;
            println!("[ERROR] Missing PCX balance of `{}`", who);
        }
    }
    println!("Missing PCX account count: {}", missing_cnt);

    println!("================================================================================");
    println!("======================= Verify X-BTC Balance ===================================");
    println!("================================================================================");

    let asset_balance = rpc.get_asset_balance(Some(genesis_hash)).await?;

    let mut sum = 0;
    let mut missing_cnt = 0;
    println!("X-BTC account number: {}", genesis.xassets.len());
    for FreeBalanceInfo { who, free } in genesis.xassets {
        if let Some(asset_balance) = asset_balance.get(&who) {
            let xbtc_asset = asset_balance.get(&1).unwrap();
            let xbtc_usable_balance = xbtc_asset
                .get(&AssetType::Usable)
                .cloned()
                .unwrap_or_default();
            assert_eq!(xbtc_usable_balance, free);
            sum += xbtc_usable_balance;
        } else {
            missing_cnt += 1;
            println!("[ERROR] Missing X-BTC asset balance of `{}`", who);
        }
    }
    println!("Missing X-BTC account count: {}", missing_cnt);
    println!("X-BTC Usable Balance Sum (AssetBalance Storage): {}", sum);

    let total_asset_balance = rpc.get_total_asset_balance(Some(genesis_hash)).await?;
    let total_xbtc_balance = total_asset_balance.get(&1).unwrap();
    let total_xbtc_usable_balance = total_xbtc_balance
        .get(&AssetType::Usable)
        .cloned()
        .unwrap_or_default();
    println!(
        "Total X-BTC Usable Balance (TotalAssetBalance Storage): {}",
        total_xbtc_usable_balance
    );

    assert_eq!(
        total_xbtc_usable_balance,
        genesis.xmining_asset.xbtc_info.balance
    );

    println!("================================================================================");
    println!("======================== Verify X-BTC Weight ===================================");
    println!("================================================================================");

    let miner_ledgers = rpc.get_miner_ledgers(Some(genesis_hash)).await?;

    let mut sum = 0;
    let mut missing_cnt = 0;
    println!(
        "X-BTC miners number: {}",
        genesis.xmining_asset.xbtc_miners.len()
    );
    for XBtcMiner { who, weight } in genesis.xmining_asset.xbtc_miners {
        if let Some(miner_ledger) = miner_ledgers.get(&who) {
            let xbtc_miner_ledger = miner_ledger.get(&1).unwrap();
            let xbtc_mining_weight = xbtc_miner_ledger.last_mining_weight;
            assert_eq!(xbtc_mining_weight, weight);
            sum += xbtc_mining_weight;
        } else {
            missing_cnt += 1;
            println!("[ERROR] Missing X-BTC mining weight of `{}`", who);
        }
    }
    println!("Missing X-BTC miner count: {}", missing_cnt);
    println!("X-BTC Mining Weight Sum (MinerLedgers Storage): {}", sum);

    let asset_ledgers = rpc.get_asset_ledgers(Some(genesis_hash)).await?;
    let total_xbtc_mining_weight = asset_ledgers.get(&1).unwrap().last_total_mining_weight;
    println!(
        "Total X-BTC Mining Weight (AssetLedgers Storage): {}",
        total_xbtc_mining_weight
    );

    println!(
        "X-BTC Mining Weight: got = {}, expect = {}",
        total_xbtc_mining_weight, genesis.xmining_asset.xbtc_info.weight
    );

    println!("================================================================================");
    println!("====================== Verify Vote Nomination & Weight =========================");
    println!("================================================================================");

    let nominator_ledgers = rpc.get_nominations(Some(genesis_hash)).await?;

    let mut nomination_sum = 0;
    let mut vote_weight_sum = 0;
    let mut missing_cnt = 0;
    println!("Nominator number: {}", genesis.xstaking.nominators.len());
    for NominatorInfo {
        nominator,
        nominations,
    } in genesis.xstaking.nominators
    {
        for Nomination {
            nominee,
            nomination,
            weight,
        } in nominations
        {
            if let Some(nominator_ledger) = nominator_ledgers.get(&nominator) {
                if let Some(nominator_ledger) = nominator_ledger.get(&nominee) {
                    assert_eq!(nominator_ledger.nomination, nomination);
                    nomination_sum += nominator_ledger.nomination;

                    assert_eq!(nominator_ledger.last_vote_weight, weight);
                    vote_weight_sum += nominator_ledger.last_vote_weight;
                }
            } else {
                missing_cnt += 1;
                println!(
                    "[ERROR] Missing vote nomination & weight of `{}` => `{}`",
                    nominator, nominee
                );
            }
        }
    }
    println!("Missing nominator count: {}", missing_cnt);
    println!(
        "Nomination Sum (Nominations Storage): {}, Vote Weight Sum (Nominations Storage): {}",
        nomination_sum, vote_weight_sum,
    );

    let validator_ledgers = rpc.get_validator_ledgers(Some(genesis_hash)).await?;

    let mut nomination_sum = 0;
    let mut vote_weight_sum = 0;
    let mut missing_cnt = 0;
    println!("Validator number: {}", genesis.xstaking.validators.len());
    for ValidatorInfo {
        who,
        total_nomination,
        total_weight,
        ..
    } in genesis.xstaking.validators
    {
        if let Some(validator_ledger) = validator_ledgers.get(&who) {
            assert_eq!(validator_ledger.total_nomination, total_nomination);
            nomination_sum += validator_ledger.total_nomination;

            assert_eq!(validator_ledger.last_total_vote_weight, total_weight);
            vote_weight_sum += validator_ledger.last_total_vote_weight;
        } else {
            missing_cnt += 1;
            println!(
                "[ERROR] Missing vote nomination & weight of validator `{}`",
                who
            );
        }
    }
    println!("Missing validator count: {}", missing_cnt);
    println!(
        "Nomination Sum (ValidatorLedgers Storage): {}, Vote Weight Sum (ValidatorLedgers Storage): {}",
        nomination_sum, vote_weight_sum,
    );

    Ok(())
}
