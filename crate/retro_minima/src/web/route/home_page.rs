use super::*;
use crate::web::component::layout::vertical_page;
use crate::web::component::nav;
use crate::web::component::icon;
use crate::web::component::button;

#[component]
pub fn HomePage() -> Element {
    rsx! {
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
                        justify-content: start;
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
                                        style: r#"
                                            display: flex;
                                            flex-direction: row;
                                            justify-content: start;
                                            align-items: center;
                                            width: 100%;
                                            font-family: br cobane;
                                            font-size: 4em;
                                            font-weight: normal;
                                            color: {color::SILVER};
                                        "#,
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
                            width: 100%;
                            height: 200px;
                        "#,
                    }
                }
            }
            vertical_page::Section {
                "Hi"
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