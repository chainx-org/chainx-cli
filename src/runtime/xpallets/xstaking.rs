use std::{
    collections::BTreeMap,
    fmt::{self, Debug},
    marker::PhantomData,
};

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use subxt::{
    balances::{Balances},
    module,
    system::{System},
    Call, Store,
};

#[module]
pub trait XStaking: Balances + System {}

// ============================================================================
// Call
// ============================================================================

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RegisterCall<T: XStaking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Validator nickname
    pub validator_nickname: Vec<u8>,
    /// Initial bond to this validator.
    #[codec(compact)]
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
    pub new: SessionIndex,
}

/// Simple index type with which we can count sessions.
pub type SessionIndex = u32;

// ============================================================================
// Storage
// ============================================================================

/// Validators field of the `XStaking` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ValidatorsStore<'a, T: XStaking> {
    #[store(returns = ValidatorProfile<T::BlockNumber>)]
    pub account_id: &'a T::AccountId,
}

/// ValidatorLedgers field of the `XStaking` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ValidatorLedgersStore<'a, T: XStaking> {
    #[store(returns = ValidatorLedger<BalanceOf<T>, VoteWeight, T::BlockNumber>)]
    pub account_id: &'a T::AccountId,
}

/// Nominations field of the `XStaking` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct NominationsStore<'a, T: XStaking> {
    #[store(returns = NominatorLedger<BalanceOf<T>, VoteWeight, T::BlockNumber>)]
    pub nominator: &'a T::AccountId,
    pub nominatee: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct LocksStore<'a, T: XStaking> {
    #[store(returns = BTreeMap<LockedType, BalanceOf<T>>)]
    pub staker: &'a T::AccountId,
}

pub type BalanceOf<T> = <T as Balances>::Balance;

pub type VoteWeight = u128;

pub type ReferralId = Vec<u8>;

/// Profile of staking validator.
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct ValidatorProfile<BlockNumber> {
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

impl<BlockNumber: Debug> Debug for ValidatorProfile<BlockNumber> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ValidatorProfile")
            .field("registered_at", &self.registered_at)
            .field("is_chilled", &self.is_chilled)
            .field("last_chilled", &self.last_chilled)
            .field("referral_id", &String::from_utf8_lossy(&self.referral_id))
            .finish()
    }
}

/// Vote weight properties of validator.
#[derive(PartialEq, Eq, Clone, Default, Debug, Encode, Decode)]
pub struct ValidatorLedger<Balance, VoteWeight, BlockNumber> {
    /// The total amount of all the nominators' vote balances.
    pub total_nomination: Balance,
    /// Last calculated total vote weight of current validator.
    pub last_total_vote_weight: VoteWeight,
    /// Block number at which point `last_total_vote_weight` just updated.
    pub last_total_vote_weight_update: BlockNumber,
}

/// Vote weight properties of nominator.
#[derive(PartialEq, Eq, Clone, Default, Debug, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NominatorLedger<Balance, VoteWeight, BlockNumber> {
    /// The amount of vote.
    pub nomination: Balance,
    /// Last calculated total vote weight of current nominator.
    pub last_vote_weight: VoteWeight,
    /// Block number at which point `last_vote_weight` just updated.
    pub last_vote_weight_update: BlockNumber,
    /// Unbonded entries.
    pub unbonded_chunks: Vec<Unbonded<Balance, BlockNumber>>,
}

/// Type for noting when the unbonded fund can be withdrawn.
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unbonded<Balance, BlockNumber> {
    /// Amount of funds to be unlocked.
    pub value: Balance,
    /// Block number at which point it'll be unlocked.
    pub locked_until: BlockNumber,
}

/// Detailed types of reserved balances in Staking.
#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Encode, Decode, Debug)]
pub enum LockedType {
    /// Locked balances when nominator calls `bond`.
    Bonded,
    /// The locked balances transition from `Bonded` into `BondedWithdrawal` state
    /// when nominator calls `unbond`.
    BondedWithdrawal,
}
