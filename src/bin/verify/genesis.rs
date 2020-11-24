use std::{fs::File, path::PathBuf};

use anyhow::Result;
use chainx_cli::{
    runtime::primitives::{AccountId, Balance},
    serde_num_str, serde_text,
};
use serde::{Deserialize, Serialize};

pub fn read_genesis_json(path: PathBuf) -> Result<GenesisParams> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader::<_, GenesisParams>(file)?)
}

pub type GenesisParams = AllParams<AccountId, Balance, Balance, Balance>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AllParams<AccountId, Balance, AssetBalanceOf, StakingBalanceOf> {
    pub balances: BalancesParams<AccountId, Balance>,
    pub xassets: Vec<FreeBalanceInfo<AccountId, AssetBalanceOf>>,
    pub xstaking: XStakingParams<AccountId, StakingBalanceOf>,
    pub xmining_asset: XMiningAssetParams<AccountId>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalancesParams<AccountId, Balance> {
    pub free_balances: Vec<FreeBalanceInfo<AccountId, Balance>>,
    pub wellknown_accounts: WellknownAccounts<AccountId>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FreeBalanceInfo<AccountId, Balance> {
    pub who: AccountId,
    pub free: Balance,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WellknownAccounts<AccountId> {
    pub legacy_council: AccountId,
    pub legacy_team: AccountId,
    pub legacy_pots: Vec<(AccountId, AccountId)>,
    pub legacy_xbtc_pot: AccountId,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct XStakingParams<AccountId, Balance> {
    pub validators: Vec<ValidatorInfo<AccountId, Balance>>,
    pub nominators: Vec<NominatorInfo<AccountId, Balance>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo<AccountId, Balance> {
    pub who: AccountId,
    #[serde(with = "serde_text")]
    pub referral_id: Vec<u8>,
    pub self_bonded: Balance,
    pub total_nomination: Balance,
    #[serde(with = "serde_num_str")]
    pub total_weight: u128,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NominatorInfo<AccountId, Balance> {
    pub nominator: AccountId,
    pub nominations: Vec<Nomination<AccountId, Balance>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Nomination<AccountId, Balance> {
    pub nominee: AccountId,
    pub nomination: Balance,
    #[serde(with = "serde_num_str")]
    pub weight: u128,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct XMiningAssetParams<AccountId> {
    pub xbtc_miners: Vec<XBtcMiner<AccountId>>,
    pub xbtc_info: XBtcInfo,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct XBtcInfo {
    pub balance: Balance,
    #[serde(with = "serde_num_str")]
    pub weight: u128,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct XBtcMiner<AccountId> {
    pub who: AccountId,
    #[serde(with = "serde_num_str")]
    pub weight: u128,
}
