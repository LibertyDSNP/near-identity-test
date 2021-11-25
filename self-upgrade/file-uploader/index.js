const nearApi = require('near-api-js');
const fs = require("fs");
const os = require("os");

const accountId = 'dev-1637796671006-59179411080268';
const contractId = 'dev-1637796671006-59179411080268';
const method = 'store_blob';
const gas = '100000000000000';
const network = 'testnet';
const rpc = 'https://rpc.testnet.near.org';
const contractPath = '../res/self_upgrade.wasm';
const near_cred = os.homedir() + '/.near-credentials';

async function GetAccount(account_id) {
    try {
        console.log(near_cred);
        const fileKeyStore = new nearApi.keyStores.UnencryptedFileSystemKeyStore(near_cred);
        const keyPair = await fileKeyStore.getKey(network, accountId);
        console.log(keyPair);
        const keyStore = new nearApi.keyStores.InMemoryKeyStore();
        await keyStore.setKey("testnet", account_id, keyPair);
        const near = await nearApi.connect({
            networkId: network,
            deps: {keyStore},
            masterAccount: account_id,
            nodeUrl: rpc
        });

        return await near.account(account_id);
    } catch (e) {
        console.log(e);
    }
}

async function upload(account_id, recipient) {
    try {
        const data = Uint8Array.from([...fs.readFileSync(contractPath)]);
        const account = await GetAccount(account_id);
        return await account.functionCall(
            recipient,
            method,
            data,
            gas,
            0);
    } catch (e) {
        console.log(e);
    }
}

upload(accountId, contractId).then(v => console.log(v));
