use anyhow::Result;
use subxt::system::AccountStoreExt;

use chainx_cli::{block_hash, build_client, parse_account};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::ChainXAccount);

    // let url = "ws://116.62.46.8:8087";
    let url = "ws://127.0.0.1:8087";
    let block_number: u32 = 120_000;

    let client = build_client(url.clone()).await?;
    let who = parse_account("5SpVGHLk3RtmehCq1TbZ8HrvGnEpq4ZFSQUAorb6g6F2RRZi").unwrap();

    let mut last_balance = 0;
    let mut new_diff = 0;
    for i in 0..block_number {
        let at = block_hash(&client, Some(i)).await?;
        let account_info = client.account(&who, at).await?;
        let new_free = account_info.data.free;
        if new_free != last_balance {
            let diff = new_free - last_balance;
            if diff != new_diff {
                println!("========= New diff: {}", diff);
                new_diff = diff;
            }
            println!("New free {} at Block {}, diff: {}", new_free, i, diff);
            last_balance = new_free;
        }
    }

    Ok(())
}
