use super::*;
use crate::web::component::layout::vertical_page;
use crate::web::component::nav;
use crate::web::component::icon;

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
                        height: 400px;
                        border-radius: 2px;
                        border-width: 1px;
                        border-style: solid;
                        border-color: {color::CARBON};
                        color: white;
                        font-family: br cobane;
                        font-size: 5em;
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
                        icon::Icon { size: "15px", url: asset!("asset/icon/social/discord.svg") }
                        icon::Icon { size: "15px", url: asset!("asset/icon/social/github.svg") }
                        icon::Icon { size: "15px", url: asset!("asset/icon/social/medium.svg") }
                        icon::Icon { size: "15px", url: asset!("asset/icon/social/telegram.svg") }
                        icon::Icon { size: "15px", url: asset!("asset/icon/social/x.svg") }
                    }
                    div {
                        style: r#"
                            display: flex;
                            flex-direction: row;
                            justify-content: end;
                        "#
                    }
                }
            }
            vertical_page::Section {
                "Hi"
            }
        }
    }
}