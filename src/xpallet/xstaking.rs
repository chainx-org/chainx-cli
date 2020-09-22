use std::marker::PhantomData;

use codec::Encode;
use substrate_subxt::{
    balances::{Balances, BalancesEventsDecoder},
    module,
    system::{System, SystemEventsDecoder},
    Call, Store,
};

use crate::primitives::BlockNumber;

#[module]
pub trait XStaking: Balances + System {}

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RegisterCall<T: XStaking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Validator nickname
    pub validator_nickname: Vec<u8>,
    /// Initial bond to this validator.
    pub initial_bond: <T as Balances>::Balance,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct BondCall<'a, T: XStaking> {
    /// Destination of the bond.
    pub target: &'a <T as System>::Address,
    /// Amount to bond.
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RebondCall<'a, T: XStaking> {
    /// Source of the rebond.
    pub from: &'a <T as System>::Address,
    /// Target of the rebond.
    pub to: &'a <T as System>::Address,
    /// Amount to rebond.
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct UnbondCall<'a, T: XStaking> {
    /// Source of the rebond.
    pub target: &'a <T as System>::Address,
    /// Amount to unbond.
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ClaimCall<'a, T: XStaking> {
    /// Target of the claim.
    pub target: &'a <T as System>::Address,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ValidateCall<T: XStaking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ChillCall<T: XStaking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SetValidatorCountCall<T: XStaking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// New ideal validator count.
    #[codec(compact)]
    pub new: u32,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SetSessionsPerEraCall<T: XStaking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// New ideal validator count.
    #[codec(compact)]
    pub new: BlockNumber,
}

pub type ReferralId = Vec<u8>;

/// Profile of staking validator.
///
/// These fields are static or updated less frequently.
#[derive(PartialEq, Eq, Clone, Default, codec::Encode, codec::Decode)]
pub struct ValidatorProfile {
    /// Block number at which point it's registered on chain.
    pub registered_at: BlockNumber,
    /// Validator is chilled right now.
    ///
    /// Declared no desire to be a validator or forced to be chilled due to `MinimumCandidateThreshold`.
    pub is_chilled: bool,
    /// Block number of last performed `chill` operation.
    pub last_chilled: Option<BlockNumber>,
    /// Referral identity that belongs to the validator.
    pub referral_id: ReferralId,
}

impl std::fmt::Debug for ValidatorProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ValidatorProfile")
            .field("registered_at", &self.registered_at)
            .field("is_chilled", &self.is_chilled)
            .field("last_chilled", &self.last_chilled)
            .field("referral_id", &String::from_utf8_lossy(&self.referral_id))
            .finish()
    }
}

/// Account field of the `System` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, codec::Encode)]
pub struct ValidatorsStore<'a, T: System> {
    #[store(returns = ValidatorProfile)]
    /// Account to retrieve the `ValidatorProfile` for.
    pub account_id: &'a T::AccountId,
}
