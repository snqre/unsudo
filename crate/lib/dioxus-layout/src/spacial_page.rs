use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SpacialPageProps {
    pub x: u32,
    pub y: u32,
    pub animation_duration_seconds: f64,
    pub animation_easing: String,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

/// Top-level layout component enabling 2D navigation through sections.
///
/// ## Layout Behavior:
/// - Layout is treated as a grid of full-viewport (100vw x 100vh) sections.
/// - The `x` and `y` props determine the current viewport offset.
/// - Positioning must be **consecutive**: you cannot skip directly from `(1, 0)` to `(9, 0)`; the next section must be `(2, 0)`, etc.
/// - Overlapping sections will display the **last declared** component.
/// - **Negative positioning (e.g., `(-1, 0)`) is not supported.**
#[component]
pub fn SpacialPage(props: SpacialPageProps) -> Element {
    rsx!(
        Col {
            class: props.class,
            style: format!(
                r#"
                    position: relative;
                    width: 100vw;
                    height: 100vh;
                    overflow: hidden;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            div {
                style: format!(
                    r#"
                        position: absolute;
                        display: grid;
                        grid-template-columns: repeat(1, 100vw);
                        grid-template-rows: repeat(1, 100vh);
                        width: 100vw;
                        height: 100vh;
                        left: -{}%;
                        top: -{}%;
                        transition: {}s {};
                    "#,
                    props.x * 100,
                    props.y * 100,
                    props.animation_duration_seconds,
                    props.animation_easing
                ),
                { props.children }
            }
        }
    )
}