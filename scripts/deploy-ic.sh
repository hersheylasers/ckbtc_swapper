#!/bin/bash
set -e

# Check if we have enough cycles
dfx wallet --network=ic balance

# Deploy wallet backend
echo "Deploying wallet backend..."
dfx deploy --network=ic wallet_backend --argument '(variant { mainnet })'

# Deploy frontend assets
echo "Deploying frontend..."
dfx deploy --network=ic frontend

echo "IC deployment completed successfully!"
echo "Frontend canister ID: $(dfx canister --network=ic id frontend)"
