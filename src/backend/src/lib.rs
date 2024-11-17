use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;

mod bitcoin;
mod ckbtc;
mod types;

use bitcoin::BitcoinWallet;
use ckbtc::CkBTCWallet;
use types::{Network, UserPreferences, ConversionRecord, ConversionStatus};

#[derive(CandidType)]
struct Balances {
    btc: u64,
    ckbtc: u64,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
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
