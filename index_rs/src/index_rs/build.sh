#!/usr/bin/env bash
set -euo pipefail

# Compile frontend assets to dist
# echo SKIPPING: Compiling frontend assets
npm run build

INDEX_DIR="$(dirname "$0")"
TARGET="wasm32-unknown-unknown"

cargo build --manifest-path "$INDEX_DIR/Cargo.toml" --target $TARGET --release -j1

# keep version in sync with Dockerfile
cargo install ic-cdk-optimizer --version 0.3.1 --root "$INDEX_DIR"/../../target
STATUS=$?

if [ "$STATUS" -eq "0" ]; then
      "$INDEX_DIR"/../../target/bin/ic-cdk-optimizer \
      "$INDEX_DIR/../../target/$TARGET/release/index_rs.wasm" \
      -o "$INDEX_DIR/../../target/$TARGET/release/index_rs.wasm"

  true
else
  echo Could not install ic-cdk-optimizer.
  false
fi
