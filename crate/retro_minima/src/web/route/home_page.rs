use super::*;
use crate::web::component::counter;
use crate::web::component::layout::vertical_page;
use crate::web::component::nav;
use crate::web::component::icon;
use crate::web::component::button;
use crate::web::window;
use crate::web::easing;

#[component]
pub fn HomePage() -> Element {
	let device: Signal<window::Device> = window::use_device();
	rsx! {
		match device() {
			window::Device::Laptop4K | window::Device::LaptopL | window::Device::Laptop | window::Device::Tablet => rsx! {
				div {
					style: format! {
						r#"
                            display: flex;
                            flex-direction: row;
                            justify-content: center;
                            align-items: center;
                            width: 100%;
                            height: 10px;
                            background: linear-gradient(to right, {}, {});
                        "#,
						color::SILVER,
						color::STEEL
					}
				}
			},
			_ => rsx! {}
		}
		vertical_page::VerticalPage {
			style: r#"
                background: {color::OBSIDIAN};
                color: white;
            "#,
			vertical_page::Section {
				navbar: rsx! {
					nav::Nav {}
				},
				div {
					style: r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: space-between;
                        align-items: center;
                        width: 100%;
                        height: 100%;
                    "#,
					div { 
						style: r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: space-between;
                            align-items: center;
                            width: 100%;
                            max-width: 1440px;
                            height: 400px;
                            border-radius: 2px;
                            border-width: 1px;
                            border-style: solid;
                            border-color: {color::CARBON};
                        "#,
						div { 
							style: r#"
                                display: flex;
                                flex-direction: row;
                                justify-content: start;
                                align-items: center;
                                width: 100%;
                                padding: 10px;
                                gap: 10px;
                            "#,
							icon::Icon { size: "20px", url: asset!("asset/icon/social/discord.svg") }
							icon::Icon { size: "20px", url: asset!("asset/icon/social/github.svg") }
							icon::Icon { size: "20px", url: asset!("asset/icon/social/medium.svg") }
							icon::Icon { size: "20px", url: asset!("asset/icon/social/telegram.svg") }
							icon::Icon { size: "20px", url: asset!("asset/icon/social/x.svg") }
						}
						div {
							style: r#"
                                display: flex;
                                flex-direction: row;
                                justify-content: end;
                                align-items: center;
                                width: 100%;
                                height: 100%;
                            "#,
							div {
								style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    justify-content: center;
                                    align-items: start;
                                    flex-grow: 1;
                                    flex-basis: 4;
                                    width: 100%;
                                    height: 100%;
                                    padding: 10px;
                                    gap: 50px;
                                "#,
								div {
									style: r#"
                                        display: flex;
                                        flex-direction: column;
                                        justify-content: center;
                                        align-items: center;
                                        gap: 20px;
                                    "#,
									div {
										class: r#"
                                            conditional
                                            
                                        "#
									}
									div {
										style: format! {
											r#"
                                                display: flex;
                                                flex-direction: row;
                                                justify-content: start;
                                                align-items: center;
                                                width: 100%;
                                                font-family: br cobane;
                                                font-size: {};
                                                font-weight: normal;
                                                color: {};
                                            "#,
											match device() {
												window::Device::Laptop4K => "5em",
												window::Device::LaptopL => "4em",
												window::Device::Laptop => "3em",
												_ => "3em"
											},
											color::SILVER
										},
										"Empower communities to do the impossible."
									}
									div {
										style: r#"
                                            display: flex;
                                            flex-direction: row;
                                            justify-content: start;
                                            align-items: center;
                                            width: 100%;
                                            font-family: br cobane;
                                            font-size: 2em;
                                            font-weight: normal;
                                            color: {color::SILVER};
                                        "#,
										"Decentralized autonomus protocols."
									}
								}
								div {
									class: r#"
                                        conditional 
                                        not-mobile-s 
                                        not-mobile-m 
                                        not-mobile-l 
                                        not-tablet"#,
									div {
										style: r#"
                                            display: flex;
                                            flex-direction: row;
                                            justify-content: start;
                                            align-items: center;
                                            width: 100%;
                                            gap: 20px;
                                        "#,
										button::RollingButton {
											child_on_idle: rsx! {
												"Create"
											},
											child_on_hover: rsx! {
												div {
													style: r#"
                                                        width: 15px;
                                                        aspect-ratio: 1 / 1;
                                                        background-image: url({asset!("asset/icon/chev_r.svg")});
                                                        background-position: center;
                                                        background-size: contain;
                                                        background-repeat: no-repeat;
                                                    "#
												}
											}
										}
										button::RollingButton {
											child_on_idle: rsx! {
												"Join"
											},
											child_on_hover: rsx! {
												div {
													style: r#"
                                                        width: 15px;
                                                        aspect-ratio: 1 / 1;
                                                        background-image: url({asset!("asset/icon/chev_r.svg")});
                                                        background-position: center;
                                                        background-size: contain;
                                                        background-repeat: no-repeat;
                                                    "#
												}
											}
										}
									}
								}
							}
							div {
								class: r#"
                                    not-mobile-s
                                    not-mobile-m
                                    not-mobile-l
                                    not-tablet
                                "#,
								div {
									style: r#"
                                        display: flex;
                                        flex-direction: row;
                                        justify-content: end;
                                        align-items: center;
                                        flex: 1;
                                        width: 100%;
                                        height: 100%;
                                        padding: 20px;
                                    "#,
									HeroImage {}
								}
							}
						}
						div {
							style: r#"
                                display: flex;
                                flex-direction: row;
                                justify-content: end;
                                align-items: center;
                                width: 100%;
                                padding: 10px;
                            "#,
							button {
								style: r#"
                                    all: unset;
                                    display: flex;
                                    flex-direction: row;
                                    justify-content: center;
                                    align-items: center;
                                    font-family: br cobane;
                                    font-size: 1em;
                                    font-weight: normal;
                                    cursor: pointer;
                                    color: {color::SILVER};
                                "#,
								"Learn More"
							}
						}
					}
					div {
						style: r#"
                            display: flex;
                            flex-direction: row;
                            justify-content: center;
                            align-items: center;
                            width: 100%;
                            height: 200px;
                        "#,
						TinyCard {
							heading: rsx! { "Centralized Power Breeds Corruption" },
							sub_heading: rsx! { "$4.5 Trillion Lost to Fraud Each Year" },
							body: rsx! { "Traditional organizations concentrate control, enabling insider fraud, manipulation, and unchecked authority. DAOs automate governance transparently — reducing fraud, boosting trust, and eliminating gatekeepers." }
						}
					}
				}
			}
			vertical_page::Section {
				div {
					class: "float reveal",
					style: r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: center;
                        width: 400px;
                        height: 500px;
                        border-top-left-radius: 200px;
                        border-top-right-radius: 200px;
                        border-bottom-left-radius: 2px;
                        border-bottom-right-radius: 2px;
                        border-width: 1px;
                        border-style: solid;
                        border-color: {color::CARBON};
                        font-family: br cobane;
                        font-size: 7em;
                        font-weight: normal;
                    "#,
					counter::Counter {
						start: 0.0f32,
						end: 74.0f32,
						duration: Duration::from_secs(10),
						easing: easing::ease_out_quint
					}
				}
			}
		}
	}
}



#[component]
fn Conta() -> Element {
	rsx! {

	}
}




#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
struct TinyCardProps {
	pub style: Option<String>,
	pub heading: Option<Element>,
	pub heading_style: Option<String>,
	pub sub_heading: Option<Element>,
	pub sub_heading_style: Option<String>,
	pub body: Option<Element>,
	pub body_style: Option<String>
}

#[component]
fn TinyCard(props: TinyCardProps) -> Element {
	let device: Signal<window::Device> = window::use_device();
	rsx! {
		div {
			style: r#"
				display: flex;
				flex-direction: row;
				justify-content: start;
				align-items: center;
				min-width: 100%;
			"#,
			match device() {
				window::Device::Laptop4K | window::Device::LaptopL | window::Device::Laptop | window::Device::Tablet => rsx! {
					div {
						style: r#"
							display: flex;
							flex-direction: row;
							justify-content: center;
							align-items: center;
							padding: 20px;
						"#,
						EyeShutShape {}
					}
				},
				_ => rsx! {}
			}
			div {
				style: format! {
					r#"
						display: flex;
						flex-direction: column;
						justify-content: center;
						align-items: start;
						max-width: 1440px
						min-width: 200px;
						min-height: 100%;
						flex: 1;
						padding: 10px;
						{}
					"#,
					props.style.to_owned().unwrap_or_default()
				},
				match device() {
					window::Device::MobileL | window::Device::MobileM | window::Device::Mobile => rsx! {
						div {
							style: r#"
								display: flex;
								flex-direction: row;
								justify-content: center;
								align-items: center;
								min-width: 100%;
								margin-bottom: 10px;
							"#,
							EyeShutShape {}
						}
					},
					_ => rsx! {}
				}
				div {
					style: format! {
						r#"
							opacity: 0;
							display: flex;
							flex-direction: row;
							justify-content: start;
							align-items: center;
							min-width: 100%;
							font-family: br cobane;
							font-size: 2em;
							font-weight: bold;
							color: {};
							margin-bottom: 10px;
							animation: faulty-neon 1s linear;
							animation-delay: 0s;
							animation-fill-mode: forwards;
							{}
						"#,
						color::SILVER,
						props.heading_style.to_owned().unwrap_or_default()
					},
					{ props.heading }
				}
				div {
					style: format! {
						r#"
							opacity: 0;
							display: flex;
							flex-direction: row;
							justify-content: start;
							align-items: center;
							min-width: 100%;
							font-family: br cobane;
							font-size: 1.5em;
							font-weight: normal;
							color: {};
							margin-bottom: 20px;
							animation: faulty-neon 1s linear;
							animation-delay: 0.2s;
							animation-fill-mode: forwards;
							{}
						"#,
						color::interpolate((color::OBSIDIAN, color::SILVER), 0.75f32),
						props.sub_heading_style.to_owned().unwrap_or_default()
					},
					{ props.sub_heading }
				}
				div {
					style: format! {
						r#"
							opacity: 0;
							display: flex;
							flex-direction: row;
							justify-content: start;
							align-items: center;
							min-width: 100%;
							font-family: br cobane;
							font-size: 1em;
							font-weight: normal;
							color: {};
							line-height: 1.5;
							animation: faulty-neon 1s linear;
							animation-delay: 0.5s;
							animation-fill-mode: forwards;
							{}
						"#,
						color::SILVER,
						props.body_style.to_owned().unwrap_or_default()
					},
					{ props.body }
				}
			}
		}
	}
}




#[component]
fn EyeShutShape() -> Element {
	rsx! {
        svg {
			style: r#"
				min-width: 50px;
				max-width: 50px;
				aspect-ratio: 1 / 1;
			"#,
            view_box: "0 0 200 200",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                clip_path: "url(#clip0_105_520)",
                path {
                    d: "M200 99.5882C200 99.5882 155.228 150.176 100 150.176C44.7715 150.176 0 99.5882 0 99.5882C0 99.5882 44.7715 49 100 49C155.228 49 200 99.5882 200 99.5882Z",
                    fill: "url(#paint0_linear_105_520)"
                }
            }
            defs {
                linearGradient {
                    id: "paint0_linear_105_520",
                    x1: "177",
                    y1: "49",
                    x2: "124.191",
                    y2: "164.777",
                    gradient_units: "userSpaceOnUse",
                    stop {
                        offset: "0",
                        stop_color: color::SILVER
                    }
                    stop {
                        offset: "1",
                        stop_color: color::SILVER
                    }
                }
                clipPath {
                    id: "clip0_105_520",
                    rect {
                        width: "200",
                        height: "200",
                        fill: color::SILVER
                    }
                }
            }
        }
	}
}



#[component]
pub fn Slide() -> Element {
	rsx! {
		div {
			style: r#"
                display: flex;
                flex-direction: column;
                justify-content: start;
                align-items: center;
                width: 100px;
                height: 50px;
            "#,
			counter::Counter {
				start: 0.0f32,
				end: 57.0f32,
				duration: Duration::from_secs(3),
				easing: easing::ease_out_cubic
			}
			div {
				"of global investors feel like companies ."
			}
		}
	}
}



pub mod laptop_version {
	use super::*;
	
	#[component]
	pub fn LaptopVersion() -> Element {
		rsx! {
			div {
				class: r#"
                    conditional
                    not-tablet
                    not-mobile-l
                    not-mobile-m
                    not-mobile-s
                "#,
				vertical_page::VerticalPage {
					vertical_page::Section {
						navbar: rsx! {
							nav::Nav {}
						},
						div {
							style: r#"
                                display: flex;
                                flex-direction: column;
                                justify-content: center;
                                align-items: center;
                                min-width: 100%;
                                max-width: 100%;
                                min-height: 100%;
                                max-height: 100%;
                            "#
						}
					}
				}
			}
		}
	}
}


#[component]
fn HeroImage() -> Element {
	rsx! {
		svg {
			style: r#"
                width: 250px;
                height: 100%;
            "#,
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
						stop_color: color::SILVER
					}
					stop {
						offset: "1",
						stop_color: color::STEEL
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
	}
}



pub mod container {
	use super::*;

	#[derive(Clone)]
	#[derive(PartialEq)]
	#[derive(Props)]
	pub struct ArcProps {
		pub style: Option<String>,
		pub children: Option<Element>
	}

	#[component]
	pub fn Arc(props: ArcProps) -> Element {
		rsx! {
			div {
				style: r#"
					display: flex;
					flex-direction: column;
					justify-content: center;
					align-items: center;
					min-width: 400px;
					min-height: 500px;
					border-top-left-radius: 200px;
					border-top-right-radius: 200px;
					border-bottom-left-radius: 2px;
					border-bottom-right-radius: 2px;
					border-width: 1px;
					border-style: solid;
					border-color: {color::CARBON};
					{props.style.to_owned().unwrap_or_default()}
				"#
			}
		}
	}
}