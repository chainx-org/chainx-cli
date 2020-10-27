use std::marker::PhantomData;

use codec::Encode;
use subxt::{
    module,
    system::{System, SystemEventsDecoder},
    Call,
};

use crate::runtime::primitives::AssetId;

#[module]
pub trait XMiningAsset: System {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ClaimCall<T: XMiningAsset> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Target of the claim.
    pub target: AssetId,
}
