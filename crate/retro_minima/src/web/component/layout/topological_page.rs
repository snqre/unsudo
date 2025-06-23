use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct TopologicalProps {
    pub x: u32,
    pub y: u32,
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
pub fn Topological(props: TopologicalProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: 100vw;
                height: 100vh;
                position: relative;
                overflow-x: hidden;
                overflow-y: hidden;
            "#,
            div {
                style: r#"
                    display: grid;
                    grid-template-columns: repeat(1, 100vw);
                    grid-template-rows: repeat(1, 100vh);
                    width: 100vw;
                    height: 100vh;
                    position: absolute;
                    left: -{props.x * 100}%;
                    top: -{props.y * 100}%;
                    transition: .5s ease-in;
                "#,
                { props.children }
            }
        }
    }
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SectionProps {
    pub x: u32,
    pub y: u32,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Section(props: SectionProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: 100vw;
                height: 100vh;
                font-size: 2em;
                grid-column: {props.x + 1};
                grid-row: {props.y + 1};
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                { props.children }
            }
        }
    }
}