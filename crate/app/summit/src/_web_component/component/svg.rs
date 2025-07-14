use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct BlockyArrowRightProps {
    pub w: String,
    pub h: String,
	pub color: (String, String),
	pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn BlockyArrowRight(props: BlockyArrowRightProps) -> Element {
    rsx!(
        svg {
			style: format!(
				r#"
					width: {};
					height: {};
					{}
				"#,
				props.w,
				props.h,
				props.style.unwrap_or_default()
			),
			view_box: "0 0 200 200",
			fill: "none",
			xmlns: "http://www.w3.org/2000/svg",
			g {
				clip_path: "url(#clip0_238_1313)",
				path {
					fill_rule: "evenodd",
					clip_rule: "evenodd",
					d: "M4.37114e-06 2.76541e-06L7.54022e-06 50L100 100L2.18557e-06 150L0 200L100 150L100 200L200 150V100V50L100 0V50L4.37114e-06 2.76541e-06ZM100 50L100 100L100 150L200 100L100 50Z",
					fill: "url(#paint0_linear_238_1313)"
				}
			}
			defs {
				linearGradient {
					id: "paint0_linear_238_1313",
					x1: "14",
					y1: "26",
					x2: "179",
					y2: "179.5",
					gradient_units: "userSpaceOnUse",
					stop {
						offset: "0",
						stop_color: props.color.0
					}
					stop {
						offset: "1",
						stop_color: props.color.1
					}
				}
				clipPath {
					id: "clip0_238_1313",
					rect {
						width: "200",
						height: "200",
						fill: "white"
					}
				}
			}
		}
	)
}

