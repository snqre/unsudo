use super::*;
use crate::web::component::*;
use crate::web::hook::window as win;

leak!(
    hero_heading
    horizontal_carousel_highlight
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
                            justify-content: space-between;
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
                    }
                    HorizontalCarousel {
                        style: r#"
                            width: 100%;
                            min-width: 100%;
                            max-width: 100%;
                            height: 100%;
                            min-height: 0%;
                            max-height: 100%;
                            flex: 1;
                            padding-left: {web::sequence(2)}px;
                            padding-right: {web::sequence(2)}px;
                        "#,
                        slots: vec![rsx! {
                            HorizontalCarouselHighlight {
                                heading: rsx! { "DAOs are the next great form of organisation." },
                                citation: rsx! { "'Yury Serdich, entrepreneur, Forbes'" },
                                source: "https://www.forbes.com/sites/davidprosser/2022/08/16/why-punk-master-believes-daos-are-the-future/"
                            }
                        }, rsx! {
                            HorizontalCarouselHighlight {
                                heading: rsx! { "DAOs are the new limited liability companies…In five years, companies won’t have equity anymore. They’ll have tokens and they’ll be represented as DAOs." },
                                citation: rsx! { "'Cooper Turley, DAO investor via Cointelegraph'" },
                                source: "https://cointelegraph.com/magazine/dao-challenge-business-model-become-new-corporate-paradigm/"
                            }
                        }, rsx! {
                            HorizontalCarouselHighlight {
                                heading: rsx! { "DAOs are a way of democratizing the management structure for the businesses, projects and communities that employ it." },
                                citation: rsx! { "Forbes Finance Council" },
                                source: "https://www.forbes.com/councils/forbesfinancecouncil/2022/10/14/the-state-of-daos-and-what-that-can-mean-for-web3/"
                            }
                        }]
                    }
                }
            }
            layout::VerticalPageSection {
                layout::HorizontalSlider {
                    
                }
            }
        }
    }
}