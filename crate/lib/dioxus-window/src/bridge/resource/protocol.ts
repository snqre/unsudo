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

export const UTXO: ReadonlyArray<Protocol> = [
    Protocol.Bitcoin,
    Protocol.Dogecoin,
    Protocol.Litecoin,
    Protocol.Zcash
];

export const EVM: ReadonlyArray<Protocol> = [
    Protocol.Ethereum,
    Protocol.Arbitrum,
    Protocol.Optimism,
    Protocol.BinanceSmartChain,
    Protocol.Polygon,
    Protocol.Avalanche
];

export const SUBSTRATE: ReadonlyArray<Protocol> = [
    Protocol.Polkadot,
    Protocol.Kusama
];

export function is_utxo(protocol: Protocol): boolean {
    return UTXO.includes(protocol);
}

export function is_substrate(protocol: Protocol): boolean {
    return SUBSTRATE.includes(protocol);
}

export function is_evm(protocol: Protocol): boolean {
    return EVM.includes(protocol);
}