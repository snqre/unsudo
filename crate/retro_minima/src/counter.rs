use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Props {
    pub start: f64,
    pub end: f64
}

#[component]
pub fn Counter(props: Props) -> Element {
    let mut count: Signal<f64> = use_signal(|| props.start);
    
    use_coroutine(move |x: UnboundedReceiver<_>| async {
        x
    });

    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
            "#
        }
    }
}