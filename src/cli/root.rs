use structopt::clap;
use structopt::StructOpt;

use crate::error::Result;
use crate::rpc::RpcAndCall;
use crate::transport::{http_connect, ws_connect};
use crate::types::{Hash, Token};

#[derive(Debug, StructOpt)]
pub struct RootCommand {
    /// 0x-prefix hex hash string, which represents the private key of the sender
    key: Hash,
    #[structopt(subcommand)]
    root_type: RootCommandType,
}

#[derive(Debug, StructOpt)]
enum RootCommandType {
    /// Confirm a pending proposal given its id.
    #[structopt(name = "confirm")]
    Confirm {
        /// 0x-prefix hex hash string, which represents the ID of a pending proposal
        #[structopt(value_name = "ID")]
        id: Hash,
        /// The acceleration speed of transaction packaging.
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },

    /// Initiate a proposal on a multisig account.
    #[structopt(name = "propose")]
    #[structopt(setting = clap::AppSettings::DisableHelpSubcommand)]
    Propose(ProposeCommand),
}

#[derive(Debug, StructOpt)]
enum ProposeCommand {
    /// Set nomination record.
    #[structopt(name = "nomination-record")]
    NominationRecord {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// The name of intention
        #[structopt(value_name = "NAME")]
        name: Option<String>,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Set deposit record.
    #[structopt(name = "deposit-record")]
    DepositRecord {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: Token,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Set claim restriction.
    #[structopt(name = "claim-restriction")]
    ClaimRestriction {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: Token,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
    /// Set psedu intention profs.
    #[structopt(name = "psedu-intention-profs")]
    PseduIntentionProfs {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "BTC")]
        token: Token,
        /// The acceleration speed of transaction packaging
        #[structopt(value_name = "ACCELERATION", default_value = "1")]
        acc: u32,
    },
}

impl RootCommand {
    /// Dispatch root subcommand
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

    /// Dispatch root subcommand implement
    fn dispatch_impl<RC: RpcAndCall>(self, _rc: RC) -> Result<()> {
        use ProposeCommand::*;
        use RootCommandType::*;
        let _key = self.key;
        match self.root_type {
            Confirm { .. } => unimplemented!(),
            Propose(propose) => match propose {
                NominationRecord { .. } => unimplemented!(),
                DepositRecord { .. } => unimplemented!(),
                ClaimRestriction { .. } => unimplemented!(),
                PseduIntentionProfs { .. } => unimplemented!(),
            },
        }
    }
}
