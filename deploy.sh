#!/bin/bash
# Expects a running `dfx start` in the root dAppreciate
# directory

# Versions:
# - dfx 0.8.0
# - node v16.x.x
# - rustc stable?
set -euxo pipefail

pushd internet-identity
npm ci
II_ENV=development dfx deploy --no-wallet --argument '(null)'
II_PRINCIPAL=$(dfx canister --no-wallet id internet_identity)
popd

pushd dapp
npm ci
dfx deploy
popd

pushd index
npm ci
dfx deploy
INDEX_PRINCIPAL=$(dfx canister id index)
popd

pushd index_rs
# npm ci # enable this once the front-end details for index_rs are clear
dfx deploy
INDEX_RS_PRINCIPAL=$(dfx canister id index_rs)
popd

pushd client_rs
npm ci
pushd src/frontend
npm ci
popd
DFX_NETWORK=local dfx deploy
popd

