use substrate_subxt::{
    balances::{AccountData, Balances},
    extrinsic::DefaultExtra,
    sp_core,
    sp_runtime::{
        generic::Header,
        traits::{BlakeTwo256, IdentifyAccount, StaticLookup, Verify},
        MultiSignature, OpaqueExtrinsic,
    },
    sudo::Sudo,
    system::System,
    Runtime,
};

use crate::xpallets::{xassets::XAssets, xmining_asset::XMiningAsset, xstaking::XStaking};

/// Concrete type definitions for ChainX.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChainXRuntime;

impl Runtime for ChainXRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for ChainXRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = <chainx_runtime::Indices as StaticLookup>::Source;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
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
