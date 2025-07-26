import * as reliq from "reliq";

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

export const GROUP_PHYSICAL_MODE: ReadonlyArray<ConnectionMode> = [
    ConnectionMode.WebUsb,
    ConnectionMode.WebHid,
    ConnectionMode.Bluetooth,
    ConnectionMode.NFC
];