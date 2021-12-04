#!/bin/bash
set -e

# cargo build --target wasm32-unknown-unknown --release
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

# near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/dsnp_identity.wasm
# near deploy --wasmFile target/wasm32-unknown-unknown/release/dsnp_identity.wasm --accountId dsnp.testnet
# dev-1638480602878-94256636952712

# near call dsnp.testnet "new" '{"owner_id":  "dsnp.testnet"}' --accountId dsnp.testnet
# $1,655.56 
# $1,497.59

# add delegate 2 pennies
# add registration 2 pennies
# transfer ownership 2 pennies
# deploy identity


# 1,667.55

# 196625 // 2 Near
# 143967

