use anyhow::Result;
use structopt::StructOpt;

use crate::{
    primitives::{AccountId, AccountIndex, Balance, Hash},
    rpc::Rpc,
    types::{AccountData, AccountInfo},
};

const EXPECTED_1_0_TOTAL_ISSUANCE: u128 = 700005000000000u128;
const EXPECTED_ENDOWED_TOTAL_ISSUANCE: u128 = 10000000000000u128 * 4;
const EXPEXTED_TOTAL_ISSUANCE: u128 = EXPECTED_1_0_TOTAL_ISSUANCE + EXPECTED_ENDOWED_TOTAL_ISSUANCE;

#[derive(Debug, StructOpt)]
pub enum Verifier {
    #[structopt(name = "balances")]
    Balances,
}

pub fn total_balance_of(account_info: &AccountInfo<AccountIndex, AccountData<Balance>>) -> Balance {
    account_info.data.free + account_info.data.reserved
}

pub async fn calc_total_issuance(rpc: &Rpc, at: Hash) -> Result<Balance> {
    let accounts_info = rpc.get_accounts_info(Some(at)).await?;
    Ok(accounts_info
        .iter()
        .map(|(_who, account_info)| total_balance_of(account_info))
        .sum::<Balance>())
}

impl Verifier {
    pub async fn run(self, url: String) -> Result<()> {
        let client = crate::utils::build_client(url.clone()).await?;
        let genesis_hash = client.genesis();
        println!("genesis hash:{:?}", genesis_hash);

        let rpc = Rpc::new(&url).await?;

        let total_issuance = calc_total_issuance(&rpc, *genesis_hash).await?;
        if total_issuance != EXPEXTED_TOTAL_ISSUANCE {
            println!(
                "ERROR! total issuance is incorrect, expected: {}, got: {}",
                EXPEXTED_TOTAL_ISSUANCE, total_issuance
            );
        } else {
            println!("PASS: total issuance is {}", total_issuance);
        }

        let nominations = rpc.get_nominations(Some(*genesis_hash)).await?;
        let validator_ledgers = rpc.get_validator_ledgers(Some(*genesis_hash)).await?;

        Ok(())
    }
}
