#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/ed25519_verification.wasm --accountId ed25519-verification.testnet

# near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/dsnp_identity.wasm