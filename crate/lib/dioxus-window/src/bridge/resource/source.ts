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




export namespace Source {



    export function from_injected_flags(): Option<Source> {
        // @ts-ignore 
        if ((window as any).ethereum?.isMetaMask) {
            return Some(Source.Metamask);
        }

        // @ts-ignore
        if ((window as any).ethereum?.isRabby) {
            return Some(Source.Rabby);
        }
        
        // @ts-ignore
        if ((window as any).ethereum?.isCoinbaseWallet) {
            return Some(Source.Coinbase);
        }

        // @ts-ignore
        if ((window as any).ethereum?.isBraveWallet) {
            return Some(Source.Brave);
        }

        return None;
    }

    export function from_eip6963(data: Eip6963Data): Option<Source> {
        switch (data.rdns) {
            case "io.metamask": 
                return Some(Source.Metamask);
            case "com.coinbase.wallet": 
                return Some(Source.Coinbase);
            case "fi.brave": 
                return Some(Source.Brave);
            case "xyz.rabby": 
                return Some(Source.Rabby);
            default: return None;
        }
    }

    export function from_polkadot_meta_source(source: string): reliq.Option<Source> {
        switch (source.toLowerCase()) {
            case "talisman": 
                return Some(Source.Talisman);
            case "subwallet-js": 
                return Some(Source.SubWallet);
            case "polkadot-js": 
                return Some(Source.PolkadotJs);
            default: 
                return None;
        }
    }

    export function from_wallet_name(name: string): reliq.Option<Source> {
        switch (name.toLowerCase()) {
            case "ledger live": 
                return reliq.Some(Source.Ledger);
            case "trezor suite": 
                return reliq.Some(Source.Trezor);
            case "keystone": 
                return Some(Source.Keystone);
            case "metamask": 
                return Some(Source.Metamask);
            case "phantom": 
                return Some(Source.Phantom);
            case "trust wallet": 
                return Some(Source.TrustWallet);
            case "nova wallet": 
                return Some(Source.NovaWallet);
            case "rabby": 
                return Some(Source.Rabby);
            case "brave": 
                return Some(Source.Brave);
            case "coinbase wallet": 
                return Some(Source.Coinbase);
            case "xdefi": 
                return Some(Source.XDEFI);
            case "exodus": 
                return Some(Source.Exodus);
            case "frame": 
                return Some(Source.Frame);
            default: 
                return None;
        }
    }
}
