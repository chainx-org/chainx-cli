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
    #[allow(dead_code)]
    #[structopt(long, default_value = "44")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,

    #[structopt(long, default_value = "100_000_000")]
    pub min_balance: Balance
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
    output.push(format!("{}_on_{}.json", prefix, block_number));

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

#[derive(serde::Serialize, serde::Deserialize)]
struct SherpaXBalances{
    pub balances: Vec<(AccountId, u128)>
}
impl SherpaXBalances {
    fn from(b: Vec<(AccountId, u128)>) -> Self {
        SherpaXBalances{
            balances: b
        }
    }
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

    // Minimum balance to receive KSX airdrop.
    let min_balance: Balance = app.min_balance;

    let mut total_issuance = 0u128;

    let mut dust_count = 0;
    let mut dust_sum = 0;
    // 5ScUq4UWtp4Tpve8e6YJoWhXDFpapVwiqxjk3drMtEvpR2y9
    let x_association: AccountId = hex_literal::hex!("8387441be6459881fb86af8e36254d537a9d2b86374a553176380811163b7441").into();
    let mut x_association_balance = 0;

    let (mut non_dust_accounts, dust_accounts): (
        Vec<Option<BalanceRecord>>,
        Vec<Option<BalanceRecord>>,
    ) = account_info
        .into_iter()
        .map(|(id, info)| {
            let total = info.data.free + info.data.reserved;

            total_issuance += total;

            if id == x_association {
                x_association_balance = total;
                return (None, None)
            }

            let maybe_ignored = BalanceRecord {
                account_id: id,
                free: info.data.free,
                reserved: info.data.reserved,
                total,
            };

            if total < min_balance {
                dust_count += 1;
                dust_sum += total;
                (None, Some(maybe_ignored))
            } else {
                (Some(maybe_ignored), None)
            }
        })
        .unzip();

    let treasury_account: AccountId = ModuleId(*b"pcx/trsy").into_account();
    let mut treasury_balance = 0u128;
    let mut new_treasury_balance = 0u128;

    let (non_dust_accounts, dust_accounts): (Vec<_>, Vec<_>) = (
        non_dust_accounts
            .iter_mut()
            .flatten()
            .map(|record| {
                if record.account_id == treasury_account {
                    treasury_balance = record.total;

                    record.free += x_association_balance;
                    record.total += x_association_balance;

                    record.free += dust_sum;
                    record.total += dust_sum;

                    new_treasury_balance = record.total;
                }
                KsxAccount {
                    free: record.total,
                    account_id: record.account_id.clone(),
                }
            })
            .collect(),
        dust_accounts.into_iter().flatten().collect(),
    );

    let non_dust_balances: Vec<(AccountId, u128)> = non_dust_accounts
        .iter()
        .map(|non_dust_account|{
            (non_dust_account.account_id.clone(), non_dust_account.free.saturating_mul(10_000_000_000))
        })
        .collect();
    let dust_balances: Vec<(AccountId, u128)> = dust_accounts
        .iter()
        .map(|dust_account|{
            (dust_account.account_id.clone(), dust_account.total.saturating_mul(10_000_000_000))
        })
        .collect();

    println!("   On ChainX(decimals=8)  ");
    println!("        Total issuance: {}", total_issuance);
    let total_accounts = non_dust_accounts.len() + dust_accounts.len();
    println!("        Total accounts: {}", total_accounts);
    println!("     Non-dust accounts: {}", non_dust_accounts.len());
    println!("Minim balance for dust: {}", min_balance);
    println!("         Dust accounts: {}", dust_count);
    println!("   Total dust balances: {}", dust_sum);
    println!("      Treasury balance: {}", treasury_balance);
    println!(" X-association balance: {}", x_association_balance);

    // Verify
    let total_pcx = non_dust_accounts
        .iter()
        .map(|r| r.free)
        .sum::<Balance>();
    assert_eq!(total_pcx, total_issuance);

    let total_ksx = non_dust_balances
        .iter()
        .map(|(_, balance)|balance)
        .sum::<u128>();

    assert_eq!(total_pcx.saturating_mul(10_000_000_000), total_ksx);
    assert_eq!(total_accounts, non_dust_balances.len() + dust_balances.len());

    assert_eq!(dust_count, dust_accounts.len());
    assert_eq!(total_accounts, non_dust_accounts.len() + dust_accounts.len());

    let total_dust = dust_balances.iter().map(|(_, b)|b).sum::<u128>();
    assert_eq!(dust_sum.saturating_mul(10_000_000_000), total_dust);
    let total_non_dust = non_dust_balances.iter().map(|(_, b)|b).sum::<u128>();
    assert_eq!(total_ksx, total_non_dust);

    let new_treasury_balance = new_treasury_balance.saturating_mul(10_000_000_000);

    println!("==========================");
    println!("  On SherpaX(decimals=18) ");
    println!("     Total airdrop ksx: {}", total_ksx);
    println!("        Total accounts: {}", total_accounts);
    println!("     Non-dust accounts: {}", non_dust_balances.len());
    println!("Minim balance for dust: {}", min_balance.saturating_mul(10_000_000_000));
    println!("         Dust accounts: 0");
    println!("   Total dust balances: 0");
    println!("      Treasury balance: {}", new_treasury_balance);
    println!(" X-association balance: 0");
    println!("Total non-dust balance: {}", total_non_dust);

    let non_dust_prefix = format!("origin_chainx_snapshot_non_dust_{}_{}", non_dust_accounts.len(), total_non_dust);
    let dust_prefix = format!("origin_chainx_snapshot_dust_{}_{}", dust_accounts.len(), total_dust);

    save_snapshot(block_number, non_dust_prefix , &SherpaXBalances::from(non_dust_balances))?;
    save_snapshot(block_number, dust_prefix, &SherpaXBalances::from(dust_balances))?;

    Ok(())
}
