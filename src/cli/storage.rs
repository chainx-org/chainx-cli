use structopt::StructOpt;
use web3::futures::Future;

use crate::error::Result;
use crate::rpc::StorageRpc;
use crate::transport::{http_connect, ws_connect};
use crate::types::Hash;

#[derive(Debug, StructOpt)]
pub enum StorageCommand {
    /// Get AccountNonce storage.
    #[structopt(name = "account-nonce")]
    AccountNonce {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get MultiSig AddrInfo storage.
    #[structopt(name = "multisig-addrinfo")]
    MultiSigAddrInfo {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT", default_value = "council")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get MultiSig PendingListFor storage.
    #[structopt(name = "multisig-pendinglist")]
    MultiSigPendingList {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT", default_value = "council")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get MultiSig PendingStateFor storage.
    #[structopt(name = "multisig-pendingstate")]
    MultiSigPendingState {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex hash string for pending multisig transaction
        #[structopt(value_name = "HASH")]
        pending_hash: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
}

impl StorageCommand {
    /// Dispatch storage subcommand
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

    /// Dispatch storage subcommand implement
    fn dispatch_impl<R: StorageRpc>(self, rpc: R) -> Result<()> {
        use StorageCommand::*;
        let fut = match self {
            AccountNonce { who, hash } => {
                let response = rpc.account_nonce(who, hash).wait()?;
                println!("{}", response);
                return Ok(());
            }
            MultiSigAddrInfo { who, hash } => rpc.multisig_addr_info(who, hash),
            MultiSigPendingList { who, hash } => rpc.multisig_pending_list(who, hash),
            MultiSigPendingState {
                who,
                pending_hash,
                hash,
            } => {
                let response = rpc.multisig_pending_state(who, pending_hash, hash).wait()?;
                println!("{:?}", response);
                return Ok(());
            }
        };
        let response = fut.wait()?;
        let response = serde_json::to_string_pretty(&response)?;
        println!("{}", response);
        Ok(())
    }
}
