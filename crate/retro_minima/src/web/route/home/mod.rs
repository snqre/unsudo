use super::*;
use crate::web::component::*;
use crate::web;

leak!(
    hero_heading
    horizontal_carousel
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
                background: rsx! {

                },
                top: rsx! {
                    Stripe {}
                    navbar::Container {
                        left: rsx! {
                            navbar::Logo {}
                            layout::Row {
                                style: r#"
                                    min-width: 100%;
                                    gap: {web::sequence(2)}px;
                                "#,
                                intf::Button {                                                 style: r#"
                                font-size: {web::sequence(1)}px;
                            "#,"Get Started" }
                                intf::Button {                                                 style: r#"
                                font-size: {web::sequence(1)}px;
                            "#,"Community" }
                                intf::Button {                                                 style: r#"
                                font-size: {web::sequence(1)}px;
                            "#,"Whitepaper" }
                            }
                        }
                    },
                    layout::Col {
                        style: r#"
                            width: 100%;
                            height: 100%;
                            padding-left: {web::sequence(2)}px;
                            padding-right: {web::sequence(2)}px;
                        "#,
                        component::window::Rect {
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
                                            gap: {web::sequence(2)}px;
                                        "#,
                                        layout::Row {
                                            style: r#"
                                                width: 100%;
                                                min-width: 100%;
                                                max-width: 100%;
                                                justify-content: start;
                                            "#,
                                            SocialIconGroup {
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/discord.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/github.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/medium.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/telegram.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/x.svg") }
                                            }
                                        }
                                        layout::Col {
                                            style: r#"
                                                justify-content: start;
                                                height: 100%;
                                                min-height: 0%;
                                                max-height: 100%;
                                                flex: 1;
                                                gap: {web::sequence(1)}px;
                                            "#,
                                            layout::Row {
                                                style: r#"
                                                    width: 100%;
                                                    min-width: 100%;
                                                    max-width: 100%;
                                                    justify-content: start;
                                                    font-family: br cobane;
                                                    font-size: {web::sequence(5)}px;
                                                    font-weight: bold;
                                                    color: {color::SILVER};
                                                "#,
                                                "Trustless by Design. Ruthless in Reliability."
                                            }
                                            layout::Row {
                                                style: r#"
                                                    width: 100%;
                                                    min-width: 100%;
                                                    max-width: 100%;
                                                    justify-content: start;
                                                    font-family: br cobane;
                                                    font-size: {web::sequence(2)}px;
                                                    font-weight: normal;
                                                    color: {color::SILVER};
                                                "#,
                                                "The silent layer beneath the loudest DAOs—secure, scalable, sovereign."
                                            }
                                        }
                                        layout::Row {
                                            layout::Row {
                                                
                                                decor::HStripe {
                                                    w: "100%",
                                                    h: "5px",
                                                    color: color::IMPERIAL_RED
                                                }
                                            }
                                            "Cta Buttons go here"
                                        }
                                    }
                                }
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
                                            color_0: color::interpolate((color::OBSIDIAN, color::SILVER), 0.5),
                                            color_1: color::interpolate((color::OBSIDIAN, color::STEEL), 0.5)
                                        }
                                        layout::Row {
                                            style: r#"
                                                width: 100%;
                                                min-width: 100%;
                                                max-width: 100%;
                                                justify-content: end;
                                            "#,
                                            intf::LinkedButton {
                                                style: r#"
                                                    font-size: {web::sequence(1)}px;
                                                "#,
                                                url: "/",
                                                leads_to_external_url: true,
                                                "Learn More"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        HorizontalCarousel {
                            style: r#"
                                font-size: 2em;
                                color: white;
                            "#,
                            slots: vec![rsx! {
                                "Heellooooo"
                            }, rsx! {
                                "Big Messagee here"
                            }, rsx! {
                                "Too much pizzaaa"
                            }, rsx! {
                                "Its me again!!!"
                            }]
                        }
                    }
                }
            }
            layout::VerticalPageSection {

            }
        }
    }
}