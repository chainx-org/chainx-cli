use structopt::StructOpt;

use crate::types::HeightOrHash;

#[derive(Debug, StructOpt)]
pub enum RpcCommand {
    // Chain RPC
    // ========================================================================
    /// Get header of a relay chain block.
    #[structopt(name = "header")]
    Header {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    /// Get hash of the last finalized block in the canon chain.
    #[structopt(name = "header_finalized")]
    FinalizedHeader,
    /// Get hash of the n-th block in the canon chain.
    #[structopt(name = "block_hash")]
    BlockHash {
        /// Block height [default: latest block height]
        #[structopt(value_name = "NUM")]
        height: Option<u64>,
    },
    /// Get header and body of a relay chain block.
    #[structopt(name = "block")]
    Block {
        /// 0x-prefix hex block hash string or block height [default: hash or height of the latest block]
        #[structopt(value_name = "HASH/HEIGHT")]
        hash_or_height: Option<HeightOrHash>,
    },

    // System Rpc
    // ========================================================================
    /// Get the node's implementation name. Plain old string.
    #[structopt(name = "system_name")]
    SystemName,
    /// Get the node implementation's version. Should be a semver string.
    #[structopt(name = "system_version")]
    SystemVersion,
    /// Get the chain's type. Given as a string identifier.
    #[structopt(name = "system_chain")]
    SystemChain,
    /// Get a custom set of properties as a JSON object, defined in the chain spec.
    #[structopt(name = "system_properties")]
    SystemProperties,
    /// Return health status of the node.
    #[structopt(name = "system_health")]
    SystemHealth,
    /// Returns currently connected peers.
    #[structopt(name = "system_peers")]
    SystemPeers,
    /// Returns current state of the network.
    #[structopt(name = "system_network_state")]
    SystemNetworkState,

    // ChainX Rpc
    // ========================================================================
    #[structopt(name = "next_renominate")]
    NextRenominate {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "asset")]
    Asset {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "assets")]
    Assets {
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "verify_addr")]
    VerifyAddr {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: String,
        /// Address
        #[structopt(value_name = "ADDR")]
        addr: String,
        /// Memo
        #[structopt(value_name = "MEMO")]
        memo: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "withdraw_limit")]
    WithdrawLimit {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "deposit_limit")]
    DepositLimit {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "withdraw_list")]
    WithdrawList {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: String,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "deposit_list")]
    DepositList {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: String,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "nomination_records")]
    NominationRecords {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "psedu_nomination_records")]
    PseduNominationRecords {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "intention")]
    Intention {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "intentions")]
    Intentions {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "psedu_intentions")]
    PseduIntentions {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "trading_pairs")]
    TradingPairs {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "quotations")]
    Quotations {
        /// Trading pair index
        #[structopt(value_name = "INDEX")]
        id: u32,
        #[structopt(value_name = "HASH")]
        piece: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "orders")]
    Orders {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "addr_by_account")]
    AddrByAccount {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "trustee_session")]
    TrusteeSession {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: String,
        /// Trustee session number, [0, latest) [default: latest trustee session number]
        #[structopt(value_name = "NUM")]
        number: Option<u32>,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "trustee_info")]
    TrusteeInfo {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "call_fee")]
    CallFee {
        /// The parameters of Call
        #[structopt(value_name = "PARAMS")]
        call_params: String,
        /// The length of transaction
        tx_length: u64,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "withdraw_tx")]
    WithdrawTx {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "mock_btc_new_trustees")]
    MockBitcoinNewTrustees {
        /// 0x-prefix hex hash string for new trustee accounts
        #[structopt(value_name = "ACCOUNTS")]
        candidates: Vec<String>,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
    #[structopt(name = "particular_accounts")]
    ParticularAccounts {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<String>,
    },
}
