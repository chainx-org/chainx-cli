//! Exports the entire state of balances at a certain block.
//!
//! Used for the SherpaX genesis.

use std::fmt::Display;

use anyhow::Result;
use structopt::StructOpt;

use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::{
    generic::{Block, SignedBlock},
    traits::AccountIdConversion,
    ModuleId,
};
use subxt::system::System;

use chainx_cli::{
    block_hash, build_client,
    rpc::Rpc,
    runtime::{
        primitives::{AccountId, Balance, BlockNumber},
        ChainXClient, ChainXRuntime,
    },
};

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

fn save_snapshot<B, P, V>(block_number: B, prefix: P, value: &V) -> anyhow::Result<()>
where
    B: Display,
    P: Display,
    V: ?Sized + serde::Serialize,
{
    let mut output = std::env::current_dir()?;
    output.push(format!("{}_snapshot_{}.json", prefix, block_number));
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

#[derive(serde::Serialize, serde::Deserialize)]
struct KsxAccount {
    account_id: AccountId,
    free: Balance,
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let app = App::from_args();

    sp_core::crypto::set_default_ss58_version(Ss58AddressFormat::ChainXAccount);

    let client = build_client(app.url.clone()).await?;

    let rpc = Rpc::new(app.url).await?;

    let block_number = if let Some(number) = app.block_number {
        number
    } else {
        latest_block_number(&client).await?
    };

    let at = block_hash(&client, Some(block_number)).await?;

    let account_info = rpc.get_accounts_info(at).await?;

    /// Minimum balance to receive KSX airdrop.
    const MINIMUM_AIRDROP_BALANCE: Balance = 100_000_000;

    let mut total_issuance = 0u128;

    let mut dust_count = 0;
    let mut dust_sum = 0;

    let (mut ksx_accounts, dust_accounts): (
        Vec<Option<BalanceRecord>>,
        Vec<Option<BalanceRecord>>,
    ) = account_info
        .into_iter()
        .map(|(id, info)| {
            let total = info.data.free + info.data.reserved;

            total_issuance += total;

            let maybe_ignored = BalanceRecord {
                account_id: id,
                free: info.data.free,
                reserved: info.data.reserved,
                total,
            };

            if total < MINIMUM_AIRDROP_BALANCE {
                dust_count += 1;
                dust_sum += total;
                (None, Some(maybe_ignored))
            } else {
                (Some(maybe_ignored), None)
            }
        })
        .unzip();

    let treasury_account: AccountId = ModuleId(*b"pcx/trsy").into_account();

    let (ksx_accounts, dust_accounts): (Vec<_>, Vec<_>) = (
        ksx_accounts
            .iter_mut()
            .flatten()
            .map(|record| {
                if record.account_id == treasury_account {
                    record.free += dust_sum;
                    record.total += dust_sum;
                }
                KsxAccount {
                    free: record.total,
                    account_id: record.account_id.clone(),
                }
            })
            .collect(),
        dust_accounts.into_iter().flatten().collect(),
    );

    save_snapshot(block_number, "ksx_accounts", &ksx_accounts)?;
    save_snapshot(block_number, "dust_accounts", &dust_accounts)?;

    println!("Total issuance: {}", total_issuance);
    println!(
        "Total accounts: {}",
        ksx_accounts.len() + dust_accounts.len()
    );
    println!("KSX accounts: {}", ksx_accounts.len());
    println!("Dust accounts(less than 1PCX): {}", dust_count);
    println!("Sum of dust balances: {}", dust_sum);

    // Verify
    let total_ksx = ksx_accounts.iter().map(|r| r.free).sum::<Balance>();

    assert_eq!(total_ksx, total_issuance);

    Ok(())
}
