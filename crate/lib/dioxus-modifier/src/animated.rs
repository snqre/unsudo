use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct AnimateProps {

    /// A CSS transition string that defines how property changes should animate.
    ///
    /// This value is directly inserted into the `transition` style property of the component.
    /// It supports any valid CSS transition syntax, such as:
    ///
    /// - `"all 0.3s ease-in-out"`
    /// - `"opacity 0.5s ease"`
    /// - `"transform 0.4s cubic-bezier(0.4, 0, 0.2, 1)"`
    /// - `"opacity 0.3s ease, transform 0.5s ease-in-out"`
    /// - `"none"` (to disable transitions)
    ///
    /// Only animatable CSS properties will respond to transitions.
    /// If `None` is provided, no transition will be applied (empty string).
    ///
    /// See: [MDN - transition](https://developer.mozilla.org/en-US/docs/Web/CSS/transition)
    pub transition: String,
    
    pub children: Option<Element>
}

#[component]
pub fn Animate(props: AnimateProps) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: contents;
                    transition: {};
                "#,
                props.transition
            )
        }
    )
}