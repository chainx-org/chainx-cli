use anyhow::Result;
use structopt::StructOpt;

use xp_genesis_builder::AllParams;

use crate::{
    primitives::{AccountId, AccountIndex, Balance},
    rpc::Rpc,
    types::{AccountData, AccountInfo},
};

#[derive(Debug, StructOpt)]
pub enum Verifier {
    #[structopt(name = "balances")]
    Balances,
}

fn total_balance_of(account_info: &AccountInfo<AccountIndex, AccountData<Balance>>) -> Balance {
    account_info.data.free
        + account_info.data.reserved
        + account_info.data.misc_frozen
        + account_info.data.fee_frozen
}

pub fn genesis_builder_params() -> AllParams<AccountId, Balance, Balance, Balance> {
    serde_json::from_str(include_str!(
        "../../../ChainX-2.0/cli/src/res/genesis_builder_params.json"
    ))
    .map_err(|e| log::error!("{:?}", e))
    .expect("JSON was not well-formatted")
}

impl Verifier {
    pub async fn run(self, url: String) -> Result<()> {
        let client = crate::utils::build_client(url.clone()).await?;
        let genesis_hash = client.genesis();
        println!("genesis hash:{:?}", genesis_hash);
        let rpc = Rpc::new(&url).await?;
        let accounts = rpc.get_accounts(Some(*genesis_hash)).await?;
        let accounts_info = rpc.get_accounts_info(Some(*genesis_hash)).await?;
        let origin_balances = genesis_builder_params().balances.free_balances;
        // println!("{:#?}", accounts_info);
        let total_balance = accounts_info
            .iter()
            .map(|(who, account_info)| {
                if origin_balances
                    .iter()
                    .find(|x| x.who.public() == who)
                    .is_some()
                {
                    println!("-------------- find: {:?}", who);
                } else {
                    println!("============== not found: {:?}", who);
                }
                total_balance_of(account_info)
            })
            .sum::<Balance>();
        let expected_total_balance = 700005000000000u128;
        if total_balance != expected_total_balance {
            println!("expected_total_balance: {:#?}", expected_total_balance);
            println!("                   got: {:#?}", total_balance);
        } else {
            println!("Congrats! {:#?}", total_balance);
        }
        let nominations = rpc.get_nominations(Some(*genesis_hash)).await?;
        let validator_ledgers = rpc.get_validator_ledgers(Some(*genesis_hash)).await?;

        Ok(())
    }
}
