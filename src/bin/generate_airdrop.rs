use chainx_cli::{
    runtime::primitives::{AccountId, BlockNumber},
};
use anyhow::Result;

use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat};
use sp_runtime::{
    traits::AccountIdConversion, ModuleId,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SherpaXBalances{
    pub balances: Vec<(AccountId, u128)>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SherpaXVesting {
    // * who - Account which we are generating vesting configuration for
    // * begin - Block when the account will start to vest
    // * length - Number of blocks from `begin` until fully vested
    // * liquid - Number of units which can be spent before vesting begins
    pub vesting: Vec<(AccountId, BlockNumber, BlockNumber, u128)>
}

/// Struct to encode the vesting schedule of an individual account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SherpaXSchedule {
    // * who - Account which we are generating vesting configuration for
    // * locked - Locked amount at genesis.
    // * per_block - Amount that gets unlocked every block after `starting_block`.
    // * starting_block - Starting block for unlocking(vesting).
    pub schedules: Vec<(AccountId, u128, u128, BlockNumber)>
}

macro_rules! balances {
    ($file:expr, $total_accounts:expr, $total_balance:expr) => {{
        let file = std::fs::File::open($file)
            .map_err(|e| format!("Error opening balances json file: {}", e))?;

        let mut config: SherpaXBalances = serde_json::from_reader(file)
            .map_err(|e| format!("Error parsing balances json file: {}", e))?;

        config.balances.dedup_by_key(|(account, _)| account.clone());

        let total = config.balances.iter().map(|(_, b)| b).sum::<u128>();

        assert_eq!($total_accounts, config.balances.len());
        assert_eq!($total_balance, total);

        config
    }};
}

pub mod configs {
    use super::*;

    // use for check_duplicate
    pub fn origin_balances() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/origin_chainx_snapshot1_non_dust_7418_10500000000000000000000000_on_2761158.json"
                    ),
                    7418,
                    10500000000000000000000000
                ),
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/origin_comingchat_miners_decimal_8.json"
                    ),
                    252706,
                    146256930040000
                ),
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/origin_sherpax_contributors_1873_9404698487265_decimal_8.json"
                    ),
                    1873,
                    9404698487265
                )
            ]
        )
    }

    // split (snapshot1_balances - snapshot2_balances) to transfer vesting
    pub fn filter_snapshots() -> Result<SherpaXBalances, String> {
        let mut ss1 = balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/origin_chainx_snapshot1_non_dust_7418_10500000000000000000000000_on_2761158.json"
                    ),
                    7418,
                    10500000000000000000000000
                );
        let ss2 = balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/origin_chainx_snapshot2_non_dust_22220_11957624700000000000000000_on_1949100.json"
                    ),
                    22220,
                    11957624700000000000000000
                );

        // Skip 5S7WgdAXVK7mh8REvXfk9LdHs3Xqu9B2E9zzY8e4LE8Gg2ZX
        let treasury_account: AccountId = ModuleId(*b"pcx/trsy").into_account();

        let mut to_genesis = SherpaXBalances {
            balances: vec![]
        };
        let mut to_transfer = SherpaXBalances {
            balances: vec![]
        };

        let treasury = ss1
            .balances
            .iter()
            .find(|t| t.0 == treasury_account)
            .unwrap()
            .clone();

        println!("treasury balance: {:?}", treasury.1);

        ss1.balances.retain(|(account,_)| *account != treasury_account);
        const MIN_BALANCE: u128 = 1_000_000_000_000_000_000;

        for e1 in ss1.balances {
            match ss2.balances.iter().find(|e2| e2.0 == e1.0) {
                Some(e2) if e1.1 > e2.1 && e1.1.saturating_sub(e2.1) >= MIN_BALANCE => {
                    let transfer = e1.1.saturating_sub(e2.1);

                    if e2.1 != 0 {
                        to_genesis.balances.push(e2.clone())
                    }

                    to_transfer.balances.push((e1.0, transfer))
                },
                _ => {
                    to_genesis.balances.push(e1)
                }
            }
        }

        // add treasury
        to_genesis.balances.push(treasury);

        let total = to_genesis.balances.iter().map(|(_, b)| b).sum::<u128>();
        let prefix = format!(
            "genesis_balances_chainx_snapshot_{}_{}",
            to_genesis.balances.len(),
            total
        );

        to_genesis.balances.sort_unstable_by_key(|(_, b)| *b);

        to_file::<SherpaXBalances>(&prefix, &to_genesis)
            .map_err(|e| format!("{:?}", e))?;

        let total = to_transfer.balances.iter().map(|(_, b)| b).sum::<u128>();
        let prefix = format!(
            "transfer_balances_{}_{}",
            to_transfer.balances.len(),
            total
        );

        to_transfer.balances.sort_unstable_by_key(|(_, b)| *b);

        to_file::<SherpaXBalances>(&prefix, &to_transfer)
            .map_err(|e| format!("{:?}", e))?;

        to_vesting_transfer(to_transfer)?;

        Ok(to_genesis)
    }

    pub fn genesis_balances() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/genesis_balances_chainx_snapshot_7418_7873700047225540000000000.json"
                    ),
                    7418,
                    7873700047225540000000000u128
                ),
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/genesis_balances_comingchat_miners_252706_1462569300400000000000000.json"
                    ),
                    252706,
                    1462569300400000000000000u128
                ),
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/genesis_balances_sherpax_contributors_1873_94046984872650000000000.json"
                    ),
                    1873,
                    94046984872650000000000u128
                )
            ]
        )
    }

    pub fn check_genesis_balances() {
        let mut balances: Vec<(AccountId, u128)> = genesis_balances()
            .unwrap()
            .into_iter()
            .flat_map(|s| s.balances)
            .collect();

        balances.dedup_by_key(|(account, _)| account.clone());

        assert_eq!(
            balances.len(),
            7418 + 252706 + 1873,
            "Need manual process duplicate account balance"
        );

        let total = balances.iter().map(|(_, b)| b).sum::<u128>();
        assert_eq!(
            total,
            7873700047225540000000000u128
                .saturating_add(1462569300400000000000000)
                .saturating_add(94046984872650000000000),
        );

        println!("total genesis balances: {:?}", total);
        println!("total genesis accounts: {:?}", balances.len());
    }

    pub fn balances_chainx_snapshot() -> Result<Vec<SherpaXBalances>, String> {
        Ok(vec![filter_snapshots()?])
    }

    pub fn balances_comingchat_mine() -> Result<Vec<SherpaXBalances>, String> {
        let mut origin = balances!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/origin_comingchat_miners_decimal_8.json"),
            252706,
            146256930040000u128
        );

        translate_decimal_from_8_to_18_balances(&mut origin);

        origin.balances.sort_unstable_by_key(|(_, b)| *b);

        to_file::<SherpaXBalances>(
            "genesis_balances_comingchat_miners_252706_1462569300400000000000000",
            &origin
        )
            .map_err(|e| format!("{:?}", e))?;

        Ok(vec![origin])
    }

    pub fn balances_sherpax_crowdloan() -> Result<Vec<SherpaXBalances>, String> {
        let mut origin = balances!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/origin_sherpax_contributors_1873_9404698487265_decimal_8.json"),
            1873,
            9404698487265u128
        );

        translate_decimal_from_8_to_18_balances(&mut origin);

        origin.balances.sort_unstable_by_key(|(_, b)| *b);

        to_file::<SherpaXBalances>("genesis_balances_sherpax_contributors_1873_94046984872650000000000", &origin)
            .map_err(|e| format!("{:?}", e))?;

        Ok(vec![origin])
    }

    pub fn check_origin_duplicate() {
        let mut balances: Vec<(AccountId, u128)> = origin_balances()
            .unwrap()
            .into_iter()
            .flat_map(|s| s.balances)
            .collect();

        balances.dedup_by_key(|(account, _)| account.clone());

        assert_eq!(
            7418 + 252706 + 1873,
            balances.len(),
            "Need manual process duplicate account balance"
        )
    }

    pub fn to_vesting_genesis() -> Result<SherpaXVesting, String> {
        // Skip 5S7WgdAXVK7mh8REvXfk9LdHs3Xqu9B2E9zzY8e4LE8Gg2ZX
        let treasury_account: AccountId = ModuleId(*b"pcx/trsy").into_account();

        let vesting: Vec<(AccountId, BlockNumber, BlockNumber, u128)> = genesis_balances()?
            .into_iter()
            .flat_map(|s| s.balances)
            .filter_map(|(account, free)|{
                if account == treasury_account {
                    return None
                }
                Some((account, 1296000, 2592000, free.saturating_div(10u128)))
            })
            .collect();

        let vesting_free = vesting
            .iter()
            .map(|(_,_,_,free)|free)
            .sum::<u128>();
        let vesting_accounts = vesting.len();

        assert_eq!(
            vesting_accounts,
            7418 + 252706 + 1873 - 1
        );
        assert_eq!(
            vesting_free.saturating_mul(10),
            7873700047225540000000000u128
                .saturating_add(1462569300400000000000000)
                .saturating_add(94046984872650000000000)
                .saturating_sub(1067642049647850000000000)
        );

        let v = SherpaXVesting{ vesting };

        let prefix = format!("genesis_vesting_{}_{}", vesting_accounts, vesting_free);
        to_file::<SherpaXVesting>(&prefix, &v)
            .map_err(|e| format!("{:?}", e))?;

        Ok(v)
    }

    pub fn to_vesting_transfer(to_transfer: SherpaXBalances) -> Result<(), String> {
        let accounts = to_transfer.balances.len();
        let total = to_transfer.balances.iter().map(|(_, b)| b).sum::<u128>();

        let schedules: Vec<(AccountId, u128, u128, BlockNumber)> = to_transfer
            .balances
            .into_iter()
            .map(|(account, balance)|{
                (account, balance, balance.saturating_div(5184000), 3888000)
            })
            .collect();

        let total_locks = schedules.iter().map(|(_, b, _, _)| b).sum::<u128>();

        assert_eq!(schedules.len(), accounts);
        assert_eq!(total_locks, total);

        let prefix = format!("transfer_vesting_{}_{}", accounts, total);
        to_file::<SherpaXSchedule>(&prefix, &SherpaXSchedule{ schedules })
            .map_err(|e| format!("{:?}", e))
    }

    pub fn to_file<V>(prefix: &str, value: &V) -> Result<()>
        where V: ?Sized + serde::Serialize,
    {
        let mut output = std::env::current_dir()?;
        output.push(format!("{}.json", prefix));

        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(output)?;

        Ok(serde_json::to_writer_pretty(file, value)?)
    }

    pub fn translate_decimal_from_8_to_18_balances(origin: &mut SherpaXBalances) {
        let new_balances: Vec<(AccountId, u128)>  = origin
            .balances
            .iter()
            .map(|(account, free)| (account.clone(), free.saturating_mul(10_000_000_000)))
            .collect();

        origin.balances = new_balances
    }

    pub fn translate_decimal_from_12_to_18_balances(origin: &mut SherpaXBalances) {
        let new_balances: Vec<(AccountId, u128)>  = origin
            .balances
            .iter()
            .map(|(account, free)| (account.clone(), free.saturating_mul(1_000_000)))
            .collect();

        origin.balances = new_balances
    }
}

#[async_std::main]
async fn main() -> Result<(), String> {
    set_default_ss58_version(Ss58AddressFormat::ChainXAccount);

    // 1. check origin balances
    crate::configs::check_origin_duplicate();

    // 2. filter origin balances to
    // 2.1) save genesis balances
    // 2.2) save transfer balances
    // 2.3) save transfer vesting
    crate::configs::balances_sherpax_crowdloan()?;
    crate::configs::balances_comingchat_mine()?;
    crate::configs::balances_chainx_snapshot()?;

    // 3. check genesis balances
    crate::configs::check_genesis_balances();

    // 4. generate genesis vesting
    crate::configs::to_vesting_genesis()?;

    Ok(())
}
