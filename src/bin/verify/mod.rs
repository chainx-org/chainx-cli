mod genesis;

use std::path::PathBuf;

use anyhow::Result;
use codec::Encode;
use frame_support::parameter_types;
use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat, UncheckedFrom};
use sp_runtime::{
    traits::{AccountIdConversion, Hash},
    ModuleId,
};
use structopt::StructOpt;
use subxt::system::System;

use chainx_cli::runtime::{
    primitives::{AccountId, AssetId, Balance, BlockNumber},
    xpallets::xassets::AssetType,
    ChainXRuntime,
};

use self::genesis::{
    read_genesis_json, FreeBalanceInfo, Nomination, NominatorInfo, ValidatorInfo, XBtcMiner,
};
use chainx_cli::rpc::Rpc;
use frame_support::sp_std::collections::btree_map::BTreeMap;

/// The decimals of PCX.
const PCX: u128 = 100_000_000;

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

parameter_types! {
    pub const TreasuryModuleId: ModuleId = ModuleId(*b"pcx/trsy");
}

/// Returns the validator account by the given reward pot account.
fn validator_for<'a, I: Iterator<Item = &'a (AccountId, AccountId)>>(
    target_pot: &AccountId,
    mut pots: I,
) -> Option<&'a AccountId> {
    pots.find(|(pot, _)| *pot == *target_pot)
        .map(|(_, validator)| validator)
}

/// BTC asset in ChainX backed by the Mainnet Bitcoin.
pub const X_BTC: AssetId = 1;

/// Simple Asset reward pot account determiner.
///
/// Formula: `blake2_256(blake2_256(asset_id) + blake2_256(registered_block_number))`
fn asset_reward_pot_account_for(asset_id: &AssetId) -> AccountId {
    let id_hash = <ChainXRuntime as System>::Hashing::hash(&asset_id.to_le_bytes()[..]);

    let registered_at = BlockNumber::default();
    let registered_at_hash = <ChainXRuntime as System>::Hashing::hash(&registered_at.encode());

    let id_slice = id_hash.as_ref();
    let registered_at_slice = registered_at_hash.as_ref();

    let mut buf = Vec::with_capacity(id_slice.len() + registered_at_slice.len());
    buf.extend_from_slice(id_slice);
    buf.extend_from_slice(registered_at_slice);

    UncheckedFrom::unchecked_from(<ChainXRuntime as System>::Hashing::hash(&buf[..]))
}

/// Simple validator reward pot account determiner.
///
/// Formula: `blake2_256(blake2_256(validator_pubkey) + blake2_256(registered_at))`
fn validator_reward_pot_account_for(validator: &AccountId) -> AccountId {
    let validator_hash = <ChainXRuntime as System>::Hashing::hash(validator.as_ref());

    let registered_at = BlockNumber::default();
    let registered_at_hash = <ChainXRuntime as System>::Hashing::hash(&registered_at.encode());

    let validator_slice = validator_hash.as_ref();
    let registered_at_slice = registered_at_hash.as_ref();

    let mut buf = Vec::with_capacity(validator_slice.len() + registered_at_slice.len());
    buf.extend_from_slice(validator_slice);
    buf.extend_from_slice(registered_at_slice);

    UncheckedFrom::unchecked_from(<ChainXRuntime as System>::Hashing::hash(&buf[..]))
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

    let legacy_council = genesis.balances.wellknown_accounts.legacy_council;
    let treasury_account: AccountId = TreasuryModuleId::get().into_account();
    println!(
        "legacy_council `{}` => treasury_account `{}`",
        legacy_council, treasury_account
    );

    let legacy_team = genesis.balances.wellknown_accounts.legacy_team;
    let vesting_account = rpc.get_vesting_account(Some(genesis_hash)).await?;
    println!(
        "legacy_team `{}` => vesting_account `{}`",
        legacy_team, vesting_account
    );

    let legacy_xbtc_pot = genesis.balances.wellknown_accounts.legacy_xbtc_pot;
    let new_xbtc_pot = asset_reward_pot_account_for(&X_BTC);
    println!(
        "legacy_xbtc_pot `{}` => new_xbtc_pot `{}`",
        legacy_xbtc_pot, new_xbtc_pot
    );

    let legacy_pots = genesis.balances.wellknown_accounts.legacy_pots;
    let mut new_pots = BTreeMap::new();
    for (legacy_pot, validator) in legacy_pots.clone() {
        let new_pot = validator_reward_pot_account_for(&validator);
        println!("validator `{}`", validator);
        println!("legacy_pot `{}` => new_pot `{}`", legacy_pot, new_pot);
        new_pots.insert(validator, new_pot);
    }
    let validators = rpc.get_validators(Some(genesis_hash)).await?;
    println!(
        "validators number (XStaking Validators Storage): {}",
        validators.len()
    );

    println!("================================================================================");
    println!("========================= Verify PCX Balance ===================================");
    println!("================================================================================");

    let account_info = rpc.get_accounts_info(Some(genesis_hash)).await?;

    let mut sum = 0;
    let mut missing_cnt = 0;
    for FreeBalanceInfo { who, free } in genesis.balances.free_balances {
        let who = if who == legacy_council {
            treasury_account.clone()
        } else if who == legacy_team {
            vesting_account.clone()
        } else if who == legacy_xbtc_pot {
            new_xbtc_pot.clone()
        } else if let Some(validator) = validator_for(&who, legacy_pots.iter()) {
            if let Some(new_pot) = new_pots.get(validator).cloned() {
                new_pot
            } else {
                who
            }
        } else {
            who
        };

        if let Some(account_info) = account_info.get(&who) {
            let free_from_chain = account_info.data.free + account_info.data.reserved;
            if who == vesting_account {
                println!("verify team account");
                // The missing 5000 PCX of team account is used for the 5 genesis nodes in 2.0 POA stage.
                assert_eq!(free_from_chain + 5000 * PCX, free, "team: {}", who);
                sum += free_from_chain + 5000 * PCX;
            } else if who == treasury_account {
                println!("verify treasury account");
                // The missing 100 PCX of treasury account is used for the root account.
                assert_eq!(free_from_chain + 100 * PCX, free, "treasury: {}", who);
                sum += free_from_chain + 100 * PCX;
            } else if who == new_xbtc_pot {
                println!("verify xbtc pot account");
                assert_eq!(free_from_chain, free, "xbtc pot: {}", who);
                sum += free_from_chain;
            } else {
                // if free_from_chain != free {
                //     println!(
                //         "[ERROR] who: {}, chain: {}, json: {}",
                //         who,
                //         free_from_chain,
                //         free
                //     );
                // }
                assert_eq!(free_from_chain, free, "{}", who);
                sum += free_from_chain;
            }
        } else {
            missing_cnt += 1;
            println!("[ERROR] Missing PCX balance of `{}`", who);
        }
    }
    println!("Missing PCX account count: {}", missing_cnt);
    println!("PCX Balance Sum (System Account Storage): {}", sum);

    let total_pcx_balance = account_info
        .into_iter()
        .map(|(_account, info)| info.data.free + info.data.reserved)
        .sum::<Balance>();
    println!(
        "Total PCX Balance (System Account Storage): {}",
        total_pcx_balance
    );

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
    println!(
        "X-BTC Usable Balance Sum (XAssets AssetBalance Storage): {}",
        sum
    );

    let total_asset_balance = rpc.get_total_asset_balance(Some(genesis_hash)).await?;
    let total_xbtc_balance = total_asset_balance.get(&1).unwrap();
    let total_xbtc_usable_balance = total_xbtc_balance
        .get(&AssetType::Usable)
        .cloned()
        .unwrap_or_default();
    println!(
        "Total X-BTC Usable Balance (XAssets TotalAssetBalance Storage): {}",
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
    println!(
        "X-BTC Mining Weight Sum (XMiningAsset MinerLedgers Storage): {}",
        sum
    );

    let asset_ledgers = rpc.get_asset_ledgers(Some(genesis_hash)).await?;
    let total_xbtc_mining_weight = asset_ledgers.get(&1).unwrap().last_total_mining_weight;
    println!(
        "Total X-BTC Mining Weight (XMiningAsset AssetLedgers Storage): {}",
        total_xbtc_mining_weight
    );

    assert_eq!(
        total_xbtc_mining_weight,
        genesis.xmining_asset.xbtc_info.weight
    );

    println!("================================================================================");
    println!("====================== Verify Vote Nomination & Weight =========================");
    println!("================================================================================");

    let validator_ledgers = rpc.get_validator_ledgers(Some(genesis_hash)).await?;
    let (total_nomination, total_vote_weight) = (
        validator_ledgers
            .iter()
            .map(|(_, validator_ledger)| validator_ledger.total_nomination)
            .sum::<Balance>(),
        validator_ledgers
            .iter()
            .map(|(_, validator_ledger)| validator_ledger.last_total_vote_weight)
            .sum::<Balance>(),
    );
    println!(
        "Total Nomination: {}, Total Vote Weight: {}",
        total_nomination, total_vote_weight
    );

    let nominator_ledgers = rpc.get_nominations(Some(genesis_hash)).await?;

    let mut nomination_sum = 0;
    let mut vote_weight_sum = 0;
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
            }
        }
    }
    println!(
        "Nominator Nomination Sum (XStaking Nominations Storage): {}, Nominator Vote Weight Sum (XStaking Nominations Storage): {}",
        nomination_sum, vote_weight_sum,
    );
    // The 5000 PCX nomination is from the 5 genesis nodes in 2.0 POA.
    assert_eq!(total_nomination, nomination_sum + 5000 * PCX);
    assert_eq!(total_vote_weight, vote_weight_sum);

    let mut nomination_sum = 0;
    let mut vote_weight_sum = 0;
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
        }
    }
    println!(
        "Validator Nomination Sum (ValidatorLedgers Storage): {}, Validator Vote Weight Sum (ValidatorLedgers Storage): {}",
        nomination_sum, vote_weight_sum,
    );
    // The 5000 PCX nomination is from the 5 genesis nodes in 2.0 POA.
    assert_eq!(total_nomination, nomination_sum + 5000 * PCX);
    assert_eq!(total_vote_weight, vote_weight_sum);

    Ok(())
}
