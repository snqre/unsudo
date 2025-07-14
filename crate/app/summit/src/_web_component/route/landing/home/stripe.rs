use super::*;

#[component]
pub fn Stripe() -> Element {
	let device: Signal<_> = dioxus_window::use_device();
    
    rsx!(
        
    )

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