# BTC-ckBTC Wallet

A wallet application that enables automatic conversion between Bitcoin (BTC) and Chain-key Bitcoin (ckBTC) based on user preferences.

## Features

- 🔄 Automatic conversion between BTC and ckBTC
- 🔐 Secure authentication with Internet Identity
- 💼 Manage Bitcoin and ckBTC balances
- 📊 View conversion history
- ⚙️ Set network preferences

## Prerequisites

- [x] [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx) (dfx >= 0.12.0)
- [x] [Node.js](https://nodejs.org/) (>= 16)
- [x] [Rust](https://www.rust-lang.org/tools/install)
- [x] [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/game-of-life/setup.html)

## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/hersheylasers/ckbtc_swapper.git
cd btc-ckbtc-wallet
```

2. Install dependencies:

```bash
npm install
```

3. Install Bitcoin node:

First time:

```bash
./scripts/setup.bitcoin-node.sh
```

After first time, run with this argument occasionally:

```bash
./scripts/setup.bitcoin-node.sh --reset
```

4. Start the local dfx with bitcoin:

```bash
./scripts/dfx.start-with-bitcoin.sh
```

You can also run it by cleaning up the state:

```bash
./scripts/dfx.start-with-bitcoin.sh --clean
```

5. Deploy the canisters locally:

```bash
dfx deploy
```

Individual canister deployment

(select regtest for local)

```bash
dfx deploy basic_bitcoin
```

```bash
dfx deploy internet_identity
```

```bash
dfx deploy wallet_backend
```

```bash
dfx deploy frontend
```

### Project Structure

```
btc-ckbtc-wallet/
├── src/
│   ├── backend/           # Rust canister code
│   │   ├── src/
│   │   │   ├── lib.rs    # Main canister code
│   │   │   ├── bitcoin.rs # Bitcoin integration
│   │   │   └── ckbtc.rs  # ckBTC integration
│   │   └── Cargo.toml
│   └── frontend/         # SvelteKit frontend
│       └── src/
│           ├── components/
│           ├── lib/
│           └── routes/
├── dfx.json             # Internet Computer configuration
└── package.json
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
