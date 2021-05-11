use std::fmt::Display;

use anyhow::Result;
use chainx_cli::{
    block_hash, build_client,
    rpc::Rpc,
    runtime::{
        primitives::{AccountId, Balance, BlockNumber},
        ChainXClient, ChainXRuntime,
    },
};
use sp_runtime::generic::{Block, SignedBlock};
use structopt::StructOpt;
use subxt::system::System;

#[derive(StructOpt, Debug)]
#[structopt(author, about, no_version)]
struct App {
    /// The websocket url of ChainX node.
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    pub url: String,

    /// The start block of the balance history.
    #[structopt(long)]
    pub block_number: Option<BlockNumber>,

    /// Ss58 Address version of the network.
    ///
    /// 44 for ChainX mainnet, 42 for Substrate.
    #[structopt(long, default_value = "44")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,
}

pub type ChainBlock<T> = SignedBlock<Block<<T as System>::Header, <T as System>::Extrinsic>>;

async fn latest_block_number(client: &ChainXClient) -> Result<BlockNumber> {
    let latest_block: ChainBlock<ChainXRuntime> = client
        .block(None::<<ChainXRuntime as System>::Hash>)
        .await?
        .expect("Failed to fetch the latest block");
    Ok(latest_block.block.header.number)
}

fn save_snapshot<T, H>(height: H, value: &T) -> anyhow::Result<()>
where
    H: Display,
    T: ?Sized + serde::Serialize,
{
    let mut output = std::env::current_dir()?;
    output.push(format!("chainx_balance_snapshot_{}.json", height));
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(output)?;
    Ok(serde_json::to_writer_pretty(file, value)?)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BalanceRecord {
    account_id: AccountId,
    free: Balance,
    reserved: Balance,
    total: Balance,
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let app = App::from_args();

    sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::ChainXAccount);

    let client = build_client(app.url.clone()).await?;

    let rpc = Rpc::new(app.url).await?;

    let block_number = if let Some(number) = app.block_number {
        number
    } else {
        latest_block_number(&client).await?
    };

    let at = block_hash(&client, Some(block_number)).await?;

    let account_info = rpc.get_accounts_info(at).await?;

    let mut total_issuance = 0u128;

    let balance_records = account_info
        .into_iter()
        .map(|(id, info)| {
            let total = info.data.free + info.data.reserved;

            total_issuance += total;

            BalanceRecord {
                account_id: id,
                free: info.data.free,
                reserved: info.data.reserved,
                total,
            }
        })
        .collect::<Vec<_>>();

    save_snapshot(block_number, &balance_records)?;

    println!("Total accounts: {}", balance_records.len());
    println!("Total issuance: {}", total_issuance);
    Ok(())
}
