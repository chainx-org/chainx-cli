use chainx_runtime::AssetId;
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::{
    module,
    system::{System, SystemEventsDecoder},
    Call,
};

#[module]
pub trait XMiningAsset: System {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ClaimCall<T: XMiningAsset> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Target of the claim.
    pub target: AssetId,
}
