use candid::Principal;
use ic_cdk::api::call::{call, CallResult};
use ic_cdk::export::candid::Principal as CanisterId;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::id;
use ic_cdk_macros::update;
use std::str::FromStr;

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetUtxosResponse {
    pub utxos: Vec<Utxo>,
    pub tip_block_hash: Vec<u8>,
    pub tip_height: u32,
    pub next_page: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub struct Utxo {
    pub outpoint: Outpoint,
    pub value: u64,
    pub height: u32,
}

#[derive(CandidType, Deserialize)]
pub struct Outpoint {
    pub txid: Vec<u8>,
    pub vout: u32,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlockHeadersResponse {
    pub tip_height: u32,
    pub block_headers: Vec<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub struct MillisatoshiPerByte(pub u64);

#[derive(Clone)]
pub struct BitcoinWallet {
    network: BitcoinNetwork,
    basic_bitcoin_canister_id: CanisterId,
}

impl BitcoinWallet {
    pub fn new(network: BitcoinNetwork, basic_bitcoin_canister_id: CanisterId) -> Self {
        Self {
            network,
            basic_bitcoin_canister_id,
        }
    }

    pub async fn get_balance(&self, address: &str) -> Result<u64, String> {
        let result: CallResult<u64> = call(
            self.basic_bitcoin_canister_id,
            "get_balance",
            (address.to_string(),),
        )
        .await;

        result.map_err(|e| format!("Failed to get balance: {:?}", e))
    }

    pub async fn get_utxos(&self, address: &str) -> Result<GetUtxosResponse, String> {
        let result: CallResult<GetUtxosResponse> = call(
            self.basic_bitcoin_canister_id,
            "get_utxos",
            (address.to_string(),),
        )
        .await;

        result.map_err(|e| format!("Failed to get UTXOs: {:?}", e))
    }

    pub async fn get_block_headers(
        &self,
        start_height: u32,
        end_height: Option<u32>,
    ) -> Result<GetBlockHeadersResponse, String> {
        let result: CallResult<GetBlockHeadersResponse> = call(
            self.basic_bitcoin_canister_id,
            "get_block_headers",
            (start_height, end_height),
        )
        .await;

        result.map_err(|e| format!("Failed to get block headers: {:?}", e))
    }

    pub async fn get_current_fee_percentiles(&self) -> Result<Vec<MillisatoshiPerByte>, String> {
        let result: CallResult<Vec<MillisatoshiPerByte>> = call(
            self.basic_bitcoin_canister_id,
            "get_current_fee_percentiles",
            (),
        )
        .await;

        result.map_err(|e| format!("Failed to get fee percentiles: {:?}", e))
    }

    pub async fn get_p2pkh_address(&self) -> Result<String, String> {
        let result: CallResult<String> = call(
            self.basic_bitcoin_canister_id,
            "get_p2pkh_address",
            (),
        )
        .await;

        result.map_err(|e| format!("Failed to get P2PKH address: {:?}", e))
    }

    pub async fn send_from_p2pkh(&self, request: SendRequest) -> Result<String, String> {
        let result: CallResult<String> = call(
            self.basic_bitcoin_canister_id,
            "send_from_p2pkh",
            (request,),
        )
        .await;

        result.map_err(|e| format!("Failed to send from P2PKH: {:?}", e))
    }

    pub async fn get_p2tr_script_spend_address(&self) -> Result<String, String> {
        let result: CallResult<String> = call(
            self.basic_bitcoin_canister_id,
            "get_p2tr_script_spend_address",
            (),
        )
        .await;

        result.map_err(|e| format!("Failed to get P2TR script spend address: {:?}", e))
    }

    pub async fn send_from_p2tr_script_spend(&self, request: SendRequest) -> Result<String, String> {
        let result: CallResult<String> = call(
            self.basic_bitcoin_canister_id,
            "send_from_p2tr_script_spend",
            (request,),
        )
        .await;

        result.map_err(|e| format!("Failed to send from P2TR script spend: {:?}", e))
    }

    pub async fn get_p2tr_raw_key_spend_address(&self) -> Result<String, String> {
        let result: CallResult<String> = call(
            self.basic_bitcoin_canister_id,
            "get_p2tr_raw_key_spend_address",
            (),
        )
        .await;

        result.map_err(|e| format!("Failed to get P2TR raw key spend address: {:?}", e))
    }

    pub async fn send_from_p2tr_raw_key_spend(&self, request: SendRequest) -> Result<String, String> {
        let result: CallResult<String> = call(
            self.basic_bitcoin_canister_id,
            "send_from_p2tr_raw_key_spend",
            (request,),
        )
        .await;

        result.map_err(|e| format!("Failed to send from P2TR raw key spend: {:?}", e))
    }

    pub async fn get_or_create_address(&self, _principal: &Principal) -> Result<String, String> {
        self.get_p2pkh_address().await
    }
}
