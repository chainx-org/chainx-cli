use anyhow::Result;
use sp_core::crypto::{set_default_ss58_version, Ss58AddressFormat};
use structopt::StructOpt;
use subxt::Config;
use crate::chainx_v4::DefaultConfig;

#[subxt::subxt(
    runtime_metadata_path = "chainx_v4_metadata.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod chainx_v4 {}

type AccountId = <DefaultConfig as Config>::AccountId;
type BlockNumber = u32;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SherpaXBalances{
    pub balances: Vec<(AccountId, u128)>
}

/// Struct to encode the vesting schedule of an individual account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SherpaXSchedule {
    // * who - Account which we are generating vesting configuration for
    // * locked - Locked amount at genesis.
    // * per_block - Amount that gets unlocked every block after `starting_block`.
    // * starting_block - Starting block for unlocking(vesting).
    pub schedules: Vec<(AccountId, String, String, BlockNumber)>
}

#[derive(StructOpt, Debug)]
#[structopt(author, about, no_version)]
struct App {
    /// The start block of vesting.
    #[structopt(long)]
    pub block_number: Option<BlockNumber>,
}

macro_rules! balances {
    ($file:expr, $total_accounts:expr, $total_balance:expr) => {{
        let file = std::fs::File::open($file)
            .map_err(|e| format!("Error opening balances json file: {}", e))?;

        let mut config: SherpaXBalances = serde_json::from_reader(file)
            .map_err(|e| format!("Error parsing balances json file: {}", e))?;

        config.balances.dedup_by_key(|(account, _)| account.clone());

        config.balances.sort_by_key(|(_, b)| *b);

        let new_balances: Vec<(AccountId, u128)> = config
            .balances
            .iter()
            .map(|(account, free)| (account.clone(), free.saturating_mul(10_000_000_000)))
            .collect();

        config.balances = new_balances;

        let total = config.balances.iter().map(|(_, b)| b).sum::<u128>();

        assert_eq!($total_accounts, config.balances.len());
        assert_eq!($total_balance, total);

        config
    }};
}

pub mod configs {
    use super::*;

    // use for check_duplicate
    pub fn origin_transfer_balances() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/src/airdrop-202204/less-5ksx-223537-44478165122000.json"
                    ),
                    223537,
                    44478165122000u128.saturating_mul(10_000_000_000)
                )
            ]
        )
    }

    pub fn origin_vesting_balances() -> Result<Vec<SherpaXBalances>, String> {
        Ok(
            vec![
                balances!(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/src/airdrop-202204/more-5ksx-7522-38143107821000.json"
                    ),
                    7522,
                    38143107821000u128.saturating_mul(10_000_000_000)
                )
            ]
        )
    }

    pub fn filter_transfer_balances() {
        let balances: Vec<(AccountId, u128)> = origin_transfer_balances()
            .unwrap()
            .into_iter()
            .flat_map(|s| s.balances)
            .collect();

        save_transfer(SherpaXBalances{balances}).unwrap()
    }

    pub fn filter_vesting_balances(start: BlockNumber) {
        const TRANSFER: u128 = 4_000_000_000_000_000_000;

        let balances: Vec<(AccountId, u128)> = origin_vesting_balances()
            .unwrap()
            .into_iter()
            .flat_map(|s| s.balances)
            .collect();

        let mut to_transfer: Vec<(AccountId, u128)> = Vec::new();
        let mut to_vesting: Vec<(AccountId, u128)> = Vec::new();

        assert!(balances.iter().all(|(_, b)| *b >= TRANSFER));

        for (account, balance) in balances {
            to_transfer.push((account.clone(), TRANSFER));
            to_vesting.push((account, balance - TRANSFER))
        }

        save_transfer(SherpaXBalances{balances: to_transfer}).unwrap();
        save_vesting(start, SherpaXBalances{balances: to_vesting}).unwrap();
    }

    pub fn save_transfer(to_transfer: SherpaXBalances) -> Result<(), String> {
        let accounts = to_transfer.balances.len();
        let total = to_transfer.balances.iter().map(|(_, b)| b).sum::<u128>();

        let prefix = format!("transfer_{}_{}", accounts, total);
        to_file::<SherpaXBalances>(&prefix, &to_transfer)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn save_vesting(start: BlockNumber, to_vesting: SherpaXBalances) -> Result<(), String> {
        let accounts = to_vesting.balances.len();
        let total = to_vesting.balances.iter().map(|(_, b)| b).sum::<u128>();

        let schedules: Vec<(AccountId, u128, u128, BlockNumber)> = to_vesting
            .balances
            .into_iter()
            .map(|(account, balance)|{
                (account, balance, balance.saturating_div(3888000), start)
            })
            .collect();

        let total_locks = schedules.iter().map(|(_, b, _, _)| b).sum::<u128>();

        assert_eq!(schedules.len(), accounts);
        assert_eq!(total_locks, total);

        let schedules_format: Vec<(AccountId, String, String, BlockNumber)> = schedules
            .into_iter()
            .map(|s| (s.0, format!("{}", s.1), format!("{}", s.2), s.3))
            .collect();
        let prefix = format!("vesting_{}_{}", accounts, total);
        to_file::<SherpaXSchedule>(&prefix, &SherpaXSchedule{ schedules: schedules_format })
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

}

#[async_std::main]
async fn main() -> Result<(), String> {
    let start = match App::from_args().block_number {
        Some(number) => number,
        None => {
            println!("require start block number with '--block-number' for vesting");
            return Ok(())
        }
    };

    set_default_ss58_version(Ss58AddressFormat::ChainXAccount);

    /*
    accounts：231059
    balances：826212.72943 ksx
    transfer: 231059
    trasnfer_balances: 444781.65122000 + 7522 * 4 = 474869.65122000
    vesting: 7522
    vesting_balances: 381431.07821000 - 7522 * 4 = 351343.07821000 = 826212.72943 - 474869.65122000

    (1) if balance is [1, 5), direct transfer balance
    (2) if balance is [5, +oo), transfer 4 ksx, vesting (balance-4) ksx(at least vest 1 ksx)
    */

    // 1. filter origin transfer balances to
    // save transfer balances
    crate::configs::filter_transfer_balances();

    // 2. filter origin vesting balances to
    // 2.1) save transfer balances
    // 2.2) save transfer vesting
    crate::configs::filter_vesting_balances(start);

    Ok(())
}
