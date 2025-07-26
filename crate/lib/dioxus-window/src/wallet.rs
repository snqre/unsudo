use wasm_bindgen::prelude::*;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Wallet {
    pub name: Option<String>,
    pub connection_mode: Option<ConnectionMode>,
    pub accounts: Option<Vec<Account>>
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Account {
    pub address: String,
    pub key_pair: KeyPair,
    pub name: Option<String>,
    pub protocol: Option<Protocol>,
    pub source: Option<Source>
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum Source {
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

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum KeyPair {
    Ed25519,
    Sr25519,
    Secp256k1,
    Ecdsa,
    Ethereum
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum Protocol {
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

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum ConnectionMode {
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

pub async fn request_connection_to_polkadot_wallets() -> Result<Vec<Wallet>, JsValue> {
    match bridge::request_connection_to_polkadot_wallets().await {
        Ok(js_val) => serde_wasm_bindgen::from_value(js_val)
            .map_err(|e| JsValue::from_str(&format!("Deserialization failed: {:?}", e))),
        Err(err) => Err(err),
    }
}

mod bridge {
    use super::*;

    #[wasm_bindgen(module = "/src/wallet.ts")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn request_connection_to_polkadot_wallets() -> Result<JsValue, JsValue>;
    }
}