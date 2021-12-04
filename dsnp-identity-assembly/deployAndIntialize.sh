#!/bin/bash
set -e

# cargo build --target wasm32-unknown-unknown --release
# RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

# near dev-deploy --wasmFile out/main.wasm
# near deploy --wasmFile target/wasm32-unknown-unknown/release/dsnp_identity.wasm --accountId dsnp.testnet
# dev-1638480602878-94256636952712

# near deploy dev-1638572594110-70584534561471 --wasmFile out/main.wasm --accountId dev-1638572594110-70584534561471
near call dev-1638572594110-70584534561471 "upsertDelegateViaSig" '{"signature":  "2dcfbeaa399cddbb07cea7227a4b9b30b49ce7d8f7b72bd5046dffe8083b9c24c9862983254bc0c4297ca93729f4b737f8b5c26c33ef7ac205eb97ee5bdc190f", "sigPK": "3YjRJM8h1cNPL3JXvkDrWCTXtXA9EhSsbdi3LA9NacCz", "message": {"account_id": "dsnp.testnet", "end_block": "0", "public_key": "3YjRJM8h1cNPL3JXvkDrWCTXtXA9EhSsbdi3LA9NacCz"}}' --accountId dev-1638572594110-70584534561471
# near call dev-1638572594110-70584534561471 "initContract" '{"ownerId":  "dsnp.testnet"}' --accountId dev-1638572594110-70584534561471
# $1,655.56 
# $1,497.59

# add delegate 2 pennies
# add registration 2 pennies
# transfer ownership 2 pennies
# deploy identity


# 1,667.55

# 196625 // 2 Near
# 143967

