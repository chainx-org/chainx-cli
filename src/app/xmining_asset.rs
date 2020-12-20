use anyhow::Result;
use structopt::StructOpt;

use crate::{
    runtime::{
        primitives::{AccountId, AssetId, BlockNumber},
        xpallets::xmining_asset::{
            AssetLedgersStoreExt, ClaimCallExt, ClaimEventExt, MinerLedgersStoreExt,
        },
        ChainXSigner,
    },
    utils::{block_hash, build_client, parse_account},
};

/// XMingAsset
#[derive(Debug, StructOpt)]
pub enum XMingAsset {
    /// Claim asset.
    Claim {
        /// asset id
        #[structopt(index = 1)]
        asset_id: AssetId,
    },
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    AssetLedgers {
        #[structopt(index = 1, long)]
        asset_id: AssetId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    MinerLedgers {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        account_id: AccountId,
        #[structopt(index = 2, long)]
        asset_id: AssetId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl XMingAsset {
    pub async fn run(self, url: String, signer: ChainXSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Claim { asset_id } => {
                let result = client.claim_and_watch(&signer, asset_id).await?;
                if let Some(event) = result.claim()? {
                    println!("XMingAsset claim success: value: {:?}", event.amount);
                } else {
                    println!("Failed to find XMiningAsset::Claim Event");
                }
            }
            Self::Storage(storage) => match storage {
                Storage::AssetLedgers {
                    asset_id,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let asset_ledgers = client.asset_ledgers(asset_id, at).await?;
                    println!("AssetLedgers of {:?}: {:#?}", asset_id, asset_ledgers);
                }
                Storage::MinerLedgers {
                    account_id,
                    asset_id,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let miner_ledgers = client.miner_ledgers(&account_id, asset_id, at).await?;
                    println!("MinerLedgers of {:?}: {:#?}", asset_id, miner_ledgers);
                }
            },
        }

        Ok(())
    }
}
