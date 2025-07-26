import * as reliq from "reliq";
import * as api from "@polkadot/api";
import * as ext from "@polkadot/extension-dapp";
import * as helper_type from "@polkadot/util/types";
import * as helper from "@polkadot/util";
import * as crypto from "@polkadot/util-crypto";

export type Wallet = {
    name: string,
    connection_mode?: ConnectionMode,
    accounts?: Account
}

export type Account = {
    address: string,
    key_pair: KeyPair,
    name: reliq.Option<string>,
    protocol: reliq.Option<Protocol>,
    source: reliq.Option<Source>
}

export enum Source {
    Ledger,
    Trezor,
    Keystone,
    Metamask,
    Phantom,
    TrustWallet,
    PolkadotJs,
    SubWallet,
    NovaWallet,
    Talisman,
    Rabby,
    Brave,
    CoinbaseWallet,
    XDEFI,
    Exodus,
    Frame
}

export enum KeyPair {
    Ed25519,
    Sr25519,
    Secp256k1,
    Ecdsa,
    Ethereum
}

export enum Protocol {
    Polkadot,
    Kusama,
    Ethereum,
    Arbitrum,
    Optimism,
    BinanceSmartChain,
    Polygon,
    Solana,
    Avalanche,
    Bitcoin,
    Cosmos,
    Near,
    Tezos,
    Dogecoin,
    Litecoin,
    Zcash,
    Starknet,
    Aptos,
    Sui
}

export enum ConnectionMode {
    Extension,
    WebUsb,
    WebHid,
    WalletConnect,
    NativeApp,
    Injected,
    QRCode,
    Bluetooth,
    NFC
}

export type ConnectionRequest = {
    source: Source,
    protocols: Array<Protocol>
}

export async function connect(requests: Array<ConnectionRequest>): Promise<Array<Wallet>> {

}