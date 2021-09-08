#!/bin/bash
# Expects a running `dfx start` in the root dAppreciate
# directory

# Versions:
# - dfx 0.8.0
# - node v16.x.x
# - rustc stable?
set -euxo pipefail

export DFX_NETWORK=local

pushd internet-identity
npm ci
II_ENV=development dfx deploy --no-wallet --argument '(null)'
II_CANISTER_ID=$(dfx canister --no-wallet id internet_identity)
popd

pushd index_rs
npm ci
dfx deploy
INDEX_CANISTER_ID=$(dfx canister id index_rs)
popd

pushd client_rs
npm ci
pushd src/frontend
npm ci
popd
dfx deploy
popd

pushd dapp
npm ci
INDEX_CANISTER_URL=https://localhost:8000/?canisterId=$INDEX_CANISTER_ID dfx deploy
popd
