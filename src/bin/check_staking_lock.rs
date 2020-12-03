use anyhow::Result;
use subxt::{balances::AccountData, system::AccountInfo};

use chainx_cli::runtime::xpallets::xstaking::{LockedType, LocksStoreExt};
use chainx_cli::{block_hash, build_client, rpc::Rpc};
use frame_support::sp_std::collections::btree_map::BTreeMap;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let url = "ws://116.62.46.8:8087";
    let block_number: u32 = 120_000;

    let client = build_client(url.clone()).await?;
    let at = block_hash(&client, Some(block_number)).await?;

    let rpc = Rpc::new(url.clone()).await?;

    let accounts_info = rpc.get_accounts_info(at).await?;

    let mut total_negative = 0;
    let mut total_unlocking = 0;
    for (who, info) in accounts_info {
        let mut locks = client.locks(&who, at).await?;
        let total_locked = locks.values().sum::<u128>();
        total_unlocking += *locks.entry(LockedType::BondedWithdrawal).or_default();
        if total_locked > 0 {
            // let account_data = account.data;
            if total_locked == info.data.misc_frozen {
                println!(
                    "[PASS] {}: total_locked: {}, misc_frozen: {}, locks: {:?}",
                    who, total_locked, info.data.misc_frozen, locks
                );
            } else {
                println!(
                    "[ERROR] XXXXXXXXX {}: total_locked: {}, misc_frozen: {}, locks: {:#?}",
                    who, total_locked, info.data.misc_frozen, locks
                );
            }
            if info.data.free <= info.data.misc_frozen {
                total_negative += info.data.misc_frozen - info.data.free;
                println!(
                    "[ERROR] {} has negative usable: -{}",
                    who,
                    info.data.misc_frozen - info.data.free,
                );
            }
        }
    }

    println!("Total negative: {}", total_negative);
    println!("Total unlockings: {}", total_unlocking);

    Ok(())
}
