use anyhow::Result;
use chainx_cli::{
    block_hash, build_client,
    rpc::Rpc,
    runtime::{primitives::BlockNumber, xpallets::xstaking::LocksStoreExt},
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

    sp_core::crypto::set_default_ss58_version(app.ss58_prefix);

    let client = build_client(app.url.clone()).await?;
    let at = block_hash(&client, app.block_number).await?;

    let rpc = Rpc::new(app.url.clone()).await?;

    println!(
        "Running at Block #{:?}",
        at.unwrap_or(client.block_hash(None).await?.unwrap_or_default())
    );

    let genesis_hash = client.genesis().clone();

    let accounts_info = rpc.get_accounts_info(at).await?;

    println!("Total account number: {}", accounts_info.len());

    let mut never_claimed = Vec::with_capacity(accounts_info.len());

    for (who, info) in accounts_info {
        let locks = client.locks(&who, Some(genesis_hash)).await?;
        let total_locked = locks.values().sum::<u128>();
        if total_locked > 0 && info.nonce == 0 {
            never_claimed.push((who, total_locked));
        }
    }

    never_claimed.sort_unstable_by_key(|k| k.1);
    never_claimed.reverse();

    let pretty_balance =
        |balance: u128| format!("{}.{}", balance / 100_000_000, balance % 100_000_000);

    for (who, locked) in never_claimed {
        println!(
            "who {} locked {}, but never claimed",
            who,
            pretty_balance(locked)
        );
    }
    Ok(())
}
