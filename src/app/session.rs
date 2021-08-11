use anyhow::Result;

use crate::{
    frame::session::{NextKeysStoreExt, ValidatorsStoreExt},
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
    NextKeys {
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

            Self::NextKeys { block_number } => {
                let at = block_hash(&client, block_number).await?;
                let validators = client.validators(at).await?;

                for validator in validators {
                    let keys = client.next_keys(&validator, at).await?;
                    if let Some(keys) = keys {
                        let referral_id =
                            crate::utils::get_referral_id(&client, &validator, at).await?;
                        println!("{}: {:#?}", referral_id, keys);
                    }
                }
            }
        }

        Ok(())
    }
}
