use std::{collections::BTreeMap, marker::PhantomData};

use codec::{Decode, Encode};
use subxt::{
    balances::{Balances},
    module,
    system::{System},
    Call, Event, Store,
};

use crate::runtime::primitives::AssetId;

#[module]
pub trait XAssets: Balances + System {}

// ============================================================================
// Call
// ============================================================================

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct TransferCall<'a, T: XAssets> {
    pub dest: &'a <T as System>::Address,
    #[codec(compact)]
    pub asset_id: AssetId,
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
}

// ============================================================================
// Event
// ============================================================================

/// Transfer event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferEvent<T: XAssets> {
    /// Account of asset was transferred from.
    pub from: <T as System>::AccountId,
    /// Asset type was transferred from.
    pub from_type: AssetId,
    /// Account of asset was transferred to.
    pub to: <T as System>::AccountId,
    /// Asset type was transferred to.
    pub to_type: AssetId,
    /// Amount of asset that was transferred.
    pub amount: <T as Balances>::Balance,
}

// ============================================================================
// Storage
// ============================================================================

/// AssetBalance field of the `XAssets` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AssetBalanceStore<'a, T: XAssets> {
    #[store(returns = BTreeMap<AssetType, BalanceOf<T>>)]
    pub account_id: &'a <T as System>::AccountId,
    pub asset_id: AssetId,
}

/// TotalAssetBalance field of the `XAssets` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct TotalAssetBalanceStore<T: XAssets> {
    #[store(returns = BTreeMap<AssetType, BalanceOf<T>>)]
    pub _runtime: PhantomData<T>,
    pub asset_id: AssetId,
}

pub type BalanceOf<T> = <T as Balances>::Balance;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Encode, Decode)]
pub enum AssetType {
    Usable,
    Locked,
    Reserved,
    ReservedWithdrawal,
    ReservedDexSpot,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Usable
    }
}
