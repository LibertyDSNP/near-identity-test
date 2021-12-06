#!/bin/bash
set -e

# near deploy ecrecover.testnet target/wasm32-unknown-unknown/release/ecrecover_experiemnts.wasm

# near call ecrecover.testnet "verify" '{"message_hash": "ce0677bb30baa8cf067c88db9811f4333d131bf8bcf12fe7065d211dce971008", "signature": "90f27b8b488db00b00606796d2987f6a5f59ae62ea05effe84fef5b8b0e549984a691139ad57a3f0b906637673aa2f63d1f55cb1a69199d4009eea23ceaddc93" }' --accountId ecrecover.testnet
# near call ecrecover.testnet "verify" '{"message_hash": "60be35518133bb945595a87176455ea5cb358f0768edf67855fd8b4317486a61", "signature": "0000000000000000000000000000000000000000000000000000000000000000443a4b19c24abdace71770faad25665fbc5b00450ec0b76c6e87e9a4383747fd" }' --accountId ecrecover.testnet

near call ecrecover.testnet "verify" '{"message_hash": "a727ef196c4ed856629b4274297ae7a7b6225043defbde6cd30c0d78f30d6d0b", "signature": "000000000000000000000000000000000000000000000000000000000000000188785d53d67fe3cfff690d4c8785c5facef3a19e9bec59933d352973a5da554a" }' --accountId dsnp.testnet