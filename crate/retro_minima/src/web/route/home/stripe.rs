use super::*;

#[component]
pub fn Stripe() -> Element {
	let device: Signal<window::Device> = window::use_device();
    rsx! {
		match device() {
			window::Device::Laptop4K | window::Device::LaptopL | window::Device::Laptop | window::Device::Tablet => rsx! {
				div {
                    style: r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        width: 100%;
                        height: 10px;
                        background: linear-gradient(to right, {color::SILVER}, {color::STEEL});
                    "#
                }
			},
			_ => rsx! {}
		}
    }
}