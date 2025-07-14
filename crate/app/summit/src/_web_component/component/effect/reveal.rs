use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct Props {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Reveal(props: Props) -> Element {
    let mut opacity: Signal<f32> = use_signal(|| 0.0f32);

    use_effect(move || {
        opacity.set(1.0f32);
    });

    rsx! {
        div {
            style: r#"
                transition: 1s linear;
                opacity: {opacity()};
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}