import * as Dot from "@polkadot/extension-dapp";


namespace Wallet {

    export async function connect(app_name: string) {
        (await Dot.web3Enable(app_name));

        
    }

    //
    export async function request_transaction() {

    }
}


