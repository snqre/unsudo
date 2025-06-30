use ::prelude::*;
use ::dioxus::prelude::*;

bundle!(
    attrs_props
    event_props
);

macro_rules! div {
    ($props:ident $($attr:ident $route:ident)*) => {
        paste::paste! {
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

pub type MaybeListener<T> = Option<EventHandler<Event<T>>>;
pub type MaybeOpcode = Option<String>;

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
        align_content align_content
        align_items align_items
        align_self align_self
        alignment_adjust alignment_adjust
        alignment_baseline alignment_baseline
        all all
        alt alt
        animation animation
        animation_delay animation_delay
        animation_direction animation_direction
        animation_duration animation_duration
        animation_fill_mode animation_fill_mode
        animation_iteration_count animation_iteration_count
        animation_name animation_name
        animation_play_state animation_play_state
        animation_timing_function animation_timing_function
        aria_activedescendant aria_active_descendant
        aria_atomic aria_atomic
        aria_autocomplete aria_auto_complete
        aria_busy aria_busy
        aria_checked aria_checked
        aria_colcount aria_col_count
        aria_colindex aria_col_index
        aria_colspan aria_col_span
        aria_controls aria_controls
        aria_current aria_current
        aria_describedby aria_described_by
        aria_details aria_details
        aria_disabled aria_disabled
        aria_drop_effect aria_drop_effect
        aria_errormessage aria_error_message
        aria_expanded aria_expanded
        aria_flowto aria_flow_to
        aria_grabbed aria_grabbed
        aria_haspopup aria_has_popup
        aria_hidden aria_hidden
    )
}

#[allow(dead_code)]
fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(Event<T>) {
    move |data| {
        if let Some(listener) = maybe_listener {
            listener(data);
        }
    }
}