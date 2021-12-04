#!/bin/bash
set -e

near call dsnp.testnet "upsert_delegate_via_sig" '{"signature":  "2dcfbeaa399cddbb07cea7227a4b9b30b49ce7d8f7b72bd5046dffe8083b9c24c9862983254bc0c4297ca93729f4b737f8b5c26c33ef7ac205eb97ee5bdc190f", "public_key": "3YjRJM8h1cNPL3JXvkDrWCTXtXA9EhSsbdi3LA9NacCz", "message": {"account_id": "dsnp.testnet", "end_block": "0", "public_key": "3YjRJM8h1cNPL3JXvkDrWCTXtXA9EhSsbdi3LA9NacCz"}}' --accountId dsnp.testnet