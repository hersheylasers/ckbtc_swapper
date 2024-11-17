use candid::Principal;
use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    // Mock storage for ckBTC balances
    static BALANCES: RefCell<HashMap<Principal, u64>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct CkBTCWallet {
    pub minter_id: Principal,
}

impl CkBTCWallet {
    pub fn new(minter_id: Principal) -> Self {
        Self { minter_id }
    }

    pub fn get_ckbtc_balance_sync(&self, owner: Principal) -> Result<u64, String> {
        // For testing, return mock balance
        BALANCES.with(|balances| {
            Ok(*balances.borrow().get(&owner).unwrap_or(&0))
        })
    }

    pub async fn get_btc_address(&self, _owner: Principal) -> Result<String, String> {
        // For testing, return a mock BTC address
        Ok("tb1qsg537l0sp7qjq8hkpu9rpxjv0jx3rclhcm0kd9".to_string())
    }

    pub async fn retrieve_btc(
        &self,
        owner: Principal,
        _address: String,
        amount: u64,
    ) -> Result<(), String> {
        // Mock implementation: just decrease the ckBTC balance
        BALANCES.with(|balances| {
            let mut balances = balances.borrow_mut();
            let current_balance = *balances.get(&owner).unwrap_or(&0);
            if current_balance < amount {
                return Err("Insufficient ckBTC balance".to_string());
            }
            balances.insert(owner, current_balance - amount);
            Ok(())
        })
    }

    pub async fn get_ckbtc_balance(&self, owner: Principal) -> Result<u64, String> {
        self.get_ckbtc_balance_sync(owner)
    }

    // Helper function for testing: mint some ckBTC to an account
    #[cfg(test)]
    pub fn mint_test_tokens(&self, owner: Principal, amount: u64) {
        BALANCES.with(|balances| {
            let mut balances = balances.borrow_mut();
            let current_balance = *balances.get(&owner).unwrap_or(&0);
            balances.insert(owner, current_balance + amount);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_operations() {
        let wallet = CkBTCWallet::new(Principal::anonymous());
        let user = Principal::anonymous();

        // Initial balance should be 0
        assert_eq!(wallet.get_ckbtc_balance_sync(user).unwrap(), 0);

        // Mint some test tokens
        wallet.mint_test_tokens(user, 1000);
        assert_eq!(wallet.get_ckbtc_balance_sync(user).unwrap(), 1000);
    }
}
