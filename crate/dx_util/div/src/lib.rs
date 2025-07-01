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
        contenteditable content_editable
        data data
        dir dir
        draggable draggable
        enterkeyhint enter_key_hint
        exportparts export_parts
        hidden hidden
        id id
        inputmode input_mode
        is is
        itemid icon
        itemprop item_prop
        itemref item_ref
        itemscope item_scope
        itemtype item_type
        lang lang
        nonce nonce
        part part
        popover popover
        role role
        slot slot
        spellcheck spell_check
        style style
        tabindex tab_index
        title title
        translate translate
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
        animation_name animation_name
        animation_play_state animation_play_state
        animation_timing_function animation_timing_function
        aspect_ratio aspect_ratio
        azimuth azimuth
        backdrop_filter backdrop_filter
        backface_visibility backface_visibility
        background background
        background_attachment background_attachment
        background_clip background_clip
        background_color background_color
        background_image background_image
        background_origin background_origin
        background_position background_position
        background_repeat background_repeat
        background_size background_size
        background_blend_mode background_blend_mode
        baseline_shift baseline_shift
    )
}