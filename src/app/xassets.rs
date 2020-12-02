use anyhow::Result;
use structopt::StructOpt;

use crate::{
    runtime::{
        primitives::{AccountId, AssetId, BlockNumber},
        xpallets::xassets::{
            AssetBalanceStoreExt, TotalAssetBalanceStoreExt, TransferCallExt, TransferEventExt,
        },
        ChainXSigner,
    },
    utils::{block_hash, build_client, parse_account},
};

/// XAssets
#[derive(Debug, StructOpt)]
pub enum XAssets {
    /// Transfer some assets from signer to another account.
    Transfer {
        /// receiver
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        dest: AccountId,
        /// asset id
        #[structopt(index = 2)]
        asset_id: AssetId,
        /// amount
        #[structopt(index = 3)]
        value: u128,
    },
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    AssetBalance {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        account_id: AccountId,
        #[structopt(index = 2, long)]
        asset_id: AssetId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    TotalAssetBalance {
        #[structopt(index = 1, long)]
        asset_id: AssetId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl XAssets {
    pub async fn run(self, url: String, signer: ChainXSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Transfer {
                dest,
                asset_id,
                value,
            } => {
                let result = client
                    .transfer_and_watch(&signer, &dest.into(), asset_id, value)
                    .await?;
                if let Some(event) = result.transfer()? {
                    println!("XAssets transfer success: value: {:?}", event.amount);
                } else {
                    println!("Failed to find XAssets::Transfer Event");
                }
            }
            Self::Storage(storage) => match storage {
                Storage::AssetBalance {
                    account_id,
                    asset_id,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let asset_balance = client.asset_balance(&account_id, asset_id, at).await?;
                    println!("AssetBalance of {:?}: {:#?}", account_id, asset_balance);
                }
                Storage::TotalAssetBalance {
                    asset_id,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let total_asset_balance = client.total_asset_balance(asset_id, at).await?;
                    println!(
                        "TotalAssetBalance of {:?}: {:#?}",
                        asset_id, total_asset_balance
                    );
                }
            },
        }

        Ok(())
    }
}
