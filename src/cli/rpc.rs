use structopt::StructOpt;
use web3::futures::Future;

use crate::error::Result;
use crate::rpc::Rpc;
use crate::transport::{http_connect, ws_connect};
use crate::types::{Chain, Hash, HashOrHeight};

#[derive(Debug, StructOpt)]
pub enum RpcCommand {
    // Chain RPC
    // ========================================================================
    /// Get header of a relay chain block.
    #[structopt(name = "header")]
    Header {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
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
        #[structopt(value_name = "HEIGHT/HASH")]
        hash_or_height: Option<HashOrHeight>,
    },

    // State Rpc
    // ========================================================================
    /// Get the runtime version.
    #[structopt(name = "runtime_version")]
    RuntimeVersion {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
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
    /// Get the block height of the account's next switchable vote.
    #[structopt(name = "next_renominate")]
    NextRenominate {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the asset information of the account.
    #[structopt(name = "asset")]
    Asset {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "10")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the assets information.
    #[structopt(name = "assets")]
    Assets {
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "10")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the binding BTC address of the account.
    #[structopt(name = "addr_by_account")]
    AddrByAccount {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Verify the correctness of the withdrawal address.
    #[structopt(name = "addr_verification")]
    AddrVerification {
        ///  Withdrawal address that needs to be verified
        #[structopt(value_name = "ADDR")]
        addr: String,
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: String,
        /// Memo
        #[structopt(value_name = "MEMO", default_value = "")]
        memo: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the limitation related to withdrawals.
    #[structopt(name = "withdraw_limit")]
    WithdrawLimit {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the limitation related to deposits.
    #[structopt(name = "deposit_limit")]
    DepositLimit {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get all current withdrawal records.
    #[structopt(name = "withdraw_list")]
    WithdrawList {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "10")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get all current deposit records.
    #[structopt(name = "deposit_list")]
    DepositList {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "10")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the staking dividend of the account.
    #[structopt(name = "staking_dividend")]
    StakingDividend {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the cross mining dividend of the account.
    #[structopt(name = "cross_mining_dividend")]
    CrossMiningDividend {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the nominate records of the account.
    #[structopt(name = "nomination_records")]
    NominationRecords {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// For getting the nominate records version1 of the account.
        #[structopt(value_name = "VERSION", default_value = "0")]
        version: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the voting information of the account.
    #[structopt(name = "psedu_nomination_records")]
    PseduNominationRecords {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// For getting the voting information version1 of the account.
        #[structopt(value_name = "VERSION", default_value = "0")]
        version: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the information of the node address.
    #[structopt(name = "intention")]
    Intention {
        /// 0x-prefix hex hash string for node address
        #[structopt(value_name = "ACCOUNT")]
        addr: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the all node information.
    #[structopt(name = "intentions")]
    Intentions {
        /// For getting the all node information version1 of the account.
        #[structopt(value_name = "VERSION", default_value = "0")]
        version: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the mining list.
    #[structopt(name = "psedu_intentions")]
    PseduIntentions {
        /// For getting the mining list version1 of the account.
        #[structopt(value_name = "VERSION", default_value = "0")]
        version: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the trading pairs list.
    #[structopt(name = "trading_pairs")]
    TradingPairs {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the trading quotations list.
    #[structopt(name = "quotations")]
    Quotations {
        /// Trading pair index
        #[structopt(value_name = "INDEX")]
        id: u32,
        /// Piece (must <= 10)
        #[structopt(value_name = "PIECE")]
        piece: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the pending orders list of the account.
    #[structopt(name = "orders")]
    Orders {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "10")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the current trustee information of the chain.
    #[structopt(name = "trustee_session")]
    TrusteeSession {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// Trustee session era, [0, latest) [default: latest trustee session era]
        #[structopt(value_name = "ERA")]
        era: Option<u32>,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the trustee information of the account.
    #[structopt(name = "trustee_info")]
    TrusteeInfo {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the withdrawal transactions of the chain.
    #[structopt(name = "withdraw_tx")]
    WithdrawTx {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Simulate the generation of next era BTC trustee address.
    #[structopt(name = "mock_btc_new_trustees")]
    MockBtcNewTrustees {
        /// 0x-prefix hex hash string for new trustee accounts
        #[structopt(value_name = "ACCOUNTS")]
        candidates: Vec<Hash>,
    },
    /// Get the fee according to the call and transaction length.
    #[structopt(name = "call_fee")]
    CallFee {
        /// The parameters of Call
        #[structopt(value_name = "PARAMS")]
        call: String,
        /// The length of transaction
        tx_len: u64,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the fee weight map, transaction base fee and transaction byte fee.
    #[structopt(name = "call_fee_map")]
    CallFeeMap {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get the particular account addresses (council, team, trustees).
    #[structopt(name = "particular_accounts")]
    ParticularAccounts {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
}

impl RpcCommand {
    /// Dispatch rpc subcommand
    pub fn dispatch(self, url: &str) -> Result<()> {
        if url.starts_with("ws://") || url.starts_with("wss://") {
            let (_handle, chainx) = ws_connect(url)?;
            self.dispatch_impl(chainx)?;
        } else {
            let (_handle, chainx) = http_connect(url)?;
            self.dispatch_impl(chainx)?;
        }
        Ok(())
    }

    /// Dispatch rpc subcommand implement
    #[rustfmt::skip]
    fn dispatch_impl<R: Rpc>(self, rpc: R) -> Result<()> {
        use RpcCommand::*;
        let fut = match self {
            // Chain Rpc
            Header { hash } => rpc.header(hash),
            FinalizedHeader => rpc.finalized_head(),
            BlockHash { height } => rpc.block_hash(height),
            Block { hash_or_height } => match hash_or_height {
                Some(HashOrHeight::Height(number)) => rpc.block_by_number(Some(number)),
                Some(HashOrHeight::Hash(hash)) => rpc.block_by_hash(Some(hash)),
                None => rpc.block_by_hash(None),
            }

            // State Rpc
            RuntimeVersion { hash } => rpc.runtime_version(hash),

            // System Rpc
            SystemName => rpc.system_name(),
            SystemVersion => rpc.system_version(),
            SystemChain => rpc.system_chain(),
            SystemProperties => rpc.system_properties(),
            SystemHealth => rpc.system_health(),
            SystemPeers => rpc.system_peers(),
            SystemNetworkState => rpc.system_network_state(),

            // ChainX Rpc
            NextRenominate { who, hash } => rpc.next_renominate(who, hash),
            Asset { who, index, size, hash } => rpc.asset(who, index, size, hash),
            Assets { index, size, hash } => rpc.assets(index, size, hash),
            AddrByAccount { who, chain, hash } => rpc.addr_by_account(who, chain, hash),
            AddrVerification { token, addr, memo, hash} => rpc.verify_addr(token, addr, memo, hash),
            WithdrawLimit { token, hash } => rpc.withdraw_limit(token, hash),
            DepositLimit { token, hash } => rpc.deposit_limit(token, hash),
            WithdrawList { chain, index, size, hash} => rpc.withdraw_list(chain, index, size, hash),
            DepositList { chain, index, size, hash } => rpc.deposit_list(chain, index, size, hash),
            StakingDividend { who, hash } => rpc.staking_dividend(who, hash),
            CrossMiningDividend { who, hash } => rpc.cross_mining_dividend(who, hash),
            NominationRecords { who, version, hash } => rpc.nomination_records(who, version, hash),
            PseduNominationRecords { who, version, hash } => rpc.psedu_nomination_records(who, version, hash),
            Intention { addr, hash } => rpc.intention(addr, hash),
            Intentions { version, hash } => rpc.intentions(version, hash),
            PseduIntentions { version, hash } => rpc.psedu_intentions(version, hash),
            TradingPairs { hash } => rpc.trading_pairs(hash),
            Quotations { id, piece, hash } => rpc.quotations(id, piece, hash),
            Orders { who, index, size, hash} => rpc.orders(who, index, size, hash),
            TrusteeSession { chain, era, hash } => rpc.trustee_session_info(chain, era, hash),
            TrusteeInfo { who, hash } => rpc.trustee_by_account(who, hash),
            WithdrawTx { chain, hash } => rpc.withdraw_tx(chain, hash),
            MockBtcNewTrustees { candidates } => rpc.mock_btc_new_trustees(candidates),
            CallFee { call, tx_len, hash } => rpc.call_fee(call, tx_len, hash),
            CallFeeMap { hash } => rpc.call_fee_map(hash),
            ParticularAccounts { hash } => rpc.particular_accounts(hash),
        };
        let response = fut.wait()?;
        let response = serde_json::to_string_pretty(&response)?;
        println!("{}", response);
        Ok(())
    }
}
