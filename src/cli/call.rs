use structopt::StructOpt;
use web3::futures::Future;

use crate::error::Result;
use crate::rpc::RpcAndCall;
use crate::transport::{http_connect, ws_connect};
use crate::types::{Hash, Token};

#[derive(Debug, StructOpt)]
pub struct CallCommand {
    /// 0x-prefix hex hash string, which represents the private key of the sender
    key: Hash,
    #[structopt(subcommand)]
    call_type: CallCommandType,
}

#[derive(Debug, StructOpt)]
enum CallCommandType {
    /// Transfer free balance.
    #[structopt(name = "transfer")]
    Transfer {
        /// 0x-prefix hex hash string, which represents the receiver
        #[structopt(value_name = "ACCOUNT")]
        to: Hash,
        /// The number of tokens, e.g. 0.1 PCX = 1_000_0000
        #[structopt(value_name = "VALUE")]
        value: u64,
        /// The token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: Token,
        /// Memo
        #[structopt(value_name = "MEMO", default_value = "")]
        memo: String,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Nominate the node.
    #[structopt(name = "nominate")]
    Nominate {
        /// 0x-prefix hex hash string, which represents the node address
        #[structopt(value_name = "NODE")]
        target: Hash,
        /// The number of tokens, e.g. 0.1 PCX = 1_000_0000
        #[structopt(value_name = "VALUE")]
        value: u64,
        /// Memo
        #[structopt(value_name = "MEMO", default_value = "")]
        memo: String,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Unnominate the node.
    #[structopt(name = "unnominate")]
    Unnominate {
        /// 0x-prefix hex hash string, which represents the node address
        #[structopt(value_name = "NODE")]
        target: Hash,
        /// The number of tokens, e.g. 0.1 PCX = 1_000_0000
        #[structopt(value_name = "VALUE")]
        value: u64,
        /// Memo
        #[structopt(value_name = "MEMO", default_value = "")]
        memo: String,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Re-nominate the node.
    #[structopt(name = "renominate")]
    Renominate {
        /// 0x-prefix hex hash string, which represents the node address that will decrease the vote
        #[structopt(value_name = "FROM")]
        from: Hash,
        /// 0x-prefix hex hash string, which represents the node address that will increase the vote
        #[structopt(value_name = "TO")]
        to: Hash,
        /// The number of tokens, e.g. 0.1 PCX = 1_000_0000
        #[structopt(value_name = "VALUE")]
        value: u64,
        /// Memo
        #[structopt(value_name = "MEMO", default_value = "")]
        memo: String,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Unfreeze the redeemed PCX.
    #[structopt(name = "unfreeze")]
    Unfreeze {
        /// 0x-prefix hex hash string, which represents the node address
        #[structopt(value_name = "NODE")]
        target: Hash,
        /// The index of unfreeze transaction
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Vote claim.
    #[structopt(name = "vote-claim")]
    VoteClaim {
        /// 0x-prefix hex hash string, which represents the node address
        #[structopt(value_name = "NODE")]
        target: Hash,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Deposit claim.
    #[structopt(name = "deposit-claim")]
    DepositClaim {
        /// The token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: Token,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
}

impl CallCommand {
    /// Dispatch call subcommand
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

    /// Dispatch call subcommand implement
    #[rustfmt::skip]
    fn dispatch_impl<RC: RpcAndCall>(self, rc: RC) -> Result<()> {
        use CallCommandType::*;
        let key = self.key;
        let fut = match self.call_type {
            Transfer { to, value, token, memo, acc } => rc.transfer(key, to, value, token, memo, acc),
            Nominate { target, value, memo, acc } => rc.nominate(key, target, value, memo, acc),
            Unnominate { target, value, memo, acc } => rc.unnominate(key, target, value, memo, acc),
            Renominate { from, to, value, memo, acc } => rc.renominate(key, from, to, value, memo, acc),
            Unfreeze { target, index, acc } => rc.unfreeze(key, target, index, acc),
            VoteClaim { target, acc } => rc.vote_claim(key, target, acc),
            DepositClaim { token, acc } => rc.deposit_claim(key, token, acc),
        };
        let response = fut.wait()?;
        let response = serde_json::to_string_pretty(&response)?;
        println!("{}", response);
        Ok(())
    }
}
