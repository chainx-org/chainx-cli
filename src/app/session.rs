use anyhow::Result;
use subxt::session::ValidatorsStoreExt;

use crate::{
    runtime::{primitives::BlockNumber, ChainXSigner},
    utils::{block_hash, build_client},
};

/// Session
#[derive(structopt::StructOpt, Debug)]
pub enum Session {
    SetKeys {
        #[structopt(index = 1, long)]
        keys: String,
    },
    Validators {
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl Session {
    pub async fn run(self, url: String, _signer: ChainXSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Validators { block_number } => {
                let at = block_hash(&client, block_number).await?;
                println!("{:#?}", client.validators(at).await?);
            }
            Self::SetKeys { keys } => {
                let _ = keys;
                todo!()
                // let result = client.set_keys_and_watch(&signer, &call).await?;
                // println!("{:#?}", result);
            }
        }

        Ok(())
    }
}
