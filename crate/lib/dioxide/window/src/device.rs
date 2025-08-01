use super::*;

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum Device {
    Laptop4K,
    LaptopL,
    Laptop,
    Tablet,
    MobileL,
    MobileM,
    Mobile
}

pub fn use_device() -> Signal<Device> {
    let w: Signal<_> = use_w();
    let mut device: Signal<_> = use_signal(|| Device::Laptop);

    match w() {
        w if w >= 2560.0 => device.set(Device::Laptop4K),
        w if w >= 1440.0 => device.set(Device::LaptopL),
        w if w >= 1024.0 => device.set(Device::Laptop),
        w if w >= 768.0 => device.set(Device::Tablet),
        w if w >= 425.0 => device.set(Device::MobileL),
        w if w >= 375.0 => device.set(Device::MobileM),
        _ => device.set(Device::Mobile)
    }

    device
}