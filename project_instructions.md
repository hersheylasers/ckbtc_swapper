# BTC-ckBTC Swap Wallet Prototype

## Project Description

A simplified wallet implementation that demonstrates automatic conversion between Bitcoin (BTC) and Chain-key Bitcoin (ckBTC) based on user preferences. The wallet allows users to specify their preferred network (Bitcoin or Internet Computer) and automatically handles the conversion of tokens to maintain balances on the preferred network.

## Folder Structure

```
btc-ckbtc-wallet/
├── src/
│   ├── backend/
│   │   ├── Cargo.toml           # Rust dependencies
│   │   └── src/
│   │       ├── lib.rs           # Main canister code
│   │       ├── bitcoin.rs       # Bitcoin integration
│   │       ├── ckbtc.rs         # ckBTC integration
│   │       └── types.rs         # Shared types
│   ├── frontend/
│   │   ├── src/
│   │   │   ├── components/
│   │   │   │   ├── WalletHeader.svelte
│   │   │   │   ├── NetworkPreference.svelte
│   │   │   │   ├── BalanceDisplay.svelte
│   │   │   │   └── ConversionHistory.svelte
│   │   │   ├── lib/
│   │   │   │   └── api.ts      # Backend canister API
│   │   │   └── App.svelte
│   │   └── index.html
│   └── declarations/           # Auto-generated canister interfaces
├── dfx.json                    # Project configuration
└── package.json               # Node.js dependencies
```

## Features

### Core Features

1. **Network Preference Management**

   - Set preferred network (Bitcoin or Internet Computer)
   - Enable/disable automatic conversion
   - View current preference settings

2. **Wallet Functionality**

   - Generate and manage Bitcoin addresses for users
   - Display BTC and ckBTC balances
   - Show conversion history

3. **Automatic Conversion**
   - Convert BTC to ckBTC when preferred network is IC
   - Convert ckBTC to BTC when preferred network is Bitcoin
   - Handle conversion fees and transaction costs

### User Interface

1. **Simple Settings Panel**

   - Network preference toggle
   - Auto-convert switch
   - Minimum conversion amount setting

2. **Balance Display**

   - Show balances on both networks
   - Indicate pending conversions
   - Display equivalent USD values

3. **Transaction History**
   - List of recent conversions
   - Current conversion status
   - Transaction details

## Implementation Plan

### Phase 1: Basic Infrastructure

1. Set up project structure and dependencies
2. Create basic canister with Bitcoin integration
3. Implement ckBTC integration
4. Add network preference storage

### Phase 2: Core Backend Features

1. Implement Bitcoin address generation and management
2. Add ckBTC balance checking and transfers
3. Create conversion logic between networks
4. Set up automatic conversion triggers

### Phase 3: Frontend Development

1. Create basic wallet interface
2. Add network preference controls
3. Implement balance display
4. Add conversion history view

### Phase 4: Testing & Refinement

1. Test automatic conversions
2. Add error handling
3. Implement transaction monitoring
4. Add conversion status updates

## Technical Specifications

### Backend Canister

```rust
#[derive(CandidType, Deserialize)]
pub enum PreferredNetwork {
    Bitcoin,
    CkBTC,
}

#[derive(CandidType, Deserialize)]
pub struct UserPreferences {
    preferred_network: PreferredNetwork,
    auto_convert: bool,
    min_amount: Nat,
}

pub trait WalletService {
    fn set_network_preference(&mut self, network: PreferredNetwork) -> Result<(), String>;
    fn get_btc_address(&self) -> String;
    fn get_balances(&self) -> (Nat, Nat);  // (BTC, ckBTC)
    fn convert_to_preferred_network(&mut self) -> Result<(), String>;
}
```

### Frontend Components

```typescript
interface WalletState {
	btcBalance: bigint;
	ckBTCBalance: bigint;
	preferredNetwork: 'bitcoin' | 'ckbtc';
	autoConvert: boolean;
	conversionHistory: ConversionRecord[];
}

interface ConversionRecord {
	timestamp: number;
	fromNetwork: string;
	toNetwork: string;
	amount: bigint;
	status: 'pending' | 'complete' | 'failed';
}
```

## Development Guidelines

1. **Security First**

   - Use IC's secure key management
   - Implement proper access controls
   - Validate all user inputs

2. **User Experience**

   - Minimize manual input requirements
   - Provide clear conversion status
   - Show helpful error messages

3. **Code Quality**

   - Write clear documentation
   - Include unit tests
   - Follow Rust and Svelte best practices

4. **Performance**
   - Optimize conversion triggers
   - Minimize unnecessary updates
   - Handle network latency gracefully
