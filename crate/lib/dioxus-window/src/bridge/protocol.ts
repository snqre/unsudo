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

export const GROUP_UTXO: ReadonlyArray<Protocol> = [
    Protocol.Bitcoin,
    Protocol.Dogecoin,
    Protocol.Litecoin,
    Protocol.Zcash
];

export const GROUP_EVM: ReadonlyArray<Protocol> = [
    Protocol.Ethereum,
    Protocol.Arbitrum,
    Protocol.Optimism,
    Protocol.BinanceSmartChain,
    Protocol.Polygon,
    Protocol.Avalanche
];

export const GROUP_SUBSTRATE: ReadonlyArray<Protocol> = [
    Protocol.Polkadot,
    Protocol.Kusama
];

export function is_utxo(protocol: Protocol): boolean {
    return GROUP_UTXO.includes(protocol);
}

export function is_substrate(protocol: Protocol): boolean {
    return GROUP_SUBSTRATE.includes(protocol);
}

export function is_evm(protocol: Protocol): boolean {
    return GROUP_EVM.includes(protocol);
}