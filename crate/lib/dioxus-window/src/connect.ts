import * as exitCode from "reliq";
import * as dotAPI from "@polkadot/api";
import * as dotExt from "@polkadot/extension-dapp";

type Account = {

}

enum Error {
    PolkadotExtensionNotFound
}

async function __connect() {
    return exitCode.wrapAsync(async () => {
        let extension = await dotExt.web3Enable("Unsudo");
        if (extension.length === 0) {
            return exitCode.Err(Error.PolkadotExtensionNotFound);
        }
        let accounts = await dotExt.web3Accounts();
        return accounts[0]?.
    });
}


type Transaction = {
    sender: string,
    recipient: string,
    amount: number
};

async function forward_client_transaction_(): Promise<void> {

}