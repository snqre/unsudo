import * as reliq from "reliq";
import * as api from "@polkadot/api";
import * as ext from "@polkadot/extension-dapp";
import * as helper_type from "@polkadot/util/types";
import * as helper from "@polkadot/util";
import * as crypto from "@polkadot/util-crypto";











export type Wallets = {
    name: string,
    connection_mode?: ConnectionMode,
    accounts?: Account
}

export type Account = {
    address: string,
    key_pair: KeyPair,
    name: reliq.Option<string>,
    protocol?: Protocol,
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






abstract class Provider {
    public abstract name: string;
    public abstract source: Source;
    public abstract connection_mode: ConnectionMode;

}

class Metamask implements Provider {
    public readonly name: string = "Metamask";
    public readonly source: Source = Source.Metamask;
    public readonly connection_mode: ConnectionMode = ConnectionMode.Extension;

    public constructor() {

    }
}


class Talisman implements Provider {

}



async function connect_to_metamask(protocols: Array<Protocol>): Promise<Wallet> {
    if (!(window as any).ethereum || !(window as any).ethereum.isMetaMask) {
        throw new Error("MetaMask is not installed");
    }
    let ethereum = (window as any).ethereum;
    


  // Request account access (this will prompt the user)
  const addresses: string[] = await ethereum.request({ method: "eth_requestAccounts" });

  // Get current chain ID to filter protocols
  const chainIdHex: string = await ethereum.request({ method: "eth_chainId" });
  const chainId = parseInt(chainIdHex, 16);

  // Map chainId to Protocol enum(s)
  // This is simplified, you can expand this mapping

    let chain_id_to_procol = new Map<number, Protocol>([
        [1, Protocol.Ethereum],
        [137, Protocol.Polygon],

    ]);

  const chainIdToProtocol = new Map<number, Protocol>([
    [1, Protocol.Ethereum], // Mainnet
    [137, Protocol.Polygon],
    [56, Protocol.BinanceSmartChain],
    [10, Protocol.Optimism],
    [42161, Protocol.Arbitrum],
    // add other EVM chains here as needed
  ]);

  const connectedProtocol = chainIdToProtocol.get(chainId);

  if (!connectedProtocol || !protocols.includes(connectedProtocol)) {
    throw new Error(`Connected chain (ID: ${chainId}) is not in requested protocols`);
  }

  // Build accounts array
  const accounts: Account[] = addresses.map(address => ({
    address,
    key_pair: KeyPair.Ethereum,
    name: null, // MetaMask doesn't provide account nicknames by default
    protocol: connectedProtocol,
    source: Source.Metamask
  }));

  const wallet: Wallet = {
    name: "MetaMask",
    connection_mode: ConnectionMode.Extension,
    accounts
  };

  return wallet;
}





