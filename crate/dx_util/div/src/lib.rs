use ::prelude::*;
use ::dioxus::prelude::*;

bundle!(
    attrs_props
    event_props
    into_listener
    maybe_listener
    maybe_opcode
);

#[macro_export(local_inner_macros)]
macro_rules! div {
    ($props:ident $($attr:ident $route:ident)*) => {
        ::paste::paste! {
            rsx! {
                div {
                    $(
                        $attr: $props.attrs.$route,
                    )*
                    { $props.children }
                }
            }  
        }
    };
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DivProps {
    pub attrs: AttrsProps,
    pub event: EventProps,
    pub children: Option<Element>
}

#[component]
pub fn Div(props: DivProps) -> Element {
    div!(
        props
        accesskey access_key
        autocapitalize auto_capitalize
        autofocus auto_focus
        class class
        content_editable content_editable
        data data
        dir dir
        draggable draggable
        enter_key_hint enter_key_hint
        export_parts export_parts
        hidden hidden
        id id
        input_mode input_mode
        is is
        item_id icon
        item_prop item_prop
    )
}