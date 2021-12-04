import { PersistentMap, u128, PersistentSet, storage, base58, env, Context } from "near-sdk-as";
import { AccountId, PublicKey } from './types'


@nearBindgen
export class Delegate {
  end_block: u64;
  nonce: u32;

  constructor(end_block: u64, nonce: u32) {
      end_block = this.end_block;
      nonce = this.nonce;
  }
}

@nearBindgen
export class IdentityContract {
    owner_id: string;
    delegates: PersistentMap<AccountId, Delegate>;
    
    constructor(owner_id: AccountId, delegates: PersistentMap<AccountId, Delegate>) {
        owner_id = this.owner_id;
        delegates = this.delegates;

    }
}

