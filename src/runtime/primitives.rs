use sp_core::H256;
use sp_runtime::{
    traits::BlakeTwo256,
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

/// A hash of some data used by the chain.
pub type Hash = H256;

/// Hashing algorithm
pub type Hashing = BlakeTwo256;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, AccountIndex>;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// Asset ID.
pub type AssetId = u32;
