use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::{AccountId, Hash};

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::types::{Chain, TradingPairIndex};
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn block_by_number(&self, number: Option<u64>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chainx_getBlockByNumber", vec![util::serialize(number)])
    }

    pub fn next_renominate(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getNextRenominateByAccount",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    pub fn assets_by_account(
        &self,
        who: AccountId,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getAssetsByAccount",
            vec![
                util::serialize(who),
                util::serialize(page_index),
                util::serialize(page_size),
                util::serialize(hash),
            ],
        )
    }

    pub fn assets(
        &self,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getAssets",
            vec![
                util::serialize(page_index),
                util::serialize(page_size),
                util::serialize(hash),
            ],
        )
    }

    pub fn verify_addr(
        &self,
        token: String,
        addr: String,
        memo: String,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_verifyAddressValidity",
            vec![
                util::serialize(token),
                util::serialize(addr),
                util::serialize(memo),
                util::serialize(hash),
            ],
        )
    }

    pub fn withdraw_limit(
        &self,
        token: String,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getWithdrawalLimitByToken",
            vec![util::serialize(token), util::serialize(hash)],
        )
    }

    pub fn deposit_limit(
        &self,
        token: String,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getDepositLimitByToken",
            vec![util::serialize(token), util::serialize(hash)],
        )
    }

    pub fn withdraw_list(
        &self,
        chain: Chain,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getWithdrawalList",
            vec![
                util::serialize(chain),
                util::serialize(page_index),
                util::serialize(page_size),
                util::serialize(hash),
            ],
        )
    }

    pub fn deposit_list(
        &self,
        chain: Chain,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getDepositList",
            vec![
                util::serialize(chain),
                util::serialize(page_index),
                util::serialize(page_size),
                util::serialize(hash),
            ],
        )
    }

    pub fn nomination_records(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getNominationRecords",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    pub fn psedu_nomination_records(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getPseduNominationRecords",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    pub fn intention(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getIntentionByAccount",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    pub fn intentions(&self, hash: Option<Hash>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chainx_getIntentions", vec![util::serialize(hash)])
    }

    pub fn psedu_intentions(&self, hash: Option<Hash>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chainx_getPseduIntentions", vec![util::serialize(hash)])
    }

    pub fn trading_pairs(&self, hash: Option<Hash>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chainx_getTradingPairs", vec![util::serialize(hash)])
    }

    pub fn quotations(
        &self,
        id: TradingPairIndex,
        piece: u32,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getQuotations",
            vec![
                util::serialize(id),
                util::serialize(piece),
                util::serialize(hash),
            ],
        )
    }

    pub fn orders(
        &self,
        who: AccountId,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getOrders",
            vec![
                util::serialize(who),
                util::serialize(page_index),
                util::serialize(page_size),
                util::serialize(hash),
            ],
        )
    }

    pub fn address_by_account(
        &self,
        who: AccountId,
        chain: Chain,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getAddressByAccount",
            vec![
                util::serialize(who),
                util::serialize(chain),
                util::serialize(hash),
            ],
        )
    }

    pub fn trustee_session_info(
        &self,
        chain: Chain,
        number: Option<u32>,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getTrusteeSessionInfo",
            vec![
                util::serialize(chain),
                util::serialize(number),
                util::serialize(hash),
            ],
        )
    }

    pub fn trustee_by_account(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getTrusteeInfoByAccount",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    pub fn call_fee(
        &self,
        call_params: String,
        tx_length: u64,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getFeeByCallAndLength",
            vec![
                util::serialize(call_params),
                util::serialize(tx_length),
                util::serialize(hash),
            ],
        )
    }

    pub fn withdraw_tx(&self, hash: Option<Hash>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chainx_getWithdrawTx", vec![util::serialize(hash)])
    }

    pub fn mock_bitcoin_new_trustees(
        &self,
        candidates: Vec<AccountId>,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute(
            "chainx_getMockBitcoinNewTrustees",
            vec![util::serialize(candidates), util::serialize(hash)],
        )
    }

    pub fn particular_accounts(
        &self,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute("chainx_particularAccounts", vec![util::serialize(hash)])
    }
}
