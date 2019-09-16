use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Hash(substrate_primitives::H256);

impl Hash {
    pub fn into_inner(self) -> substrate_primitives::H256 {
        self.0
    }
}

impl From<[u8; 32]> for Hash {
    fn from(inner: [u8; 32]) -> Self {
        Self(substrate_primitives::H256::from(inner))
    }
}

impl std::str::FromStr for Hash {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "council" | "Council" | "COUNCIL" => Ok(Hash(council_account())),
            "team" | "Team" | "TEAM" => Ok(Hash(team_account())),
            _ => {
                if s.starts_with("0x") || s.starts_with("0X") {
                    let hash = s[2..]
                        .parse::<substrate_primitives::H256>()
                        .map_err(|_| "Invalid Hash Length")?;
                    Ok(Hash(hash))
                } else {
                    Err("Invalid Hash: 0x-prefix is missing")
                }
            }
        }
    }
}

fn council_account() -> substrate_primitives::H256 {
    "67df26a755e0c31ac81e2ed530d147d7f2b9a3f5a570619048c562b1ed00dfdd"
        .parse()
        .unwrap()
}

fn team_account() -> substrate_primitives::H256 {
    "6193a00c655f836f9d8a62ed407096381f02f8272ea3ea0df0fd66c08c53af81"
        .parse()
        .unwrap()
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

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Token {
    PCX,
    SDOT,
    #[serde(rename = "BTC")]
    XBTC,
    #[serde(rename = "L-BTC")]
    LBTC,
}

impl Token {
    pub fn name(&self) -> Vec<u8> {
        match self {
            Token::PCX => b"PCX".to_vec(),
            Token::SDOT => b"SDOT".to_vec(),
            Token::XBTC => b"BTC".to_vec(),
            Token::LBTC => b"L-BTC".to_vec(),
        }
    }
}

impl std::str::FromStr for Token {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PCX" | "pcx" => Ok(Token::PCX),
            "SDOT" | "S-DOT" | "sdot" | "s-dot" => Ok(Token::SDOT),
            "XBTC" | "X-BTC" | "BTC" | "xbtc" | "x-btc" | "btc" => Ok(Token::XBTC),
            "LBTC" | "L-BTC" | "lbtc" | "l-btc" => Ok(Token::LBTC),
            _ => Err("Unknown Token Type"),
        }
    }
}
