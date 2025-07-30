import * as Polkadot from "@polkadot/extension-dapp";
import {Option, Some, None} from "reliq";
import {ConnectionMode} from "./connection-mode";
import {Account} from "./account";
import {Source} from "./source";
import {Protocol} from "./protocol";
import {KeyPair} from "./key-pair";

export type Wallet = {
    name: string,
    connection_mode: Option<ConnectionMode>,
    accounts: Option<Array<Account>>
}

export namespace Wallet {

    export async function fetch_from_polkadot(app_name: Readonly<string>): Promise<Option<Array<Wallet>>> {
        try {
            const ext = await Polkadot.web3Enable(app_name);
            if (ext === undefined || ext === null || ext.length === 0) {
                return None;
            }
            const all = await Polkadot.web3Accounts();
            if (all === undefined || all === null || all.length === 0) {
                return None;
            }
            const wallets: Map<string, Wallet> = new Map();
            for (const account of all) {
                const injected_source: string = account.meta.source;
                const source: Source = Source.from_polkadot_meta_source(injected_source).unwrapOr(Source.PolkadotJs);
                const parsed_account: Account = Account.from({
                    address: account.address,
                    key_pair: KeyPair.Sr25519,
                    name: (() => {
                        if (account.meta.name === undefined || account.meta.name === null) {
                            return None;
                        }
                        return Some(account.meta.name);
                    })(),
                    protocol: Some(Protocol.Polkadot),
                    source: Some(Source.PolkadotJs)
                });
                const source_key: string = source.toString();
                if (!wallets.has(source_key)) {
                    wallets.set(source_key, {
                        name: source_key,
                        connection_mode: Some(ConnectionMode.Extension),
                        accounts: Some([
                            parsed_account
                        ])
                    });
                } else {
                    const wallet: Wallet = wallets.get(source_key)!;
                    if (wallet.accounts.some()) {
                        wallet.accounts.unwrap().push(parsed_account);
                    }
                }
            }
            return Some(Array.from(wallets.values()));
        } catch {
            return None;
        }
    }

    export function accounts_by_protocol(wallet: Wallet, protocol: Protocol): Array<Account> {  
        if (wallet.accounts.none()) {
            return [];
        }
        if (wallet.accounts.unwrap().length === 0) {
            return [];
        }
        return wallet.accounts
            .unwrap()
            .filter(account => account.protocol.unwrapOr(null) === protocol);
    }

    export function supports_protocol(wallet: Wallet, protocol: Protocol): boolean {
        if (wallet.accounts.none()) {
            return false;
        }
        if (wallet.accounts.unwrap().length === 0) {
            return false;
        }
        return wallet.accounts
            .unwrap()
            .some(account => account.protocol.unwrapOr(null) === protocol);
    }

    export function address(wallet: Wallet, key: bigint): Option<string> {
        if (wallet.accounts.none() || wallet.accounts.unwrap().length === 0 || wallet.accounts.unwrap().length > key) {
            return None;
        }
        let account: Account | undefined = wallet.accounts.unwrap().at(Number(key));
        if (account === undefined) {
            throw Error();
        }
        return Some(account.address);
    }

    export function size(wallet: Wallet): bigint {
        if (wallet.accounts.none()) {
            return 0n;
        }
        let accounts: Array<Account> = wallet.accounts.unwrap();
        return BigInt(accounts.length);
    }
}