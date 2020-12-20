use anyhow::Result;
use chainx_cli::{
    block_hash, build_client, parse_account,
    runtime::{
        primitives::{AccountId, BlockNumber},
        ChainXRuntime,
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

    /// The end block of the balance history.
    #[structopt(long)]
    pub block_number: Option<BlockNumber>,

    /// Ss58 Address version of the network.
    ///
    /// 44 for ChainX mainnet, 42 for Substrate.
    #[structopt(long, default_value = "44")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,
}

pub type ChainBlock<T> = SignedBlock<Block<<T as System>::Header, <T as System>::Extrinsic>>;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let app = App::from_args();

    sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::ChainXAccount);

    let client = build_client(app.url.clone()).await?;

    let who = app.who;

    let block_number = if let Some(block_number) = app.block_number {
        block_number
    } else {
        let latest_block: ChainBlock<ChainXRuntime> = client
            .block(None::<<ChainXRuntime as System>::Hash>)
            .await?
            .expect("Failed to fetch latest block");
        latest_block.block.header.number
    };

    let mut last_free = 0;
    let mut latest_diff = 0;

    for i in 0..=block_number {
        let at = block_hash(&client, Some(i)).await?;
        let account_info = client.account(&who, at).await?;
        let new_free = account_info.data.free;
        if new_free > last_free {
            let diff = new_free - last_free;
            if diff != latest_diff {
                println!("Block#{}, New free {}, new diff[+]: {}", i, new_free, diff);
                latest_diff = diff;
            } else {
                println!("Block#{}, New free {},     diff[+]: {}", i, new_free, diff);
            }
            last_free = new_free;
        } else if new_free < last_free {
            let diff = last_free - new_free;
            if diff != latest_diff {
                println!("Block#{}, New free {}, new diff[-]: {}", i, new_free, diff);
                latest_diff = diff;
            } else {
                println!("Block#{}, New free {},     diff[-]: {}", i, new_free, diff);
            }
            last_free = new_free;
        }
    }

    Ok(())
}
