use std::cmp::Ordering;

use anyhow::Result;
use chainx_cli::{
    block_hash, build_client, parse_account,
    runtime::{
        primitives::{AccountId, BlockNumber},
        ChainXClient, ChainXRuntime,
    },
};
use sp_runtime::generic::{Block, SignedBlock};
use structopt::StructOpt;
use subxt::system::{AccountStoreExt, System};

#[derive(StructOpt, Debug)]
#[structopt(author, about, no_version)]
struct App {
    /// The websocket url of ChainX node.
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    pub url: String,

    /// Account
    #[structopt(short, long, parse(try_from_str = parse_account))]
    pub who: AccountId,

    /// The start block of the balance history.
    #[structopt(long)]
    pub start_block: Option<BlockNumber>,

    /// The end block of the balance history.
    #[structopt(long)]
    pub end_block: Option<BlockNumber>,

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
        .expect("Failed to fetch latest block");
    Ok(latest_block.block.header.number)
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let app = App::from_args();

    sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::ChainXAccount);

    let client = build_client(app.url.clone()).await?;

    let who = app.who;
    let start_block = app.start_block.unwrap_or(0);
    let end_block = if let Some(block_number) = app.end_block {
        block_number
    } else {
        latest_block_number(&client).await?
    };

    let mut last_free = 0;
    let mut latest_diff = 0;

    for blk in start_block..=end_block {
        let at = block_hash(&client, Some(blk)).await?;
        let account_info = client.account(&who, at).await?;
        let new_free = account_info.data.free;
        if new_free != last_free {
            let (sign, diff) = match new_free.cmp(&last_free) {
                Ordering::Greater => ("[+]", new_free - last_free),
                Ordering::Less => ("[-]", last_free - new_free),
                Ordering::Equal => unreachable!("They are not equal as just checked above"),
            };

            if diff != latest_diff {
                println!(
                    "{:>14}, free {}, new diff{}: {}",
                    format!("Block#{}", blk),
                    new_free,
                    sign,
                    diff
                );
                latest_diff = diff;
            } else {
                println!(
                    "{:>14}, free {},     diff{}: {}",
                    format!("Block#{}", blk),
                    new_free,
                    sign,
                    diff
                );
            }

            last_free = new_free;
        }
    }

    Ok(())
}
