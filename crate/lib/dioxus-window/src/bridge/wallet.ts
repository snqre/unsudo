import * as polkadot_ext from "@polkadot/extension-dapp";
import {ConnectionMode} from "./connection-mode";
import {Account} from "./account";
import {Source} from "./source";
import {Option, Some, None} from "reliq";

export type Wallet = {
    name: string,
    connection_mode: Option<ConnectionMode>,
    accounts: Option<Array<Account>>
}

export namespace Wallet {
    export async function from_polkadot(app_name: string): Promise<reliq.Option<Array<Wallet>>> {
        try {
            let ext = await polkadot_ext.web3Enable(app_name);
            if (ext === undefined || ext === null || ext.length === 0) {
                return reliq.None;
            }
            let all = await polkadot_ext.web3Accounts();
            if (all === undefined || all === null || all.length === 0) {
                return reliq.None;
            }
            for (let account of all) {
                let source$0 = account.meta.source;
                let source$1 = source.from_polkadot_meta_source(source$0).unwrapOr(source.Source.PolkadotJs);
                

                let account$0 = {
                    address: account.address,
                    key_pair: cmn.KeyPair.Sr25519,
                    name: (() => {
                        if (account.meta.name !== undefined) {
                            return reliq.Some(account.meta.name);
                        } else {
                            return reliq.None;
                        }
                    })(),
                    protocol: reliq.Some(cmn.Protocol.Polkadot),
                    source: reliq.Some(cmn.Source.PolkadotJs)
                } as account.Account;
                
            }
        } catch {

        }
    }




    export function accounts_by_protocol(wallet: Wallet, protocol: cmn.Protocol): Array<cmn.Account> {  
        if (wallet.accounts.none()) {
            return [];
        }
        if (wallet.accounts.unwrap().length === 0) {
            return [];
        }
        return wallet.accounts
            .unwrap()
            .filter(account => account.protocol === protocol);
    }

    export function supports_protocol(wallet: Wallet, protocol: cmn.Protocol): boolean {
        if (wallet.accounts.none()) {
            return false;
        }
        if (wallet.accounts.unwrap().length === 0) {
            return false;
        }
        return wallet.accounts
            .unwrap()
            .some(account => account.protocol === protocol);
    }

    export function address(wallet: Wallet, key: bigint): reliq.Option<string> {
        if (wallet.accounts.none() || wallet.accounts.unwrap().length === 0 || wallet.accounts.unwrap().length > key) {
            return reliq.None;
        }
        let account: cmn.Account | undefined = wallet.accounts.unwrap().at(Number(key));
        if (account === undefined) {
            throw Error();
        }
        return reliq.Some(account.address);
    }

    export function size(wallet: Wallet): bigint {
        if (wallet.accounts.none()) {
            return 0n;
        }
        let accounts: Array<cmn.Account> = wallet.accounts.unwrap();
        return BigInt(accounts.length);
    }
}