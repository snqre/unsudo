#![allow(clippy::let_with_type_underscore)]

use ::dioxus::prelude::*;

::modwire::expose!(
    chain_event_handler_ext
    chain_event_handler
    edit_class_ext
    edit_class
    edit_ext
    edit_style_ext
    edit_style
    edit
    node
);

macro_rules! attrs_props {
    ($($field:ident)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct AttrsProps {
            $(
                #[props(default=None)] pub $field: OptionalAttributeOpcode,
            )*
        }
    };
}

macro_rules! event_props {
    ($($field:ident $payload:ty)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct EventProps {
            $(
                #[props(default=None)] pub $field: OptionalEventHandler<$payload>,
            )*
        }
    };
}

pub type AttributeOpcode = String;
pub type OptionalAttributeOpcode = Option<String>;
pub type OptionalEventHandler<T> = Option<EventHandler<Event<T>>>;

attrs_props!(
    access_key
    auto_capitalize
    auto_focus
    class
    content_editable
    data
    dir
    draggable
    enter_key_hint
    export_parts
    hidden
    id
    input_mode
    is
    item_id
    item_prop
    item_ref
    item_scope
    item_type
    lang
    nonce
    part
    popover
    role
    slot
    spell_check
    style
    tab_index
    title
    translate
    align_content
    align_items
    align_self
    alignment_adjust
    alignment_baseline
    all
    alt
    animation
    animation_delay
    animation_direction
    animation_duration
    animation_fill_mode
    animation_name
    animation_play_state
    animation_timing_function
    aspect_ratio
    azimuth
    backdrop_filter
    backface_visibility
    background
    background_attachment
    background_clip
    background_color
    background_image
    background_origin
    background_position
    background_repeat
    background_size
    background_blend_mode
    baseline_shift
    bleed
    bookmark_label
    bookmark_level
    bookmark_state
    border
    border_color
    border_style
    border_width
    border_bottom
    border_bottom_color
    border_bottom_style
    border_bottom_width
    border_left
    border_left_color
    border_left_style
    border_left_width
    border_right
    border_right_color
    border_right_style
    border_right_width
    border_top
    border_top_color
    border_top_style
    border_top_width
    border_collapse
    border_image
    border_image_outset
    border_image_repeat
    border_image_slice
    border_image_source
    border_image_width
    border_radius
    border_bottom_left_radius
    border_bottom_right_radius
    border_top_left_radius
    border_top_right_radius
    border_spacing
    bottom
    box_decoration_break
    box_shadow
    box_sizing
    box_snap
    break_after
    break_before
    break_inside
    buffered_rendering
    caption_side
    clear
    clear_side
    clip
    clip_path
    clip_rule
    color
    color_adjust
    color_correction
    color_interpolation
    color_interpolation_filters
    color_profile
    color_rendering
    column_fill
    column_gap
    column_rule
    column_rule_color
    column_rule_style
    column_rule_width
    column_span
    columns
    column_count
    column_width
    contain
    content
    counter_increment
    counter_reset
    counter_set
    cue
    cue_after
    cue_before
    cursor
    direction
    display
    display_inside
    display_outside
    display_extras
    display_box
    dominant_baseline
    elevation
    empty_cells
    enable_background
    fill
    fill_opacity
    fill_rule
    filter
    float
    float_defer_column
    float_defer_page
    float_offset
    float_wrap
    flow_into
    flow_from
    flex
    flex_basis
    flex_grow
    flex_shrink
    flex_flow
    flex_direction
    flex_wrap
    flood_color
    flood_opacity
    font
    font_family
    font_size
    font_stretch
    font_style
    font_weight
    font_feature_settings
    font_kerning
    font_language_override
    font_size_adjust
    font_synthesis
    font_variant
    font_variant_alternates
    font_variant_caps
    font_variant_east_asian
    font_variant_ligatures
    font_variant_numeric
    font_variant_position
    footnote_policy
    glyph_orientation_horizontal
    glyph_orientation_vertical
    grid
    grid_auto_flow
    grid_auto_columns
    grid_auto_rows
    grid_template
    grid_template_areas
    grid_template_columns
    grid_template_rows
    grid_area
    grid_column
    grid_column_start
    grid_column_end
    grid_row
    grid_row_start
    grid_row_end
    hanging_punctuation
    height
    hyphenate_character
    hyphenate_limit_chars
    hyphenate_limit_last
    hyphenate_limit_lines
    hyphenate_limit_zone
    hyphens
    icon
    image_orientation
    image_resolution
    image_rendering
    ime
    ime_align
    ime_mode
    ime_offset
    ime_width
    initial_letters
    inline_box_align
    isolation
    justify_content
    justify_items
    justify_self
    kerning
    left
    letter_spacing
    lighting_color
    line_box_contain
    line_break
    line_grid
    line_height
    line_slack
    line_snap
    list_style
    list_style_image
    list_style_position
    list_style_type
    margin
    margin_bottom
    margin_left
    margin_right
    margin_top
    marker
    marker_end
    marker_mid
    marker_pattern
    marker_segment
    marker_start
    marker_knockout_left
    marker_knockout_right
    marker_side
    marks
    marquee_direction
    marquee_play_count
    marquee_speed
    marquee_style
    mask
    mask_image
    mask_repeat
    mask_position
    mask_clip
    mask_origin
    mask_size
    mask_box
    mask_box_outset
    mask_box_repeat
    mask_box_slice
    mask_box_source
    mask_box_width
    mask_type
    max_height
    max_lines
    max_width
    min_height
    min_width
    mix_blend_mode
    nav_down
    nav_index
    nav_left
    nav_right
    nav_up
    object_fit
    object_position
    offset_after
    offset_before
    offset_end
    offset_start
    opacity
    order
    orphans
    outline
    outline_color
    outline_style
    outline_width
    outline_offset
    overflow
    overflow_x
    overflow_y
    overflow_style
    overflow_wrap
    padding
    padding_bottom
    padding_left
    padding_right
    padding_top
    page
    page_break_after
    page_break_before
    page_break_inside
    paint_order
    pause
    pause_after
    pause_before
    perspective
    perspective_origin
    pitch
    pitch_range
    pointer_events
    position
    quotes
    region_fragment
    resize
    rest
    rest_after
    rest_before
    richness
    right
    ruby_align
    ruby_merge
    ruby_position
    scroll_behavior
    scroll_snap_coordinate
    scroll_snap_destination
    scroll_snap_points_x
    scroll_snap_points_y
    scroll_snap_type
    shape_image_threshold
    shape_inside
    shape_margin
    shape_outside
    shape_padding
    shape_rendering
    size
    speak
    speak_as
    speak_header
    speak_numeral
    speak_punctuation
    speech_rate
    stop_color
    stop_opacity
    stress
    string_set
    stroke
    stroke_dash_array
    stroke_dash_offset
    stroke_line_cap
    stroke_line_join
    stroke_miter_limit
    stroke_opacity
    stroke_width
    tab_size
    table_layout
    text_align
    text_align_all
    text_align_last
    text_anchor
    text_combine_upright
    text_decoration
    text_decoration_color
    text_decoration_line
    text_decoration_style
    text_decoration_skip
    text_emphasis
    text_emphasis_color
    text_emphasis_style
    text_emphasis_position
    text_emphasis_skip
    text_height
    text_indent
    text_justify
    text_orientation
    text_overflow
    text_rendering
    text_shadow
    text_size_adjust
    text_space_collapse
    text_spacing
    text_transform
    text_underline_position
    text_wrap
    top
    touch_action
    transform
    transform_box
    transform_origin
    transform_style
    transition
    transition_delay
    transition_duration
    transition_property
    unicode_bidi
    vector_effect
    vertical_align
    visibility
    voice_balance
    voice_duration
    voice_family
    voice_pitch
    voice_range
    voice_stress
    voice_volume
    volume
    white_space
    widows
    width
    will_change
    word_break
    word_spacing
    word_wrap
    wrap_flow
    wrap_through
    writing_mode
    gap
    list_styler_type
    row_gap
    transition_timing_function
    user_select
    webkit_user_select
    z_index
    aria_current
    aria_details
    aria_disabled
    aria_hidden
    aria_invalid
    aria_key_shortcuts
    aria_label
    aria_role_description
    aria_auto_complete
    aria_checked
    aria_expanded
    aria_has_popup
    aria_level
    aria_modal
    aria_multi_line
    aria_multi_selectable
    aria_orientation
    aria_placeholder
    aria_pressed
    aria_readonly
    aria_required
    aria_selected
    aria_sort
    aria_value_max
    aria_value_min
    aria_value_now
    aria_value_text
    aria_atomic
    aria_busy
    aria_live
    aria_relevant
    aria_drop_effect
    aria_grabbed
    aria_active_descendant
    aria_col_count
    aria_col_index
    aria_col_span
    aria_controls
    aria_described_by
    aria_error_message
    aria_flow_to
    aria_labelled_by
    aria_owns
    aria_pos_in_set
    aria_row_count
    aria_row_index
    aria_row_span
    aria_set_size
);

event_props!(
    on_abort MediaData
    on_animation_end AnimationData
    on_animation_iteration AnimationData
    on_animation_start AnimationData
    on_blur FocusData
    on_can_play MediaData
    on_can_play_through MediaData
    on_change FormData
    on_click MouseData
    on_composition_end CompositionData
    on_composition_start CompositionData
    on_composition_update CompositionData
    on_context_menu MouseData
    on_copy ClipboardData
    on_cut ClipboardData
    on_double_click MouseData
    on_drag DragData
    on_drag_end DragData
    on_drag_enter DragData
    on_drag_exit DragData
    on_drag_leave DragData
    on_drag_over DragData
    on_drag_start DragData
    on_drop DragData
    on_duration_change MediaData
    on_emptied MediaData
    on_encrypted MediaData
    on_ended MediaData
    on_error ImageData
    on_focus FocusData
    on_focus_in FocusData
    on_focus_out FocusData
    on_got_pointer_capture PointerData
    on_input FormData
    on_invalid FormData
    on_key_down KeyboardData
    on_key_press KeyboardData
    on_key_up KeyboardData
    on_load ImageData
    on_loaded_data MediaData
    on_loaded_metadata MediaData
    on_load_start MediaData
    on_lost_pointer_capture PointerData
    on_mounted MountedData
    on_mouse_down MouseData
    on_mouse_enter MouseData
    on_mouse_leave MouseData
    on_mouse_move MouseData
    on_mouse_out MouseData
    on_mouse_over MouseData
    on_mouse_up MouseData
    on_paste ClipboardData
    on_pause MediaData
    on_play MediaData
    on_playing MediaData
    on_pointer_cancel PointerData
    on_pointer_down PointerData
    on_pointer_enter PointerData
    on_pointer_leave PointerData
    on_pointer_move PointerData
    on_pointer_out PointerData
    on_pointer_over PointerData
    on_pointer_up PointerData
    on_progress MediaData
    on_rate_change MediaData
    on_reset FormData
    on_resize ResizeData
    on_scroll ScrollData
    on_seeked MediaData
    on_seeking MediaData
    on_select SelectionData
    on_selection_change SelectionData
    on_select_start SelectionData
    on_stalled MediaData
    on_submit FormData
    on_suspend MediaData
    on_time_update MediaData
    on_toggle ToggleData
    on_touch_cancel TouchData
    on_touch_end TouchData
    on_touch_move TouchData
    on_touch_start TouchData
    on_transition_end TransitionData
    on_visible VisibleData
    on_volume_change MediaData
    on_waiting MediaData
    on_wheel WheelData
);