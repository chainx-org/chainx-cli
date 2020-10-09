use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Result;
use structopt::StructOpt;
use subxt::system::{AccountStoreExt, SetCodeWithoutChecksCallExt};

use crate::{
    primitives::AccountId,
    utils::{build_client, parse_account, Sr25519Signer},
};

/// System
#[derive(Debug, StructOpt)]
pub enum System {
    /// Get the account information.
    AccountInfo {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
    },
    /// Transfer some balances from signer to another account.
    #[structopt(name = "set-code-without-checks")]
    SetCodeWithoutChecks {
        /// Code path
        #[structopt(index = 1, long, parse(from_os_str))]
        code: PathBuf,
    },
}

pub fn read_code<P: AsRef<Path>>(code_path: P) -> Result<Vec<u8>> {
    let mut file = File::open(code_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

impl System {
    pub async fn run(self, url: String, signer: Sr25519Signer) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::AccountInfo { who } => {
                let account_info = client.account(&who, None).await?;
                println!("AccountInfo of {:?}: {:#?}", who, account_info);
            }
            Self::SetCodeWithoutChecks { code } => {
                let result = client
                    .set_code_without_checks_and_watch(&signer, &read_code(code)?)
                    .await?;
                println!("set_code_without_checks result:{:#?}", result);
            }
        }

        Ok(())
    }
}
