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
        aria_dropeffect aria_drop_effect
        aria_errormessage aria_error_message
        aria_expanded aria_expanded
        aria_flowto aria_flow_to
        aria_grabbed aria_grabbed
        aria_haspopup aria_has_popup
        aria_hidden aria_hidden
        aria_invalid aria_invalid
        aria_keyshortcuts aria_key_shortcuts
        aria_label aria_label
        aria_labelledby aria_labelled_by
        aria_level aria_level
        aria_live aria_live
        aria_modal aria_modal
        aria_multiline aria_multi_line
        aria_multiselectable aria_multi_selectable
        aria_orientation aria_orientation
        aria_owns aria_owns
        aria_placeholder aria_placeholder
        aria_posinset aria_pos_in_set
        aria_pressed aria_pressed
        aria_readonly aria_readonly
        aria_relevant aria_relevant
        aria_required aria_required
        aria_roledescription aria_role_description
        aria_rowcount aria_row_count
        aria_rowindex aria_row_index
        aria_rowspan aria_row_span
        aria_selected aria_selected
        aria_setsize aria_set_size
        aria_sort aria_sort
        aria_valuemax aria_value_max
        aria_valuemin aria_value_min
        aria_valuenow aria_value_now
        aria_valuetext aria_value_text
        aspect_ratio aspect_ratio
        autocapitalize auto_capitalize
        autofocus auto_focus
        azimuth azimuth
        backdrop_filter backdrop_filter
        backface_visibility backface_visibility
        background background
        background_attachment background_attachment
        background_blend_mode background_blend_mode
        background_clip background_clip
        background_color background_color
        background_image background_image
        background_origin background_origin
        background_position background_position
        background_repeat background_repeat
        background_size background_size
        baseline_shift baseline_shift
        bleed bleed
        bookmark_label bookmark_label
        bookmark_level bookmark_level
        bookmark_state bookmark_state
        border border
        border_bottom border_bottom
        border_bottom_color border_bottom_color
        border_bottom_left_radius border_bottom_left_radius
        border_bottom_right_radius border_bottom_right_radius
        border_bottom_style border_bottom_style
        border_bottom_width border_bottom_width
        border_collapse border_collapse
        border_color border_color
        border_image border_image
        border_image_outset border_image_outset
        border_image_repeat border_image_repeat
        border_image_slice border_image_slice
        border_image_source border_image_source
        border_image_width border_image_width
        border_left border_left
        border_left_color border_left_color
        border_left_style border_left_style
        border_left_width border_left_width
        border_radius border_radius
        border_right border_right
        border_right_color border_right_color
        border_right_style border_right_style
        border_right_width border_right_width
        border_spacing border_spacing
        border_style border_style
        border_top border_top
        
    )
}