use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;

mod bitcoin;
mod ckbtc;
mod types;
mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod schnorr_api;

use bitcoin::BitcoinWallet;
use ckbtc::CkBTCWallet;
use types::{Network, UserPreferences, ConversionRecord, ConversionStatus};

#[derive(CandidType)]
struct Balances {
    btc: u64,
    ckbtc: u64,
}

thread_local! {
    // The bitcoin network to connect to.
    //
    // When developing locally this should be `Regtest`.
    // When deploying to the IC this should be `Testnet` or `Mainnet`.
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);

    // The derivation path to use for the threshold key.
    static DERIVATION_PATH: Vec<Vec<u8>> = vec![];

    // The ECDSA key name.
    static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));
}

#[init]
pub fn init(network: BitcoinNetwork) {
    NETWORK.with(|n| n.set(network));

    KEY_NAME.with(|key_name| {
        key_name.replace(String::from(match network {
            // For local development, we use a special test key with dfx.
            BitcoinNetwork::Regtest => "dfx_test_key",
            // On the IC we're using a test ECDSA key.
            BitcoinNetwork::Mainnet | BitcoinNetwork::Testnet => "test_key_1",
        }))
    });
}

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_balance(network, address).await
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_utxos(network, address).await
}

pub type Height = u32;
pub type BlockHeader = Vec<u8>;

/// A request for getting the block headers for a given height range.
#[derive(CandidType, Debug, Deserialize, PartialEq, Eq)]
pub struct GetBlockHeadersRequest {
    pub start_height: Height,
    pub end_height: Option<Height>,
    pub network: BitcoinNetwork,
}

/// The response returned for a request for getting the block headers from a given height.
#[derive(CandidType, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct GetBlockHeadersResponse {
    pub tip_height: Height,
    pub block_headers: Vec<BlockHeader>,
}

/// Returns the block headers in the given height range.
#[update]
pub async fn get_block_headers(start_height: u32, end_height: Option<u32>) -> GetBlockHeadersResponse{
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_block_headers(network, start_height, end_height).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_current_fee_percentiles(network).await
}

/// Returns the P2PKH address of this canister at a specific derivation path.
#[update]
pub async fn get_p2pkh_address() -> String {
    let derivation_path = DERIVATION_PATH.with(|d| d.clone());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    bitcoin_wallet::p2pkh::get_address(network, key_name, derivation_path).await
}

/// Sends the given amount of bitcoin from this canister's p2pkh address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2pkh(request: SendRequest) -> String {
    let derivation_path = DERIVATION_PATH.with(|d| d.clone());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2pkh::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_script_spend_address() -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"script_spend".to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());

    bitcoin_wallet::p2tr_script_spend::get_address(network, key_name, derivation_path)
        .await
        .to_string()
}

/// Sends the given amount of bitcoin from this canister's p2tr address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2tr_script_spend(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"script_spend".to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr_script_spend::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_raw_key_spend_address() -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"key_spend".to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());

    bitcoin_wallet::p2tr_raw_key_spend::get_address(network, key_name, derivation_path)
        .await
        .to_string()
}

/// Sends the given amount of bitcoin from this canister's p2tr address to the
/// given address. Returns the transaction ID.
///
/// IMPORTANT: This function uses an untweaked key as the spending key.
///
/// WARNING: This function is not suited for multi-party scenarios where
/// multiple keys are used for spending.
#[update]
pub async fn send_from_p2tr_raw_key_spend(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"key_spend".to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr_raw_key_spend::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}

struct State {
    bitcoin_wallet: BitcoinWallet,
    ckbtc_wallet: Option<CkBTCWallet>,
    user_preferences: HashMap<Principal, UserPreferences>,
    btc_addresses: HashMap<Principal, String>,
    conversion_history: HashMap<Principal, Vec<ConversionRecord>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            bitcoin_wallet: BitcoinWallet::new(BitcoinNetwork::Testnet),
            ckbtc_wallet: None,
            user_preferences: HashMap::new(),
            btc_addresses: HashMap::new(),
            conversion_history: HashMap::new(),
        }
    }
}

#[init]
fn init(minter_id: Option<Principal>) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(minter_id) = minter_id {
            state.ckbtc_wallet = Some(CkBTCWallet::new(minter_id));
        }
    });
}

#[query]
fn get_balances() -> Balances {
    let caller = ic_cdk::caller();
    
    // Get BTC balance
    let btc_balance = STATE.with(|state| {
        let state = state.borrow();
        match state.btc_addresses.get(&caller) {
            Some(address) => state.bitcoin_wallet.get_balance_sync(address),
            None => Ok(0)
        }
    }).unwrap_or(0);
    
    // Get ckBTC balance
    let ckbtc_balance = STATE.with(|state| {
        let state = state.borrow();
        match &state.ckbtc_wallet {
            Some(wallet) => wallet.get_ckbtc_balance_sync(caller),
            None => Ok(0)
        }
    }).unwrap_or(0);
    
    Balances {
        btc: btc_balance,
        ckbtc: ckbtc_balance,
    }
}

#[query]
fn get_preferred_network() -> Network {
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        state.borrow()
            .user_preferences
            .get(&caller)
            .map(|prefs| prefs.preferred_network.clone())
            .unwrap_or(Network::Bitcoin)
    })
}

#[update]
fn set_preferred_network(network: Network) {
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.user_preferences
            .entry(caller)
            .or_insert_with(UserPreferences::default)
            .preferred_network = network;
    });
}

#[query]
fn get_conversion_history() -> Vec<ConversionRecord> {
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        state.borrow()
            .conversion_history
            .get(&caller)
            .cloned()
            .unwrap_or_default()
    })
}

#[update]
async fn convert_to_btc() -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    // Get user's ckBTC balance
    let ckbtc_balance = STATE.with(|state| {
        let state = state.borrow();
        match &state.ckbtc_wallet {
            Some(ref ckbtc_wallet) => ckbtc_wallet.get_ckbtc_balance_sync(caller),
            None => Err("ckBTC minter not initialized".to_string())
        }
    })?;
    
    if ckbtc_balance == 0 {
        return Err("No ckBTC balance to convert".to_string());
    }
    
    // Get user's BTC address
    let btc_address = STATE.with(|state| {
        state.borrow()
            .btc_addresses
            .get(&caller)
            .cloned()
            .ok_or_else(|| "BTC address not found".to_string())
    })?;
    
    // Create conversion record
    let record = ConversionRecord {
        timestamp: time(),
        from_network: Network::CkBTC,
        to_network: Network::Bitcoin,
        amount: ckbtc_balance,
        status: ConversionStatus::Pending,
    };
    
    // Add to history
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.conversion_history
            .entry(caller)
            .or_default()
            .push(record.clone());
    });
    
    // Clone necessary data before async operation
    let wallet = STATE.with(|state| {
        let state = state.borrow();
        state.ckbtc_wallet.as_ref().map(|w| w.clone())
    }).ok_or("ckBTC minter not initialized")?;

    // Initiate conversion
    match wallet.retrieve_btc(caller, btc_address.clone(), ckbtc_balance).await {
        Ok(_) => {
            // Update conversion status to complete
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                if let Some(history) = state.conversion_history.get_mut(&caller) {
                    if let Some(last) = history.last_mut() {
                        last.status = ConversionStatus::Complete;
                    }
                }
            });
            Ok(())
        }
        Err(e) => {
            // Update conversion status to failed
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                if let Some(history) = state.conversion_history.get_mut(&caller) {
                    if let Some(last) = history.last_mut() {
                        last.status = ConversionStatus::Failed(e.clone());
                    }
                }
            });
            Err(e)
        }
    }
}

#[update]
async fn convert_to_ckbtc() -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    // Get user's BTC address
    let btc_address = STATE.with(|state| {
        state.borrow()
            .btc_addresses
            .get(&caller)
            .cloned()
            .ok_or_else(|| "BTC address not found".to_string())
    })?;
    
    // Clone bitcoin wallet for async operation
    let bitcoin_wallet = STATE.with(|state| state.borrow().bitcoin_wallet.clone());
    
    // Get BTC balance
    let btc_balance = bitcoin_wallet.get_balance(&btc_address).await?;
    
    if btc_balance == 0 {
        return Err("No BTC balance to convert".to_string());
    }
    
    // Get ckBTC minter address
    let wallet = STATE.with(|state| {
        let state = state.borrow();
        state.ckbtc_wallet.as_ref().map(|w| w.clone())
    }).ok_or("ckBTC minter not initialized")?;

    let minter_address = wallet.get_btc_address(caller).await?;
    
    // Create conversion record
    let record = ConversionRecord {
        timestamp: time(),
        from_network: Network::Bitcoin,
        to_network: Network::CkBTC,
        amount: btc_balance,
        status: ConversionStatus::Pending,
    };
    
    // Add to history
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.conversion_history
            .entry(caller)
            .or_default()
            .push(record.clone());
    });
    
    // Send BTC to minter
    match bitcoin_wallet.send_btc(&btc_address, &minter_address, btc_balance).await {
        Ok(_) => {
            // Update conversion status to complete
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                if let Some(history) = state.conversion_history.get_mut(&caller) {
                    if let Some(last) = history.last_mut() {
                        last.status = ConversionStatus::Complete;
                    }
                }
            });
            Ok(())
        }
        Err(e) => {
            // Update conversion status to failed
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                if let Some(history) = state.conversion_history.get_mut(&caller) {
                    if let Some(last) = history.last_mut() {
                        last.status = ConversionStatus::Failed(e.clone());
                    }
                }
            });
            Err(e)
        }
    }
}

// Candid interface generation
ic_cdk::export_candid!();
