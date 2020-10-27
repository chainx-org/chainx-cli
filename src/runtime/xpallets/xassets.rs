use codec::Encode;
use subxt::{
    balances::{Balances, BalancesEventsDecoder},
    module,
    system::{System, SystemEventsDecoder},
    Call,
};

use crate::runtime::primitives::AssetId;

#[module]
pub trait XAssets: Balances + System {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct TransferCall<'a, T: XAssets> {
    pub dest: &'a <T as System>::Address,
    #[codec(compact)]
    pub asset_id: AssetId,
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
}
