use serde_json::Value;

use crate::error::Result;
use crate::transport::ChainXTransport;
use crate::types::{Chain, Hash, Token};

impl_rpc! {
    pub async trait ChainXRpc for ChainXTransport<T> {
        "chainx_getBlockByNumber" => fn block_by_number(&self, number: Option<u64>) -> Result<Value>;

        "chainx_getExtrinsicsEventsByBlockHash" => fn extrinsic_events(&self, hash: Option<Hash>) -> Result<Value>;

        "chainx_getNextRenominateByAccount" => fn next_renominate(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;

        "chainx_getAssetsByAccount" => fn asset(&self, who: Hash, page_index: u32, page_size: u32, hash: Option<Hash>) -> Result<Value>;
        "chainx_getAssets" => fn assets(&self, page_index: u32, page_size: u32, hash: Option<Hash>) -> Result<Value>;

        "chainx_getAddressByAccount" => fn addr_by_account(&self, who: Hash, chain: Chain, hash: Option<Hash>) -> Result<Value>;
        "chainx_verifyAddressValidity" => fn verify_addr(&self, token: Token, addr: String, memo: String, hash: Option<Hash>) -> Result<Value>;

        "chainx_getWithdrawalLimitByToken" => fn withdraw_limit(&self, token: Token, hash: Option<Hash>) -> Result<Value>;
        "chainx_getDepositLimitByToken" => fn deposit_limit(&self, token: Token, hash: Option<Hash>) -> Result<Value>;

        "chainx_getWithdrawalList" => fn withdraw_list(&self, chain: Chain, page_index: u32, page_size: u32, hash: Option<Hash>) -> Result<Value>;
        "chainx_getDepositList" => fn deposit_list(&self, chain: Chain, page_index: u32, page_size: u32, hash: Option<Hash>) -> Result<Value>;

        "chainx_getStakingDividendByAccount" => fn staking_dividend(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;
        "chainx_getCrossMiningDividendByAccount" => fn cross_mining_dividend(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;

        "chainx_getNominationRecords" => fn nomination_records(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;
        "chainx_getNominationRecordsV1" => fn nomination_records_v1(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;
        "chainx_getPseduNominationRecords" => fn psedu_nomination_records(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;
        "chainx_getPseduNominationRecordsV1" => fn psedu_nomination_records_v1(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;

        "chainx_getIntentionByAccount" => fn intention(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;
        "chainx_getIntentions" => fn intentions(&self, hash: Option<Hash>) -> Result<Value>;
        "chainx_getIntentionsV1" => fn intentions_v1(&self, hash: Option<Hash>) -> Result<Value>;
        "chainx_getPseduIntentions" => fn psedu_intentions(&self, hash: Option<Hash>) -> Result<Value>;
        "chainx_getPseduIntentionsV1" => fn psedu_intentions_v1(&self, hash: Option<Hash>) -> Result<Value>;

        "chainx_getTradingPairs" => fn trading_pairs(&self, hash: Option<Hash>) -> Result<Value>;
        "chainx_getQuotations" => fn quotations(&self, id: u32, piece: u32, hash: Option<Hash>) -> Result<Value>;
        "chainx_getOrders" => fn orders(&self, who: Hash, page_index: u32, page_size: u32, hash: Option<Hash>) -> Result<Value>;

        "chainx_getTrusteeSessionInfo" => fn trustee_session_info(&self, chain: Chain, era: Option<u32>, hash: Option<Hash>) -> Result<Value>;
        "chainx_getTrusteeInfoByAccount" => fn trustee_by_account(&self, who: Hash, hash: Option<Hash>) -> Result<Value>;

        "chainx_getWithdrawTx" => fn withdraw_tx(&self, chain: Chain, hash: Option<Hash>) -> Result<Value>;

        "chainx_getMockBitcoinNewTrustees" => fn mock_btc_new_trustees(&self, candidates: Vec<Hash>) -> Result<Value>;

        "chainx_getFeeByCallAndLength" => fn call_fee(&self, call: String, tx_len: u64, hash: Option<Hash>) -> Result<Value>;
        "chainx_getFeeWeightMap" => fn call_fee_map(&self, hash: Option<Hash>) -> Result<Value>;

        "chainx_particularAccounts" => fn particular_accounts(&self, hash: Option<Hash>) -> Result<Value>;
    }
}
