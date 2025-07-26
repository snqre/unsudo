import * as reliq from "reliq";

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
    Coinbase,
    XDEFI,
    Exodus,
    Frame
}

export type Eip6963Data = {
    name: string,
    rdns: string
}

export function from_injected_flags(): reliq.Option<Source> {
    // @ts-ignore
    if ((window as any).ethereum?.isMetaMask) {
        return reliq.Some(Source.Metamask);
    }

    // @ts-ignore
    if ((window as any).ethereum?.isRabby) 
        return reliq.Some(Source.Rabby);

    // @ts-ignore
    if ((window as any).ethereum?.isCoinbaseWallet) 
        return reliq.Some(Source.Coinbase);

    // @ts-ignore
    if ((window as any).ethereum?.isBraveWallet) 
        return reliq.Some(Source.Brave);

    return reliq.None;
}

export function from_eip6963(data: Eip6963Data): reliq.Option<Source> {
    switch (data.rdns) {
        case "io.metamask": 
            return reliq.Some(Source.Metamask);
        case "com.coinbase.wallet": 
            return reliq.Some(Source.Coinbase);
        case "fi.brave": 
            return reliq.Some(Source.Brave);
        case "xyz.rabby": 
            return reliq.Some(Source.Rabby);
        default: return reliq.None;
    }
}

export function from_polkadot_meta_source(source: string): reliq.Option<Source> {
    switch (source.toLowerCase()) {
        case "talisman": 
            return reliq.Some(Source.Talisman);
        case "subwallet-js": 
            return reliq.Some(Source.SubWallet);
        case "polkadot-js": 
            return reliq.Some(Source.PolkadotJs);
        default: 
            return reliq.None;
    }
}

export function from_wallet_name(name: string): reliq.Option<Source> {
    switch (name.toLowerCase()) {
        case "ledger live": 
            return reliq.Some(Source.Ledger);
        case "trezor suite": 
            return reliq.Some(Source.Trezor);
        case "keystone": 
            return reliq.Some(Source.Keystone);
        case "metamask": 
            return reliq.Some(Source.Metamask);
        case "phantom": 
            return reliq.Some(Source.Phantom);
        case "trust wallet": 
            return reliq.Some(Source.TrustWallet);
        case "nova wallet": 
            return reliq.Some(Source.NovaWallet);
        case "rabby": 
            return reliq.Some(Source.Rabby);
        case "brave": 
            return reliq.Some(Source.Brave);
        case "coinbase wallet": 
            return reliq.Some(Source.Coinbase);
        case "xdefi": 
            return reliq.Some(Source.XDEFI);
        case "exodus": 
            return reliq.Some(Source.Exodus);
        case "frame": 
            return reliq.Some(Source.Frame);
        default: 
            return reliq.None;
    }
}