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
                navbar: rsx! { nav::Nav {} },
                div { 
                    style: r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: space-between;
                        align-items: center;
                        width: 100%;
                        max-width: clamp(768px, 100%, 1440px);
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
                                align-items: center;
                                flex: 1;
                                width: 100%;
                                height: 100%;
                                padding: 20px;
                                gap: 20px;
                            "#,
                            div {
                                style: r#"
                                    display: flex;
                                    flex-direction: row;
                                    justify-content: start;
                                    align-items: center;
                                    width: 100%;
                                    font-family: br cobane;
                                    font-size: clamp(2em, 3em, 6em);
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
                                    font-size: 1em;
                                    font-weight: normal;
                                    color: {color::SILVER};
                                "#,
                                "Decentralized autonomus protocols."
                            }
                            div {
                                style: r#"
                                    display: flex;
                                    flex-direction: row;
                                    justify-content: start;
                                    align-items: center;
                                    width: 100%;
                                    gap: 10px;
                                "#,
                                button::Button { "Create a DAO" }
                                button::Button { "Join" }
                            }
                        }
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;
                                justify-content: center;
                                align-items: center;
                                flex: 1;
                                width: 100%;
                                height: 100%;
                                padding: 20px;
                            "#,
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
            }
            vertical_page::Section {
                "Hi"
            }
        }
    }
}
