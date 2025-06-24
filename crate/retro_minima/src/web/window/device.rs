use super::*;

#[derive(Clone)]
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
        w if w >= 2560f64 => device.set(Device::Laptop4K),
        w if w >= 1440f64 => device.set(Device::LaptopL),
        w if w >= 1024f64 => device.set(Device::Laptop),
        w if w >= 768f64 => device.set(Device::Tablet),
        w if w >= 425f64 => device.set(Device::MobileL),
        w if w >= 375f64 => device.set(Device::MobileM),
        _ => device.set(Device::Mobile)
    }

    device
}