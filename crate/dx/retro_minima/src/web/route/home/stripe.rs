use super::*;

#[component]
pub fn Stripe() -> Element {
	let device: Signal<_> = win::use_device();
    
    rsx! {
		match device() {
			win::Device::Laptop4K | win::Device::LaptopL | win::Device::Laptop | win::Device::Tablet => rsx! {
				div {
                    style: r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        width: 100%;
                        height: 5px;
                        background: linear-gradient(to right, {color::SILVER}, {color::STEEL});
                    "#
                }
			},
			_ => rsx! {}
		}
    }
}