use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::{AccountId, BlockNumber, Hash};

use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::{Chain, TradingPairIndex};
use crate::util;

pub trait ChainXRpc {
    fn block_by_number(&self, number: Option<u64>) -> BoxFuture<Value>;

    fn next_renominate(&self, who: AccountId, hash: Option<Hash>)
        -> BoxFuture<Option<BlockNumber>>;

    fn assets_by_account(
        &self,
        who: AccountId,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value>;

    fn assets(&self, page_index: u32, page_size: u32, hash: Option<Hash>) -> BoxFuture<Value>;

    fn verify_addr(
        &self,
        token: String,
        addr: String,
        memo: String,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<bool>>;

    fn withdraw_limit(&self, token: String, hash: Option<Hash>) -> BoxFuture<Value>;

    fn deposit_limit(&self, token: String, hash: Option<Hash>) -> BoxFuture<Value>;

    fn withdraw_list(
        &self,
        chain: Chain,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value>;

    fn deposit_list(
        &self,
        chain: Chain,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value>;

    fn nomination_records(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value>;

    fn psedu_nomination_records(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value>;

    fn intention(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value>;

    fn intentions(&self, hash: Option<Hash>) -> BoxFuture<Value>;

    fn psedu_intentions(&self, hash: Option<Hash>) -> BoxFuture<Value>;

    fn trading_pairs(&self, hash: Option<Hash>) -> BoxFuture<Value>;

    fn quotations(&self, id: TradingPairIndex, piece: u32, hash: Option<Hash>) -> BoxFuture<Value>;

    fn orders(
        &self,
        who: AccountId,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value>;

    fn address_by_account(
        &self,
        who: AccountId,
        chain: Chain,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<Vec<String>>>;

    fn trustee_session_info(
        &self,
        chain: Chain,
        number: Option<u32>,
        hash: Option<Hash>,
    ) -> BoxFuture<Value>;

    fn trustee_by_account(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value>;

    fn call_fee(&self, call_params: String, tx_len: u64, hash: Option<Hash>) -> BoxFuture<Value>;

    fn withdraw_tx(&self, hash: Option<Hash>) -> BoxFuture<Value>;

    fn mock_bitcoin_new_trustees(
        &self,
        candidates: Vec<AccountId>,
        hash: Option<Hash>,
    ) -> BoxFuture<Value>;

    fn particular_accounts(&self, hash: Option<Hash>) -> BoxFuture<Value>;
}

impl<T: BatchTransport + 'static> ChainXRpc for ChainXTransport<T> {
    fn block_by_number(&self, number: Option<u64>) -> BoxFuture<Value> {
        self.execute("chainx_getBlockByNumber", vec![util::serialize(number)])
    }

    fn next_renominate(
        &self,
        who: AccountId,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<BlockNumber>> {
        self.execute(
            "chainx_getNextRenominateByAccount",
            vec![util::serialize(who), util::serialize(hash)],
        )
        .and_then(util::deserialize)
    }

    fn assets_by_account(
        &self,
        who: AccountId,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value> {
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

    fn assets(&self, page_index: u32, page_size: u32, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getAssets",
            vec![
                util::serialize(page_index),
                util::serialize(page_size),
                util::serialize(hash),
            ],
        )
    }

    fn verify_addr(
        &self,
        token: String,
        addr: String,
        memo: String,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<bool>> {
        self.execute(
            "chainx_verifyAddressValidity",
            vec![
                util::serialize(token),
                util::serialize(addr),
                util::serialize(memo),
                util::serialize(hash),
            ],
        )
        .and_then(util::deserialize)
    }

    fn withdraw_limit(&self, token: String, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getWithdrawalLimitByToken",
            vec![util::serialize(token), util::serialize(hash)],
        )
    }

    fn deposit_limit(&self, token: String, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getDepositLimitByToken",
            vec![util::serialize(token), util::serialize(hash)],
        )
    }

    fn withdraw_list(
        &self,
        chain: Chain,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value> {
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

    fn deposit_list(
        &self,
        chain: Chain,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value> {
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

    fn nomination_records(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getNominationRecords",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    fn psedu_nomination_records(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getPseduNominationRecords",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    fn intention(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getIntentionByAccount",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    fn intentions(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chainx_getIntentions", vec![util::serialize(hash)])
    }

    fn psedu_intentions(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chainx_getPseduIntentions", vec![util::serialize(hash)])
    }

    fn trading_pairs(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chainx_getTradingPairs", vec![util::serialize(hash)])
    }

    fn quotations(&self, id: TradingPairIndex, piece: u32, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getQuotations",
            vec![
                util::serialize(id),
                util::serialize(piece),
                util::serialize(hash),
            ],
        )
    }

    fn orders(
        &self,
        who: AccountId,
        page_index: u32,
        page_size: u32,
        hash: Option<Hash>,
    ) -> BoxFuture<Value> {
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

    fn address_by_account(
        &self,
        who: AccountId,
        chain: Chain,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<Vec<String>>> {
        self.execute(
            "chainx_getAddressByAccount",
            vec![
                util::serialize(who),
                util::serialize(chain),
                util::serialize(hash),
            ],
        )
        .and_then(util::deserialize)
    }

    fn trustee_session_info(
        &self,
        chain: Chain,
        number: Option<u32>,
        hash: Option<Hash>,
    ) -> BoxFuture<Value> {
        self.execute(
            "chainx_getTrusteeSessionInfo",
            vec![
                util::serialize(chain),
                util::serialize(number),
                util::serialize(hash),
            ],
        )
    }

    fn trustee_by_account(&self, who: AccountId, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getTrusteeInfoByAccount",
            vec![util::serialize(who), util::serialize(hash)],
        )
    }

    fn call_fee(&self, call_params: String, tx_len: u64, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute(
            "chainx_getFeeByCallAndLength",
            vec![
                util::serialize(call_params),
                util::serialize(tx_len),
                util::serialize(hash),
            ],
        )
    }

    fn withdraw_tx(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chainx_getWithdrawTx", vec![util::serialize(hash)])
    }

    fn mock_bitcoin_new_trustees(
        &self,
        candidates: Vec<AccountId>,
        hash: Option<Hash>,
    ) -> BoxFuture<Value> {
        self.execute(
            "chainx_getMockBitcoinNewTrustees",
            vec![util::serialize(candidates), util::serialize(hash)],
        )
    }

    fn particular_accounts(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chainx_particularAccounts", vec![util::serialize(hash)])
    }
}
