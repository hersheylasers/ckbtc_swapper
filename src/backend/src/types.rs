use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Network {
    Bitcoin,
    CkBTC,
}

#[derive(CandidType, Deserialize)]
pub struct UserPreferences {
    pub preferred_network: Network,
    pub auto_convert: bool,
    pub min_amount: u64,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preferred_network: Network::Bitcoin,
            auto_convert: false,
            min_amount: 0,
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ConversionRecord {
    pub timestamp: u64,
    pub from_network: Network,
    pub to_network: Network,
    pub amount: u64,
    pub status: ConversionStatus,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum ConversionStatus {
    Pending,
    Complete,
    Failed(String),
}
