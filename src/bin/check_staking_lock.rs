use anyhow::Result;
use chainx_cli::{
    block_hash, build_client,
    rpc::Rpc,
    runtime::{
        primitives::BlockNumber,
        xpallets::xstaking::{LockedType, LocksStoreExt},
    },
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, about, no_version)]
struct App {
    /// The websocket url of ChainX node.
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    pub url: String,

    #[structopt(long)]
    pub block_number: Option<BlockNumber>,

    /// Ss58 Address version of the network.
    ///
    /// 44 for ChainX mainnet, 42 for Substrate.
    #[structopt(long, default_value = "44")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let app = App::from_args();

    let url = "ws://116.62.46.8:8087";

    sp_core::crypto::set_default_ss58_version(app.ss58_prefix);

    let client = build_client(app.url.clone()).await?;
    let at = block_hash(&client, app.block_number).await?;

    let rpc = Rpc::new(url.clone()).await?;

    let accounts_info = rpc.get_accounts_info(at).await?;

    let mut total_negative = 0;
    let mut total_unlocking = 0;
    for (who, info) in accounts_info {
        let mut locks = client.locks(&who, at).await?;
        let total_locked = locks.values().sum::<u128>();
        total_unlocking += *locks.entry(LockedType::BondedWithdrawal).or_default();
        let account_data = info.data;
        if total_locked > 0 {
            if total_locked == account_data.misc_frozen && total_locked == account_data.fee_frozen {
                println!(
                    "[PASS] {}: total_locked: {}, misc_frozen: {}, locks: {:?}",
                    who, total_locked, account_data.misc_frozen, locks
                );
            } else {
                println!(
                    "[ERROR] {}: total_locked: {}, misc_frozen: {}, fee_frozen: {}, locks: {:#?}",
                    who, total_locked, account_data.misc_frozen, account_data.fee_frozen, locks
                );
            }
            if account_data.free < account_data.misc_frozen {
                total_negative += account_data.misc_frozen - account_data.free;
                println!(
                    "[ERROR] {} has negative usable: -{}",
                    who,
                    account_data.misc_frozen - account_data.free,
                );
            }
        }
    }

    println!("Total negative: {}", total_negative);
    println!("Total unlockings: {}", total_unlocking);

    Ok(())
}
