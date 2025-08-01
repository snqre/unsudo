import {Option, Some, None} from "reliq";
import {Protocol} from "./protocol";

export enum KeyPair {
    Ed25519,
    Sr25519,
    Secp256k1,
    Ecdsa,
    Ethereum
}

export namespace KeyPair {

    export function from_string(s: string): Option<KeyPair> {
        switch (s.toLowerCase()) {
            case "ed25519": 
                return Some(KeyPair.Ed25519);
            case "sr25519": 
                return Some(KeyPair.Sr25519);
            case "secp256k1": 
                return Some(KeyPair.Secp256k1);
            case "ecdsa": 
                return Some(KeyPair.Ecdsa);
            case "ethereum": 
                return Some(KeyPair.Ethereum);
            default: 
                return None;
        }
    }

    export function for_protocol(protocol: Protocol): Option<KeyPair> {
        switch (protocol) {
            case Protocol.Polkadot:
            case Protocol.Kusama:
                return Some(KeyPair.Sr25519);
            case Protocol.Ethereum:
            case Protocol.Arbitrum:
            case Protocol.Optimism:
            case Protocol.BinanceSmartChain:
            case Protocol.Polygon:
                return Some(KeyPair.Ethereum);
            case Protocol.Solana:
                return Some(KeyPair.Ed25519);
            case Protocol.Bitcoin:
                return Some(KeyPair.Secp256k1);
            default:
                return None;
        }
    }
}