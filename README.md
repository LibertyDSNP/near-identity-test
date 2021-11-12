# NEAR protocol miniature test contract

## Run Tests

```
cargo test -- --nocapture
```

## Compile

```
cargo build --target wasm32-unknown-unknown --release
```

## Deploy to testnet

Login to your own testnet account with `near login` 

```
near deploy --wasmFile target/wasm32-unknown-unknown/release/near_identity.wasm --accountId YOUR_ACCOUNT_HERE
```

## Invoke Methods

```
near call YOUR_ACCOUNT_HERE new_registration --accountId YOUR_ACCOUNT_HERE
```