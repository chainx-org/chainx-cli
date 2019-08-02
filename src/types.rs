use base58::ToBase58;
use serde::{Deserialize, Serialize, Serializer};

pub type Hash = substrate_primitives::H256;

/// A public key.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Ed25519Public(pub [u8; 32]);

impl AsRef<[u8; 32]> for Ed25519Public {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Ed25519Public {
    /// A new instance from an H256.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    pub fn from_h256(x: substrate_primitives::H256) -> Self {
        Ed25519Public(x.into())
    }

    /// Return a slice filled with raw data.
    pub fn as_slice(&self) -> &[u8] {
        let r: &[u8; 32] = self.as_ref();
        &r[..]
    }
}

impl Serialize for Ed25519Public {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_ss58check())
    }
}

impl std::fmt::Display for Ed25519Public {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ss58check())
    }
}

impl std::fmt::Debug for Ed25519Public {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.to_ss58check();
        write!(
            f,
            "{} ({}...)",
            substrate_primitives::hexdisplay::HexDisplay::from(&self.0),
            &s[0..8]
        )
    }
}

/// Key that can be encoded to SS58.
pub trait Ss58Codec: Sized {
    /// Return the ss58-check string for this key.
    fn to_ss58check(&self) -> String;
}

impl Ss58Codec for Ed25519Public {
    fn to_ss58check(&self) -> String {
        // The default value for Substrate is 42, but for ChainX main network, it's 44.
        let mut v = vec![44u8];
        v.extend(self.as_slice());
        let r = ss58hash(&v);
        v.extend(&r.as_bytes()[0..2]);
        v.to_base58()
    }
}

const PREFIX: &[u8] = b"SS58PRE";
fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(PREFIX);
    context.update(data);
    context.finalize()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EncodeWrapper(substrate_primitives::storage::StorageKey);

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Chain {
    ChainX,
    Bitcoin,
    Ethereum,
}

impl std::str::FromStr for Chain {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ChainX" | "PCX" => Ok(Chain::Bitcoin),
            "Bitcoin" | "BTC" => Ok(Chain::Bitcoin),
            "Ethereum" | "ETH" => Ok(Chain::Ethereum),
            _ => Err("Unknown Chain Type"),
        }
    }
}

#[derive(Debug)]
pub enum HashOrHeight {
    Height(u64),
    Hash(Hash),
}

impl std::str::FromStr for HashOrHeight {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") || s.starts_with("0X") {
            let hash = s.parse::<Hash>().map_err(|_| "Invalid Hash Length")?;
            return Ok(HashOrHeight::Hash(hash));
        }
        match s.parse::<u64>() {
            Ok(height) => Ok(HashOrHeight::Height(height)),
            Err(_) => Err("The param is neither a 0x-prefix hex hash nor a number"),
        }
    }
}
