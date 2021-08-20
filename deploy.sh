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

pushd client
npm ci
pushd src/frontend
npm ci
popd
dfx deploy --argument "(principal \"$INDEX_PRINCIPAL\")"
popd

