#!/bin/bash
set -e

near call dsnp.testnet "get_delegate" '{"account_id":  "dsnp.testnet"}' --accountId dsnp.testnet