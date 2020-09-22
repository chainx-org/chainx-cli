use anyhow::Result;
use structopt::StructOpt;

use crate::{
    primitives::{AccountId, Balance},
    utils::{build_client, parse_account, Sr25519Signer},
    xpallets::xstaking::{
        BondCallExt, ChillCallExt, RebondCallExt, RegisterCallExt, SetValidatorCountCallExt,
        UnbondCallExt, ValidateCallExt, ValidatorsStoreExt,
    },
};

/// XStaking
#[derive(Debug, StructOpt)]
pub enum XStaking {
    /// Register as a validator.
    #[structopt(name = "register")]
    Register {
        /// Validator nickname
        #[structopt(index = 1, long)]
        nickname: String,
        /// Initial validator bond
        #[structopt(index = 2, long)]
        initial_bond: Balance,
    },
    #[structopt(name = "bond")]
    Bond {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        target: AccountId,
        #[structopt(index = 2, long)]
        value: Balance,
    },
    #[structopt(name = "unbond")]
    Unbond {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        target: AccountId,
        #[structopt(index = 2, long)]
        value: Balance,
    },
    #[structopt(name = "rebond")]
    Rebond {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        from: AccountId,
        #[structopt(index = 2, long, parse(try_from_str = parse_account))]
        to: AccountId,
        #[structopt(index = 3, long)]
        value: Balance,
    },
    #[structopt(name = "validate")]
    Validate,
    #[structopt(name = "chill")]
    Chill,
    #[structopt(name = "set-validator-count")]
    SetValidatorCount {
        #[structopt(index = 1, long)]
        new: u32,
    },
    #[structopt(name = "storage")]
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    #[structopt(name = "validators")]
    Validators {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        validator_id: AccountId,
    },
}

impl XStaking {
    pub async fn run(self, url: String, signer: Sr25519Signer) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Register {
                nickname,
                initial_bond,
            } => {
                let result = client
                    .register_and_watch(&signer, nickname.as_bytes().to_vec(), initial_bond)
                    .await?;
                println!("register result:{:#?}", result);
            }
            Self::Bond { target, value } => {
                let result = client
                    .bond_and_watch(&signer, &target.into(), value)
                    .await?;
                println!("bond result:{:#?}", result);
            }
            Self::Unbond { target, value } => {
                let result = client
                    .unbond_and_watch(&signer, &target.into(), value)
                    .await?;
                println!("unbond result:{:#?}", result);
            }
            Self::Rebond { from, to, value } => {
                let result = client
                    .rebond_and_watch(&signer, &from.into(), &to.into(), value)
                    .await?;
                println!("rebond result:{:#?}", result);
            }
            Self::Validate => {
                let result = client.validate_and_watch(&signer).await?;
                println!("validate result:{:#?}", result);
            }
            Self::Chill => {
                let result = client.chill_and_watch(&signer).await?;
                println!("chill result:{:#?}", result);
            }
            Self::SetValidatorCount { new } => {
                let result = client.set_validator_count_and_watch(&signer, new).await?;
                println!("set_validator_count result:{:#?}", result);
            }
            Self::Storage(storage) => match storage {
                Storage::Validators { validator_id } => {
                    println!(
                        "ValidatorProfile of {:?}: {:#?}",
                        validator_id,
                        client.validators(&validator_id, None).await?
                    );
                }
            },
        }

        Ok(())
    }
}
