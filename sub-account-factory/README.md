# Sub-Account Factory

This is directly adapted from **[Sputnikdao-factory2](https://github.com/near-daos/sputnik-dao-contract/tree/main/sputnikdao-factory2)**

# Deployment & Usage

## TestNet

```
near dev-deploy --wasmFile=res/sub_account_factory.wasm

# bash
CONTRACT_ID="<generated dev account>"

# Initialize the factory.
near call $CONTRACT_ID new '{}' --accountId $CONTRACT_ID 

# Create a new SubAccount with the given parameters.
near call $CONTRACT_ID create '{"public_key": null}'  --accountId $CONTRACT_ID --amount 30 --gas 100000000000000

# Create a new SubAccount with the given parameters while having Full Access Key to the account (trusted, but useful in case of testing or upgrades)
near call $CONTRACT_ID create '{"public_key": "<public key>"}'  --accountId $CONTRACT_ID --amount 30 --gas 100000000000000

# List all created accounts.
near view $CONTRACT_ID get_accounts_list
```
