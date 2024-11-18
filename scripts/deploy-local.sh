#!/bin/bash
set -e

# Start DFX if not already running
if ! dfx ping; then
    dfx start --clean --background
fi

# Deploy Internet Identity
if ! dfx canister --network=local id internet_identity > /dev/null 2>&1; then
    dfx deploy internet_identity
fi

# Deploy ckBTC minter if needed
if ! dfx canister --network=local id ckbtc_minter > /dev/null 2>&1; then
    dfx deploy ckbtc_minter
fi

# Deploy wallet backend with regtest network for local development
dfx deploy wallet_backend --argument '(variant { regtest })'

# Deploy frontend assets
dfx deploy frontend

echo "Local deployment completed successfully!"
echo "Frontend URL: http://localhost:4943/?canisterId=$(dfx canister id frontend)"
