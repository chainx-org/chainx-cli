use chainx_cli::{
    runtime::primitives::AccountId,
};
use anyhow::Result;

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
    pub vesting: Vec<(AccountId, u32, u32, u128)>
}

macro_rules! balances {
    ($file:literal, $total_accounts:expr, $total_balance:expr) => {{
        let raw: std::borrow::Cow<'static, [u8]> =  std::borrow::Cow::Owned(include_bytes!($file).to_vec());
        let config: SherpaXBalances = serde_json::from_slice(raw.as_ref())
            .map_err(|e| format!("Error parsing spec file: {}", e))?;

        let total = config
            .balances
            .iter()
            .map(|(_, b)|b)
            .sum::<u128>();

        assert_eq!($total_accounts, config.balances.len());
        assert_eq!($total_balance, total);

        config
    }}
}

macro_rules! vesting {
    ($file:literal, $total_accounts:expr, $total_liquid:expr) => {{
        let raw: std::borrow::Cow<'static, [u8]> =  std::borrow::Cow::Owned(include_bytes!($file).to_vec());
        let config: SherpaXVesting = serde_json::from_slice(raw.as_ref())
            .map_err(|e| format!("Error parsing spec file: {}", e))?;

        let vesting_liquid = config
            .vesting
            .iter()
            .map(|(_, _, _, liquid)|liquid)
            .sum::<u128>();

        assert_eq!($total_accounts, config.vesting.len());
        assert_eq!($total_liquid, vesting_liquid);

        config
    }}
}

pub mod configs {
    use crate::{SherpaXBalances, SherpaXVesting};

    pub fn balances() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    "../../dust_airdrop_10747_826282235120000000000.json",
                    10747,
                    826282235120000000000
                ),
                balances!(
                    "../../non_dust_airdrop_7418_10499173717764880000000000.json",
                    7418,
                    10499173717764880000000000
                ),
            ]
        )
    }

    pub fn vesting() -> Result<Vec<SherpaXVesting>, String> {
        Ok(
            vec![
                vesting!(
                    "../../vesting_airdrop_7417_943235795035215000000000.json",
                    7417,
                    943235795035215000000000
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

    let _vesting = crate::configs::vesting()?;

    Ok(())
}

