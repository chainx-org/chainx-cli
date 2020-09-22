use substrate_subxt::{
    balances::{AccountData, Balances},
    extrinsic::DefaultExtra,
    sp_runtime::{generic::Header, OpaqueExtrinsic},
    sudo::Sudo,
    system::System,
    Runtime,
};

use crate::{
    primitives::*,
    xpallet::{xassets::XAssets, xmining_asset::XMiningAsset, xstaking::XStaking},
};

/// Concrete type definitions for ChainX.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChainXRuntime;

impl Runtime for ChainXRuntime {
    type Signature = Signature;
    type Extra = DefaultExtra<Self>;
}

impl System for ChainXRuntime {
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = Hashing;
    type AccountId = AccountId;
    type Address = Address;
    type Header = Header<Self::BlockNumber, Self::Hashing>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Sudo for ChainXRuntime {}

impl Balances for ChainXRuntime {
    type Balance = u128;
}

impl XAssets for ChainXRuntime {}
impl XMiningAsset for ChainXRuntime {}
impl XStaking for ChainXRuntime {}
