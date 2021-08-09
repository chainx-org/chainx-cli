use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;
use subxt::system::{AccountStoreExt, SetCodeWithoutChecksCallExt};

use crate::{
    runtime::{
        primitives::{AccountId, BlockNumber},
        ChainXSigner,
    },
    utils::{block_hash, build_client, parse_account, read_code},
};

/// Grandpa
#[derive(Debug, StructOpt)]
pub enum Grandpa {
    /// Get the account information.
    RoundState {
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl Grandpa {
    pub async fn run(self, url: String, signer: ChainXSigner) -> Result<()> {
        let client = build_client(url.clone()).await?;

        match self {
            Self::RoundState { block_number } => {
                use crate::rpc::Rpc;

                let at = block_hash(&client, block_number).await?;

                let rpc = Rpc::new(url).await?;
                let round_states = rpc.get_grandpa_round_state(at).await?;
                println!("Grandpa round states #{:?}: {:#?}", at, round_states);
            }
        }

        Ok(())
    }
}
