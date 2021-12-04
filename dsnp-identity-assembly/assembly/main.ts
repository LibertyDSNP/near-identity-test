import {
  storage,
  logging,
  PersistentMap,
  ContractPromise,
  u128,
} from "near-sdk-as";


import { IdentityContract, Delegate } from "./model";

let contract: IdentityContract;

@nearBindgen
class AddDelegateParam {
  account_id: string;
  public_key: string;
  end_block: string;

  constructor(account_id: string, public_key: string, end_block: string) {
    account_id = this.account_id;
    public_key = this.public_key;
    end_block = this.end_block;
  }
}

export function initContract(ownerId: string): IdentityContract {
  assert(!storage.hasKey("init"), "Already initialized");

  let delegates = new PersistentMap<string, Delegate>("d");

  contract = new IdentityContract(ownerId, delegates);

  storage.set("init", true);

  return contract;
}

@nearBindgen
class Ed25519VerifyArgs {
  constructor(
    public signature: string,
    public public_key: string,
    public serialize_message: string
  ) {}
}

export function upsertDelegateViaSig(
  signature: string,
  sigPK: string,
  message: AddDelegateParam
): void {
  logging.log("signature: " + signature);

  let serialize_message = "[".concat([message.account_id,message.end_block,message.public_key].join()).concat("]");

  let sigArg = new Ed25519VerifyArgs(signature, sigPK, serialize_message);
  let promise = ContractPromise.create(
    "ed25519-verification.testnet",
    "ed25519_verify",
    sigArg.encode(),
    50000000000,
    u128.Zero
  );

  // let callbackPromise = promise.then(
  //   context.contractName,
  //   "_onVerify",
  //   new Uint8Array(0),
  //   5000000000000
  // );
}

export function _onVerify(isSuccess: u64): void {
  logging.log("Successsis now");
}
