use anyhow::Result;
use structopt::StructOpt;
use subxt::balances::{TransferCallExt, TransferEventExt};

use crate::{
    runtime::{primitives::AccountId, ChainXSigner},
    utils::{build_client, parse_account},
};

/// Balances
#[derive(Debug, StructOpt)]
pub enum Balances {
    /// Transfer some balances from signer to another account.
    Transfer {
        /// receiver
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        dest: AccountId,
        /// amount
        #[structopt(index = 2)]
        value: u128,
    },
}

impl Balances {
    pub async fn run(self, url: String, signer: ChainXSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Balances::Transfer { dest, value } => {
                let result = client
                    .transfer_and_watch(&signer, &dest.into(), value)
                    .await?;
                if let Some(event) = result.transfer()? {
                    println!("Balance transfer success: value: {:?}", event.amount);
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
            }
        }

        Ok(())
    }
}
