use super::*;

#[component]
pub fn Stripe() -> Element {
	let device: Signal<web::window::Device> = web::window::use_device();
    rsx! {
		match device() {
			web::window::Device::Laptop4K | web::window::Device::LaptopL | web::window::Device::Laptop | web::window::Device::Tablet => rsx! {
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