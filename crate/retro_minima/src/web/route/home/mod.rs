use super::*;
use crate::web::component::*;
use crate::web;

leak!(
    hero_case
    hero_heading
    social_icon_group
    social_icon
    stripe
    svg_shape_0
    svg_shape_1
);

#[component]
pub fn Route() -> Element {
    rsx! {
        layout::VerticalPage {
            style: r#"
                background: {color::OBSIDIAN};
            "#,
            layout::VerticalPageSection {
                top: rsx! {
                    Stripe {}
                    navbar::Root {
                        left: rsx! {
                            
                        },
                        layout::Row {
                            style: r#"
                                min-width: 100%;
                                gap: {web::sequence(2)}px;
                            "#,
                            interface::SimpleButton { "Get Started" }
                            interface::SimpleButton { "Community" }
                            interface::SimpleButton { "Whitepaper" }
                        }
                    },
                    layout::Col {
                        style: r#"
                            width: 100%;
                            height: 100%;
                            padding-left: {web::sequence(2)}px;
                            padding-right: {web::sequence(2)}px;
                        "#,
                        component::window_frame::Rect {
                            style: r#"
                                width: 100%;
                                min-width: 0%;
                                max-width: {web::sequence(11)}px;
                                height: 100%;
                                min-height: {web::sequence(8)}px;
                                max-height: {web::sequence(8)}px;
                                padding: {web::sequence(1)}px;
                            "#,
                            layout::AutoRow {
                                style: r#"
                                    justify-content: space-between;
                                "#,
                                layout::Col {}
                                layout::Col {
                                    style: r#"
                                        height: 100%;
                                        min-height: 100%;
                                        max-height: 100%;
                                    "#,
                                    layout::Col {
                                        style: r#"
                                            justify-content: space-between;
                                            height: 100%;
                                            min-height: 100%;
                                            max-height: 100%;
                                        "#,
                                        layout::Row {}
                                        SvgShape0 {
                                            w: "{web::sequence(7)}px",
                                            h: "{web::sequence(7)}px",
                                            color_0: color::SILVER,
                                            color_1: color::STEEL
                                        }
                                        layout::Row {
                                            style: r#"
                                                width: 100%;
                                                min-width: 100%;
                                                max-width: 100%;
                                                justify-content: end;
                                                font-family: br cobane;
                                                font-size: {web::sequence(2)};
                                                font-weight: normal;
                                                color: {color::SILVER};
                                            "#,
                                            "Learn More"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}




/**
 *                             layout::Row {
                                style: r#"
                                    width: 100%;
                                    min-width: 100%;
                                    max-width: 100%;
                                    height: 100%;
                                    min-height: 100%;
                                    max-height: 100%;
                                    justify-content: space-between;
                                "#,
                                layout::Col {
                                    style: r#"
                                        height: 100%;
                                        min-height: 100%;
                                        max-height: 100%;
                                        justify-content: start;
                                        align-items: start;
                                        flex-grow: 1;
                                        flex-shrink 1;
                                    "#,
                                    SocialIconGroup {
                                        SocialIcon { size: "{web::sequence(2u16)}px", to: "/", url: asset!("asset/icon/social/discord.svg") }
                                        SocialIcon { size: "{web::sequence(2u16)}px", to: "/", url: asset!("asset/icon/social/github.svg") }
                                        SocialIcon { size: "{web::sequence(2u16)}px", to: "/", url: asset!("asset/icon/social/medium.svg") }
                                        SocialIcon { size: "{web::sequence(2u16)}px", to: "/", url: asset!("asset/icon/social/telegram.svg") }
                                        SocialIcon { size: "{web::sequence(2u16)}px", to: "/", url: asset!("asset/icon/social/x.svg") }
                                    }
                                    layout::Col {
                                        layout::Col {
                                            style: r#"
                                                align-items: start;
                                            "#,
                                            layout::Row {
                                                style: r#"
                                                    font-family: br cobane;
                                                    font-size: {web::sequence(6u16)}px;
                                                    font-weight: bold;
                                                    color: {color::SILVER};
                                                    justify-content: start;
                                                "#,
                                                "Empower the Future"
                                            }
                                            layout::Row {
                                                style: r#"
                                                    font-family: br cobane;
                                                    font-size: {web::sequence(3u16)}px;
                                                    font-weight: normal;
                                                    color: {color::interpolate((color::OBSIDIAN, color::SILVER), 0.75f32)};
                                                    justify-content: start;
                                                "#,
                                                "Some sub heading..."
                                            }
                                        }
                                        layout::Row {
    
                                        }
                                    }
                                }
                                layout::Col {
                                    style: r#"
                                        width: 100%;
                                        min-width: 0%;
                                        max-width: 100%;
                                        height: 100%;
                                        min-height: 100%;
                                        max-height: 100%;
                                        flex-grow: 1;
                                        flex-shrink 1;
                                        align-items: end;
                                    "#,
                                    SvgShape0 {
                                        w: "{web::sequence(7u16)}px",
                                        h: "{web::sequence(7u16)}px",
                                        color_0: color::SILVER,
                                        color_1: color::STEEL
                                    }
                                }
                            }
 */
fn t() {}