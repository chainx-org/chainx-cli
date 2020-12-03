use anyhow::Result;
use structopt::StructOpt;

use crate::{
    runtime::{
        primitives::{AccountId, Balance, BlockNumber},
        xpallets::xstaking::{
            BondCallExt, ChillCallExt, LocksStoreExt, NominationsStoreExt, RebondCallExt,
            RegisterCallExt, SetValidatorCountCallExt, UnbondCallExt, ValidateCallExt,
            ValidatorLedgersStoreExt, ValidatorsStoreExt,
        },
        ChainXSigner,
    },
    utils::{block_hash, build_client, parse_account},
};

/// XStaking
#[derive(Debug, StructOpt)]
pub enum XStaking {
    /// Register as a validator.
    Register {
        /// Validator nickname
        #[structopt(index = 1, long)]
        nickname: String,
        /// Initial validator bond
        #[structopt(index = 2, long)]
        initial_bond: Balance,
    },
    Bond {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        target: AccountId,
        #[structopt(index = 2, long)]
        value: Balance,
    },
    Unbond {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        target: AccountId,
        #[structopt(index = 2, long)]
        value: Balance,
    },
    Rebond {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        from: AccountId,
        #[structopt(index = 2, long, parse(try_from_str = parse_account))]
        to: AccountId,
        #[structopt(index = 3, long)]
        value: Balance,
    },
    Validate,
    Chill,
    SetValidatorCount {
        #[structopt(index = 1, long)]
        new: u32,
    },
    GetDividend {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    GetNomination {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    Validators {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        validator_id: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    ValidatorLedgers {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        validator_id: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    Nominations {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        nominator: AccountId,
        #[structopt(index = 2, long, parse(try_from_str = parse_account))]
        nominatee: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    Locks {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        staker: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl XStaking {
    pub async fn run(self, url: String, signer: ChainXSigner) -> Result<()> {
        let client = build_client(url.clone()).await?;

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
            Self::GetDividend { who, block_number } => {
                let rpc = crate::rpc::Rpc::new(url).await?;
                let at = block_hash(&client, block_number).await?;
                let dividend = rpc.get_staking_dividend(who.clone(), at).await?;
                println!("Staking dividend of {:?}: {:#?}", who, dividend);
            }
            Self::GetNomination { who, block_number } => {
                let rpc = crate::rpc::Rpc::new(url).await?;
                let at = block_hash(&client, block_number).await?;
                let nominations = rpc.get_nominations_rpc(who.clone(), at).await?;
                println!("Nominations of {:?}: {:#?}", who, nominations);
            }
            Self::Storage(storage) => match storage {
                Storage::Validators {
                    validator_id,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let profile = client.validators(&validator_id, at).await?;
                    println!("{:?}: {:#?}", validator_id, profile);
                }
                Storage::ValidatorLedgers {
                    validator_id,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let ledgers = client.validator_ledgers(&validator_id, at).await?;
                    println!("{:?}: {:#?}", validator_id, ledgers);
                }
                Storage::Nominations {
                    nominator,
                    nominatee,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let ledgers = client.nominations(&nominator, &nominatee, at).await?;
                    println!("{:?} => {:?}: {:#?}", nominator, nominatee, ledgers);
                }
                Storage::Locks {
                    staker,
                    block_number,
                } => {
                    let at = block_hash(&client, block_number).await?;
                    let locks = client.locks(&staker, at).await?;
                    let total_locked = locks.values().sum::<u128>();
                    println!("Locks for {:?}", staker);
                    println!("Details: {:#?}", locks);
                    println!("total locked in Staking: {}", total_locked);
                }
            },
        }

        Ok(())
    }
}
