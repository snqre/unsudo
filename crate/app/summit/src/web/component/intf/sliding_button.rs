use super::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
enum State {
    Idle,
    Hover
}

#[derive(Props, Clone, PartialEq)]
pub struct SlidingButtonProps {
    pub w: f64,
    pub h: f64,
    pub style: Option<Stylesheet>,
    pub children_on_idle: Option<Element>,
    pub children_on_hover: Option<Element>
}

#[component]
pub fn SlidingButton(props: SlidingButtonProps) -> Element {
    let mut state: Signal<_> = use_signal(|| State::Idle);

    rsx! {
        layout::Stack {
            on_mouse_enter: move |_| state.set(State::Hover),
            on_mouse_leave: move |_| state.set(State::Idle),
            style: format! {
                r#"
                    width: 100%;
                    min-width: {}px;
                    max-width: {}px;
                    height: 100%;
                    min-height: {}px;
                    max-height: {}px;
                    cursor: pointer;
                    border-color: {};
                    border-style: solid;
                    border-width: 1px;
                    border-radius: 2px;
                    overflow-x: hidden;
                    overflow-y: hidden;
                    {}
                "#,
                props.w,
                props.w,
                props.h,
                props.h,
                if state() == State::Hover {
                    color::OFFICE_BLUE
                } else {
                    color::CARBON
                },
                props.style.to_owned().unwrap_or_default()
            },
            Block {
                w: props.w,
                h: props.h,
                offset: if state() == State::Idle {
                    0
                } else {
                    1
                },
                { props.children_on_idle }
            }
            Block {
                w: props.w,
                h: props.h,
                offset: if state() == State::Idle {
                    -1
                } else {
                    0
                },
                { props.children_on_hover }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BlockProps {
    pub w: f64,
    pub h: f64,
    pub offset: i8,
    pub style: Option<Stylesheet>,
    pub children: Option<Element>
}

#[component]
fn Block(props: BlockProps) -> Element {
    rsx! {
        layout::StackItem {
            z: 1,
            style: format! {
                r#"
                    width: 100%;
                    min-width: {}px;
                    max-width: {}px;
                    height: 100%;
                    min-height: {}px;
                    max-height: {}px;
                    transform: translate(0%, {}px);
                    transition-delay: 0s;
                    transition-duration: 0.1s;
                    transition-property: all;
                    transition-timing-function: ease-out;
                    {}
                "#,
                props.w,
                props.w,
                props.h,
                props.h,
                props.offset as f64 * props.h,
                props.style.to_owned().unwrap_or_default()
            },
            { props.children }
        }
    }
}