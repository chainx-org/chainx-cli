use std::{marker::PhantomData, path::PathBuf};

use anyhow::Result;
use structopt::StructOpt;
use substrate_subxt::{
    sudo::{SudoCallExt, SudoUncheckedWeightCallExt},
    system::{SetCodeCall, SetCodeWithoutChecksCall},
    Encoded,
};

use crate::{
    app::system::read_code,
    primitives::*,
    runtime::ChainXRuntime,
    utils::{build_client, ChainXClient, Sr25519Signer},
    xpallet::xstaking::{SetSessionsPerEraCall, SetValidatorCountCall},
};

#[derive(Debug, StructOpt)]
pub enum Sudo {
    #[structopt(name = "sudo")]
    Sudo(Calls),

    #[structopt(name = "sudo-unchecked-weight")]
    SudoUncheckedWeight(Calls),
}

#[derive(Debug, StructOpt)]
pub enum Calls {
    #[structopt(name = "system")]
    System(System),

    #[structopt(name = "xstaking")]
    XStaking(XStaking),
}

#[derive(Debug, StructOpt)]
pub enum XStaking {
    SetValidatorCount {
        #[structopt(index = 1, long)]
        new: u32,
    },
    SetSessionsPerEra {
        #[structopt(index = 1, long)]
        new: BlockNumber,
    },
}

#[derive(Debug, StructOpt)]
pub enum System {
    SetCode {
        #[structopt(index = 1, long, parse(from_os_str))]
        code: PathBuf,
    },
    #[structopt(name = "set-code-without-checks")]
    SetCodeWithoutChecks {
        /// Code path
        #[structopt(index = 1, long, parse(from_os_str))]
        code: PathBuf,
    },
}

impl Calls {
    pub fn as_encoded(&self, client: &ChainXClient) -> Result<Encoded> {
        match self {
            Self::System(system) => match system {
                System::SetCode { code } => {
                    println!("System::SetCode:");
                    let code = read_code(code)?;
                    Ok(client.encode(SetCodeCall::<ChainXRuntime> {
                        _runtime: PhantomData,
                        code: code.as_slice(),
                    })?)
                }
                System::SetCodeWithoutChecks { code } => {
                    println!("System::SetCodeWithoutChecks:");
                    let code = read_code(code)?;
                    Ok(client.encode(SetCodeWithoutChecksCall::<ChainXRuntime> {
                        _runtime: PhantomData,
                        code: code.as_slice(),
                    })?)
                }
            },
            Self::XStaking(xstaking) => match xstaking {
                XStaking::SetValidatorCount { new } => {
                    println!("sudo XStaking::SetValidatorCount:");
                    Ok(client.encode(SetValidatorCountCall::<ChainXRuntime> {
                        _runtime: PhantomData,
                        new: *new,
                    })?)
                }
                XStaking::SetSessionsPerEra { new } => {
                    println!("sudo XStaking::SetSessionsPerEra:");
                    Ok(client.encode(SetSessionsPerEraCall::<ChainXRuntime> {
                        _runtime: PhantomData,
                        new: *new,
                    })?)
                }
            },
        }
    }
}

impl Sudo {
    pub async fn run(self, url: String, signer: Sr25519Signer) -> Result<()> {
        let client = build_client(url).await?;

        println!("Sudo");
        match self {
            Self::Sudo(calls) => {
                let call = calls.as_encoded(&client)?;
                let result = client.sudo_and_watch(&signer, &call).await?;
                println!("{:#?}", result);
            }
            Self::SudoUncheckedWeight(calls) => {
                let call = calls.as_encoded(&client)?;
                let result = client
                    .sudo_unchecked_weight_and_watch(&signer, &call, 0u64)
                    .await?;
                println!("{:#?}", result);
            }
        }

        Ok(())
    }
}
