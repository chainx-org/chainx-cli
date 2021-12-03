use chainx_cli::{
    runtime::primitives::AccountId,
};
use anyhow::Result;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SherpaXBalances{
    pub balances: Vec<(AccountId, u128)>
}

macro_rules! balances {
    ($file:expr, $total_accounts:expr, $total_balance:expr) => {{
        let file = std::fs::File::open($file)
            .map_err(|e| format!("Error opening balances json file: {}", e))?;

        let config: SherpaXBalances = serde_json::from_reader(file)
            .map_err(|e| format!("Error parsing balances json file: {}", e))?;

        let total = config.balances.iter().map(|(_, b)| b).sum::<u128>();

        assert_eq!($total_accounts, config.balances.len());
        assert_eq!($total_balance, total);

        config
    }};
}

pub mod configs {
    use crate::SherpaXBalances;

    pub fn balances() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/dust_airdrop_10747_826282235120000000000.json"),
                    10747,
                    826282235120000000000
                ),
                balances!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/non_dust_airdrop_7418_10499173717764880000000000.json"),
                    7418,
                    10499173717764880000000000
                ),
            ]
        )
    }

    pub fn balances2() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/ksx_mine_248522_146256930040000.json"),
                    252706,
                    146256930040000
                )
            ]
        )
    }
}

#[async_std::main]
async fn main() -> Result<(), String> {
    let total = crate::configs::balances()?
        .into_iter()
        .flat_map(|s| s.balances)
        .map(|(_, b)|b)
        .sum::<u128>();

    assert_eq!(total, 10500000000000000000000000);

    Ok(())
}

#[test]
fn filter_10_balances() {
    let balances: Vec<(AccountId, u128)> = crate::configs::balances2()
        .unwrap_or_default()
        .into_iter()
        .flat_map(|s| s.balances)
        .collect();

    let filtered: Vec<(AccountId, u128)> = balances
        .iter()
        .filter(|(_, free)| *free >= 10_000_000_00u128 )
        .cloned()
        .collect();

    let total = filtered
        .iter()
        .fold(0, |acc: u128, &(_, n)| acc + n);

    println!("filter_10_balances: accounts = {}, total = {}", filtered.len(), total);
}

#[test]
fn filter_100_balances() {
    let balances: Vec<(AccountId, u128)> = crate::configs::balances2()
        .unwrap_or_default()
        .into_iter()
        .flat_map(|s| s.balances)
        .collect();

    let filtered: Vec<(AccountId, u128)> = balances
        .iter()
        .filter(|(_, free)| *free >= 100_000_000_00u128 )
        .cloned()
        .collect();

    let total = filtered
        .iter()
        .fold(0, |acc: u128, &(_, n)| acc + n);

    println!("filter_100_balances: accounts = {}, total = {}", filtered.len(), total);
}

// cargo test --release --package chainx-cli --bin check_snapshot refresh_ksx_mine -- --nocapture
#[test]
fn refresh_ksx_mine() {
    let balances: Vec<(AccountId, u128)> = crate::configs::balances2()
        .unwrap_or_default()
        .into_iter()
        .flat_map(|s| s.balances)
        .map(|(account, free)|(account, free.saturating_mul(10_000_000_000u128)))
        .collect();

    println!("refresh_ksx_mine: accounts = {}", balances.len());

    let file = std::fs::File::create("ksx_mine_248522_1462569300400000000000000.json").unwrap();
    serde_json::to_writer(file, &SherpaXBalances{ balances }).unwrap();
}
