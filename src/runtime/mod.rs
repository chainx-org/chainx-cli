pub mod primitives;
pub mod xpallets;

use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{generic::Header, impl_opaque_keys, OpaqueExtrinsic};
use subxt::{
    balances::{AccountData, Balances},
    extrinsic::DefaultExtra,
    sudo::Sudo,
    system::System,
    Client, PairSigner, Runtime,
};

use crate::frame::session::Session;

use self::{
    primitives::*,
    xpallets::{xassets::XAssets, xmining_asset::XMiningAsset, xstaking::XStaking},
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
    type Balance = Balance;
}

/// BABE marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Babe;
impl sp_runtime::BoundToRuntimeAppPublic for Babe {
    type Public = BabeId;
}

/// GRANDPA marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Grandpa;
impl sp_runtime::BoundToRuntimeAppPublic for Grandpa {
    type Public = GrandpaId;
}

/// ImOnline marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImOnline;
impl sp_runtime::BoundToRuntimeAppPublic for ImOnline {
    type Public = ImOnlineId;
}

/// Authority discovery marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthorityDiscovery;
impl sp_runtime::BoundToRuntimeAppPublic for AuthorityDiscovery {
    type Public = AuthorityDiscoveryId;
}

impl_opaque_keys! {
    /// Substrate base runtime keys
    pub struct BasicSessionKeys {
        /// BABE session key
        pub babe: Babe,
        /// GRANDPA session key
        pub grandpa: Grandpa,
        /// ImOnline session key
        pub im_online: ImOnline,
        /// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

impl Session for ChainXRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl XAssets for ChainXRuntime {}
impl XMiningAsset for ChainXRuntime {}
impl XStaking for ChainXRuntime {}

/// ChainX `Client` for ChainX runtime.
pub type ChainXClient = Client<ChainXRuntime>;

/// ChainX `Pair` for ChainX runtime.
pub type ChainXPair = sr25519::Pair;

/// ChainX `PairSigner` for ChainX runtime.
pub type ChainXSigner = PairSigner<ChainXRuntime, ChainXPair>;
