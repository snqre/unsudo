import * as exit_code from "reliq";
import * as dot_api from "@polkadot/api";
import * as dot_ext from "@polkadot/extension-dapp";








export async function connect(app_name: string) {
    let wallets = await dot_ext.web3Enable(app_name);
    (await wallets[0]?.metadata?.get()
    
}


namespace wallet {
    
    export type Wallet = {
        accounts: Array<account.Account>,

    };
}

namespace account {

    export type Account = {
        address: string,
        genesis_hash?: string,
        name?: string,
        kind?: KeyPair
    };

    export enum KeyPair {
        Ed25519,
        Sr25519,
        Ecdsa,
        Ethereum
    }
}