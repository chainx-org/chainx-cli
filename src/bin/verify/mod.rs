mod genesis;

use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Result;
use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat};
use structopt::StructOpt;

use chainx_cli::{
    build_client,
    runtime::{
        primitives::{AccountId, Balance, BlockNumber},
        xpallets::{
            xassets::{AssetBalanceStoreExt, AssetType, TotalAssetBalanceStoreExt},
            xmining_asset::{
                AssetLedger, AssetLedgersStoreExt, MinerLedger, MinerLedgersStoreExt, MiningWeight,
            },
            xstaking::{
                NominationsStoreExt, NominatorLedger, ValidatorLedger, ValidatorLedgersStoreExt,
                VoteWeight,
            },
        },
    },
};
use subxt::system::AccountStoreExt;

use self::genesis::{
    read_genesis_json, FreeBalanceInfo, Nomination, NominatorInfo, ValidatorInfo, XBtcMiner,
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
    let client = build_client(config.chainx_url).await?;
    let genesis_hash = client.genesis().clone();
    println!("Genesis Hash: {:?}", genesis_hash);

    // println!("================================================================================");
    // println!("========================= Verify PCX Balance ====================================");
    // println!("================================================================================");
    //
    // let mut miss_account_cnt = 0;
    // let mut miss_balance = 0;
    // for FreeBalanceInfo { who, free } in genesis.balances.free_balances {
    //     let account_data = client.account(&who, Some(genesis_hash)).await?.data;
    //     if account_data.free + account_data.reserved != free {
    //         miss_account_cnt += 1;
    //         miss_balance += free;
    //         println!(
    //             "[ERROR]   {}: got = {}, expect: {}",
    //             who,
    //             account_data.free + account_data.reserved,
    //             free
    //         );
    //     }
    // }
    // println!(
    //     "miss account number: {}, miss balance: {}",
    //     miss_account_cnt, miss_balance
    // );

    println!("================================================================================");
    println!("======================= Verify X-BTC Balance ===================================");
    println!("================================================================================");

    let mut sum = 0;
    println!("X-BTC account number: {}", genesis.xassets.len());
    for FreeBalanceInfo { who, free } in genesis.xassets {
        let asset_balance: BTreeMap<AssetType, Balance> =
            client.asset_balance(&who, 1, Some(genesis_hash)).await?;
        let usable_balance = asset_balance
            .get(&AssetType::Usable)
            .cloned()
            .unwrap_or_default();
        assert_eq!(usable_balance, free);
        sum += usable_balance;
        // if usable_balance != free {
        //     println!(
        //         "[ERROR]   {}: got = {}, expect: {}",
        //         who, usable_balance, free
        //     );
        // }
    }
    println!(
        "X-BTC usable balance sum (Sum of AssetBalance Storage): {}",
        sum
    );

    let total_xbtc_balance: BTreeMap<AssetType, Balance> =
        client.total_asset_balance(1, Some(genesis_hash)).await?;
    let total_xbtc_usable_balance = total_xbtc_balance
        .get(&AssetType::Usable)
        .cloned()
        .unwrap_or_default();
    println!(
        "Total X-BTC usable balance (TotalAssetBalance Storage): {}",
        total_xbtc_usable_balance
    );

    println!(
        "X-BTC balance: got = {}, expect = {}",
        total_xbtc_usable_balance, genesis.xmining_asset.xbtc_info.balance
    );

    println!("================================================================================");
    println!("======================== Verify X-BTC Weight ===================================");
    println!("================================================================================");

    let mut sum = 0;
    println!(
        "X-BTC miners number: {}",
        genesis.xmining_asset.xbtc_miners.len()
    );
    for XBtcMiner { who, weight } in genesis.xmining_asset.xbtc_miners {
        let xbtc_miner_ledger: MinerLedger<MiningWeight, BlockNumber> =
            client.miner_ledgers(&who, 1, Some(genesis_hash)).await?;
        let xbtc_mining_weight = xbtc_miner_ledger.last_mining_weight;
        assert_eq!(xbtc_mining_weight, weight);
        sum += xbtc_mining_weight;
    }
    println!(
        "X-BTC mining weight sum (Sum of MinerLedgers Storage): {}",
        sum
    );

    let total_xbtc_weight: AssetLedger<MiningWeight, BlockNumber> =
        client.asset_ledgers(1, Some(genesis_hash)).await?;
    let total_xbtc_mining_weight = total_xbtc_weight.last_total_mining_weight;
    println!(
        "Total X-BTC mining weight (AssetLedgers Storage): {}",
        total_xbtc_mining_weight
    );

    println!(
        "X-BTC mining weight: got = {}, expect = {}",
        total_xbtc_mining_weight, genesis.xmining_asset.xbtc_info.weight
    );

    println!("================================================================================");
    println!("====================== Verify Vote Nomination & Weight =========================");
    println!("================================================================================");

    let mut validators = BTreeMap::<AccountId, (u128, u128)>::new();

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
            let nominator_ledger: NominatorLedger<Balance, VoteWeight, BlockNumber> = client
                .nominations(&nominator, &nominee, Some(genesis_hash))
                .await?;

            if nominator_ledger.nomination != nomination
                || nominator_ledger.last_vote_weight != weight
            {
                println!(
                    "[ERROR]   nominator = {}, nominee = {}, \
                    Nomination (got = {}, expect: {}), \
                    Weight (got = {}, expect: {})",
                    nominator,
                    nominee,
                    nominator_ledger.nomination,
                    nomination,
                    nominator_ledger.last_vote_weight,
                    weight
                );
            }
            // assert_eq!(nominator_ledger.nomination, nomination);
            // assert_eq!(nominator_ledger.last_vote_weight, weight);

            validators
                .entry(nominee)
                .and_modify(|(total_nomination, total_vote_weight)| {
                    *total_nomination += nominator_ledger.nomination;
                    *total_vote_weight += nominator_ledger.last_vote_weight;
                })
                .or_insert(Default::default());
        }
    }

    // println!("Validators: {:#?}", validators);

    Ok(())
}
