use std::marker::PhantomData;

use codec::{Decode, Encode};
use subxt::{
    balances::{Balances, BalancesEventsDecoder},
    module,
    system::{System, SystemEventsDecoder},
    Call, Event, Store,
};

use crate::runtime::primitives::AssetId;

#[module]
pub trait XMiningAsset: Balances + System {}

// ============================================================================
// Call
// ============================================================================

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ClaimCall<T: XMiningAsset> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Target of the claim.
    pub target: AssetId,
}

// ============================================================================
// Event
// ============================================================================

/// An asset miner claimed the mining reward. [claimer, asset_id, amount]
#[derive(Clone, Debug, PartialEq, Event, Decode)]
pub struct ClaimEvent<T: XMiningAsset> {
    /// claimer.
    pub claimer: <T as System>::AccountId,
    /// asset id of target of the claim.
    pub asset_id: AssetId,
    /// amount
    pub amount: <T as Balances>::Balance,
}

// ============================================================================
// Storage
// ============================================================================

/// AssetLedgers field of the `XMiningAsset` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AssetLedgersStore<T: XMiningAsset> {
    #[store(returns = AssetLedger<MiningWeight, T::BlockNumber>)]
    pub _runtime: PhantomData<T>,
    pub asset_id: AssetId,
}

/// MinerLedgers field of the `XMiningAsset` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct MinerLedgersStore<'a, T: XMiningAsset> {
    #[store(returns = MinerLedger<MiningWeight, T::BlockNumber>)]
    pub account_id: &'a <T as System>::AccountId,
    pub asset_id: AssetId,
}

pub type MiningWeight = u128;

/// Vote weight properties of validator.
#[derive(PartialEq, Eq, Clone, Default, Debug, Encode, Decode)]
pub struct AssetLedger<MiningWeight, BlockNumber> {
    /// Last calculated total vote weight of current validator.
    pub last_total_mining_weight: MiningWeight,
    /// Block number at which point `last_total_vote_weight` just updated.
    pub last_total_mining_weight_update: BlockNumber,
}

/// Mining weight properties of asset miners.
#[derive(PartialEq, Eq, Clone, Debug, Default, Encode, Decode)]
pub struct MinerLedger<MiningWeight, BlockNumber> {
    /// Last calculated total vote weight of current validator.
    pub last_mining_weight: MiningWeight,
    /// Block number at which point `last_total_vote_weight` just updated.
    pub last_mining_weight_update: BlockNumber,
    /// Block number at which point the miner claimed last time.
    pub last_claim: Option<BlockNumber>,
}
