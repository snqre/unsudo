#![allow(clippy::let_with_type_underscore)]

use ::dioxus::prelude::*;

pub use overridable::*;
pub use attrs::*;

mod overridable {
    #[macro_export(local_inner_macros)]
    macro_rules! take_or_keep {
        ($edit:ident $self:ident $($key:ident)*) => {
            Self {
                $(
                    $key: $edit.$key.or($self.$key),
                )*
            }
        };
    }

    #[macro_export(local_inner_macros)]
    macro_rules! take_or_else {
        ($edit:ident $self:ident $($key:ident)*) => {
            Self {
                $(
                    $key: $edit.$key.or_else(|| $self.$key),
                )*
            }
        };
    }

    pub trait Overridable {

        /// # Example
        /// ```rs
        /// use ::dioxus::prelude::*;
        /// use ::unsudo_dioxus::extendable;
        /// 
        /// #[derive(Props)]
        /// #[derive(Clone)]
        /// #[derive(PartialEq)]
        /// pub struct FooProps {
        ///     pub attrs: Option<extendable::AttrsProps>,
        ///     pub event: Option<extendable::EventProps>,
        ///     pub children: Option<Element>
        /// }
        /// 
        /// #[component]
        /// pub fn Foo(props: FooProps) -> Element {
        ///     rsx! {
        ///         extendable::Node {
        ///             attrs: props.attrs.unwrap_or_default().take_or_keep(extendable::AttrsProps {
        ///                 // Will replace `display`.
        ///                 display: "flex".into(),
        ///                 ..Default::default()
        ///             }),
        ///             event: props.event,
        ///             { props.children }
        ///         }
        ///     }
        /// }
        /// ```
        fn take_or_keep(self, edit: Self) -> Self;
        
        /// # Example
        /// ```rs
        /// use ::dioxus::prelude::*;
        /// use ::unsudo_dioxus::extendable;
        /// 
        /// #[derive(Props)]
        /// #[derive(Clone)]
        /// #[derive(PartialEq)]
        /// pub struct FooProps {
        ///     pub attrs: Option<extendable::AttrsProps>,
        ///     pub event: Option<extendable::EventProps>,
        ///     pub children: Option<Element>
        /// }
        /// 
        /// #[component]
        /// pub fn Foo(props: FooProps) -> Element {
        ///     rsx! {
        ///         extendable::Node {
        ///             attrs: props.attrs,
        ///             event: props.event.unwrap_or_default().take_or_else(extendable::EventProps {
        ///                 on_click: |_| {
        ///                     // If `props.event.on_click` is `None` then this listener is passed down.
        ///                     // If `props.event.on_click` is `Some` then `props.event.on_click` listener is passed down.
        ///                 }.into(),
        ///                 ..Default::default()
        ///             }),
        ///             { props.children }
        ///         }
        ///     }
        /// }
        /// ```
        fn take_or_else(self, edit: Self) -> Self;
    }
}

mod attrs {
    use super::*;

    macro_rules! attrs_props_has {
        ($($field_ident:ident)*) => {
            #[derive(Props)]
            #[derive(Clone)]
            #[derive(PartialEq)]
            #[derive(Default)]
            pub struct AttrsProps {
                $(
                    #[props(default=None)] pub $field_ident: MaybeOpcode<'static>,
                )*
            }
        };
    } attrs_props_has!(
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

    impl AttrsProps {
        pub fn add_class(self, #[allow(unused_variables)] class: &str) -> Self {
            Self {
                class: r#"
                    {self.class.to_owned().unwrap_or_default()}
                    {class}
                "#.into(),
                ..self
            }
        }
        pub fn add_style_before(self, #[allow(unused_variables)] style: &str) -> Self {
            Self {
                style: r#"
                    {style}
                    {self.style.to_owned().unwrap_or_default()}
                "#.into(),
                ..self
            }
        }
        pub fn add_style(self, #[allow(unused_variables)] style: &str) -> Self {
            Self {
                style: r#"
                    {self.style.to_owned().unwrap_or_default()}
                    {style}
                "#.into(),
                ..self
            }
        }
    }

    impl Overridable for AttrsProps {
        fn take_or_keep(self, edit: Self) -> Self {
            take_or_keep!(
                edit self
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
            )
        }
        fn take_or_else(self, edit: Self) -> Self {
            take_or_else!(
                edit self
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
            )
        }
    }
}



macro_rules! attrs_props_has {
    ($($field_ident:ident)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct AttrsProps {
            $(
                #[props(default=None)] pub $field_ident: MaybeOpcode<'static>,
            )*
        }
    };
}
macro_rules! attrs_props_try_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| { $self.$key }),
            )*
        }
    };
}
macro_rules! attrs_props_force_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or($self.$key),
            )*
        }
    };
}
macro_rules! event_props_has {
    ($($field:ident $payload:ty)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct EventProps {
            $(
                #[props(default=None)] pub $field: MaybeListener<$payload>,
            )*
        }
    };
}
macro_rules! event_props_add_listener {
    ($edit:ident $self:ident $($key:ident $payload_ty:ty)*) => {
        Self {
            $(
                $key: match ($edit.$key, $self.$key) {
                    (Some(a), Some(b)) => {
                        Some(Callback::new(move |data: dioxus::prelude::Event<$payload_ty>| {
                            let a: _ = a.to_owned();
                            let b: _ = b.to_owned();
                            a(data.to_owned());
                            b(data);
                        }))
                    },
                    (Some(a), None) => Some(a.to_owned()),
                    (None, Some(b)) => Some(b.to_owned()),
                    (None, None) => None
                },
            )*
        }
    };
}
macro_rules! event_props_try_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| { $self.$key.to_owned() }),
            )*
        }
    };
}
macro_rules! event_props_force_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or($self.$key),
            )*
        }
    };
}
macro_rules! assign {
    ($props:ident $($attr:ident $route:ident)*) => {
        ::paste::paste! {
            rsx!(
                div {
                    onabort: into_listener($props.event.on_abort),
                    onanimationend: into_listener($props.event.on_animation_end),
                    onanimationiteration: into_listener($props.event.on_animation_iteration),
                    onanimationstart: into_listener($props.event.on_animation_start),
                    onblur: into_listener($props.event.on_blur),
                    oncanplay: into_listener($props.event.on_can_play),
                    oncanplaythrough: into_listener($props.event.on_can_play_through),
                    onchange: into_listener($props.event.on_change),
                    onclick: into_listener($props.event.on_click),
                    oncompositionend: into_listener($props.event.on_composition_end),
                    oncompositionstart: into_listener($props.event.on_composition_start),
                    oncompositionupdate: into_listener($props.event.on_composition_update),
                    oncontextmenu: into_listener($props.event.on_context_menu),
                    oncopy: into_listener($props.event.on_copy),
                    oncut: into_listener($props.event.on_cut),
                    ondoubleclick: into_listener($props.event.on_double_click),
                    ondrag: into_listener($props.event.on_drag),
                    ondragend: into_listener($props.event.on_drag_end),
                    ondragenter: into_listener($props.event.on_drag_enter),
                    ondragexit: into_listener($props.event.on_drag_exit),
                    ondragleave: into_listener($props.event.on_drag_leave),
                    ondragover: into_listener($props.event.on_drag_over),
                    ondragstart: into_listener($props.event.on_drag_start),
                    ondrop: into_listener($props.event.on_drop),
                    ondurationchange: into_listener($props.event.on_duration_change),
                    onemptied: into_listener($props.event.on_emptied),
                    onencrypted: into_listener($props.event.on_encrypted),
                    onended: into_listener($props.event.on_ended),
                    onfocus: into_listener($props.event.on_focus),
                    onfocusin: into_listener($props.event.on_focus_in),
                    onfocusout: into_listener($props.event.on_focus_out),
                    ongotpointercapture: into_listener($props.event.on_got_pointer_capture),
                    oninput: into_listener($props.event.on_input),
                    oninvalid: into_listener($props.event.on_invalid),
                    onkeydown: into_listener($props.event.on_key_down),
                    onkeypress: into_listener($props.event.on_key_press),
                    onkeyup: into_listener($props.event.on_key_up),
                    onload: into_listener($props.event.on_load),
                    onloadeddata: into_listener($props.event.on_loaded_data),
                    onloadedmetadata: into_listener($props.event.on_loaded_metadata),
                    onloadstart: into_listener($props.event.on_load_start),
                    onlostpointercapture: into_listener($props.event.on_lost_pointer_capture),
                    onmounted: into_listener($props.event.on_mounted),
                    onmousedown: into_listener($props.event.on_mouse_down),
                    onmouseenter: into_listener($props.event.on_mouse_enter),
                    onmouseleave: into_listener($props.event.on_mouse_leave),
                    onmousemove: into_listener($props.event.on_mouse_move),
                    onmouseout: into_listener($props.event.on_mouse_out),
                    onmouseover: into_listener($props.event.on_mouse_over),
                    onmouseup: into_listener($props.event.on_mouse_up),
                    onpaste: into_listener($props.event.on_paste),
                    onpause: into_listener($props.event.on_pause),
                    onplay: into_listener($props.event.on_play),
                    onplaying: into_listener($props.event.on_playing),
                    onpointercancel: into_listener($props.event.on_pointer_cancel),
                    onpointerdown: into_listener($props.event.on_pointer_down),
                    onpointerenter: into_listener($props.event.on_pointer_enter),
                    onpointerleave: into_listener($props.event.on_pointer_leave),
                    onpointermove: into_listener($props.event.on_pointer_move),
                    onpointerout: into_listener($props.event.on_pointer_out),
                    onpointerover: into_listener($props.event.on_pointer_over),
                    onpointerup: into_listener($props.event.on_pointer_up),
                    onprogress: into_listener($props.event.on_progress),
                    onratechange: into_listener($props.event.on_rate_change),
                    onreset: into_listener($props.event.on_reset),
                    onresize: into_listener($props.event.on_resize),
                    onscroll: into_listener($props.event.on_scroll),
                    onseeked: into_listener($props.event.on_seeked),
                    onseeking: into_listener($props.event.on_seeking),
                    onselect: into_listener($props.event.on_select),
                    onselectionchange: into_listener($props.event.on_selection_change),
                    onselectstart: into_listener($props.event.on_select_start),
                    onstalled: into_listener($props.event.on_stalled),
                    onsubmit: into_listener($props.event.on_submit),
                    onsuspend: into_listener($props.event.on_suspend),
                    ontimeupdate: into_listener($props.event.on_time_update),
                    ontoggle: into_listener($props.event.on_toggle),
                    ontouchcancel: into_listener($props.event.on_touch_cancel),
                    ontouchend: into_listener($props.event.on_touch_end),
                    ontouchmove: into_listener($props.event.on_touch_move),
                    ontouchstart: into_listener($props.event.on_touch_start),
                    ontransitionend: into_listener($props.event.on_transition_end),
                    onvisible: into_listener($props.event.on_visible),
                    onvolumechange: into_listener($props.event.on_volume_change),
                    onwheel: into_listener($props.event.on_wheel),
                    $(
                        $attr: $props.attrs.$route.to_owned(),
                    )*
                    { $props.children }
                }
            )
        }
    };
}

pub type MaybeOpcode<'a> = Option<&'a str>;
pub type MaybeListener<T> = Option<EventHandler<Event<T>>>;

attrs_props_has!(
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

event_props_has!(
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

impl AttrsProps {
    pub fn add_class(self, #[allow(unused_variables)] class: &str) -> Self {
        Self {
            class: r#"
                {self.class.to_owned().unwrap_or_default()}
                {class}
            "#.into(),
            ..self
        }
    }
    pub fn add_style(self, #[allow(unused_variables)] style: &str) -> Self {
        Self {
            style: r#"
                {self.style.to_owned().unwrap_or_default()}
                {style}
            "#.into(),
            ..self
        }
    }
    pub fn add_style_before(self, #[allow(unused_variables)] style: &str) -> Self {
        Self {
            style: r#"
                {style}
                {self.style.to_owned().unwrap_or_default()}
            "#.into(),
            ..self
        }
    }

    /// # Example
    /// ```rs
    /// use ::dioxus::prelude::*;
    /// use ::unsudo_dioxus::extendable;
    /// 
    /// #[derive(Props)]
    /// #[derive(Clone)]
    /// #[derive(PartialEq)]
    /// pub struct FooProps {
    ///     pub attrs: Option<extendable::AttrsProps>,
    ///     pub event: Option<extendable::EventProps>,
    ///     pub children: Option<Element>
    /// }
    /// 
    /// #[component]
    /// pub fn Foo(props: FooProps) -> Element {
    ///     rsx! {
    ///         extendable::Node {
    ///             attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
    ///                 // If `props.attrs.display` is `None` then this property is passed down.
    ///                 // If `props.attrs.display` is `Some` then `props.attrs.display` property is passed down.
    ///                 display: "flex".into(),
    ///                 ..Default::default()
    ///             }),
    ///             event: props.event,
    ///             { props.children }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn try_override(self, edit: Self) -> Self {
        attrs_props_try_override!(
            edit self
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
        )
    }

    /// # Example
    /// ```rs
    /// use ::dioxus::prelude::*;
    /// use ::unsudo_dioxus::extendable;
    /// 
    /// #[derive(Props)]
    /// #[derive(Clone)]
    /// #[derive(PartialEq)]
    /// pub struct FooProps {
    ///     pub attrs: Option<extendable::AttrsProps>,
    ///     pub event: Option<extendable::EventProps>,
    ///     pub children: Option<Element>
    /// }
    /// 
    /// #[component]
    /// pub fn Foo(props: FooProps) -> Element {
    ///     rsx! {
    ///         extendable::Node {
    ///             attrs: props.attrs.unwrap_or_default().force_override(extendable::AttrsProps {
    ///                 // Will replace `display`.
    ///                 display: "flex".into(),
    ///                 ..Default::default()
    ///             }),
    ///             event: props.event,
    ///             { props.children }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn force_override(self, edit: Self) -> Self {
        attrs_props_force_override!(
            edit self
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
        )
    }    
}

impl EventProps {

    /// # Example
    /// ```rs
    /// use ::dioxus::prelude::*;
    /// use ::unsudo_dioxus::extendable;
    /// 
    /// #[derive(Props)]
    /// #[derive(Clone)]
    /// #[derive(PartialEq)]
    /// pub struct FooProps {
    ///     pub attrs: Option<extendable::AttrsProps>,
    ///     pub event: Option<extendable::EventProps>,
    ///     pub children: Option<Element>
    /// }
    /// 
    /// #[component]
    /// pub fn Foo(props: FooProps) -> Element {
    ///     rsx! {
    ///         extendable::Node {
    ///             attrs: props.attrs,
    ///             event: props.event.unwrap_or_default().try_override(extendable::EventProps {
    ///                 on_click: |_| {
    ///                     // If `props.event.on_click` is `None` then this listener is passed down.
    ///                     // If `props.event.on_click` is `Some` then `props.event.on_click` listener is passed down.
    ///                 }.into(),
    ///                 ..Default::default()
    ///             }),
    ///             { props.children }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn try_override(self, edit: Self) -> Self {
        event_props_try_override!(
            edit self
            on_abort
            on_animation_end
            on_animation_iteration
            on_animation_start
            on_blur
            on_can_play
            on_can_play_through
            on_change
            on_click
            on_composition_end
            on_composition_start
            on_composition_update
            on_context_menu
            on_copy
            on_cut
            on_double_click
            on_drag
            on_drag_end
            on_drag_enter
            on_drag_exit
            on_drag_leave
            on_drag_over
            on_drag_start
            on_drop
            on_duration_change
            on_emptied
            on_encrypted
            on_ended
            on_error
            on_focus
            on_focus_in
            on_focus_out
            on_got_pointer_capture
            on_input
            on_invalid
            on_key_down
            on_key_press
            on_key_up
            on_load
            on_loaded_data
            on_loaded_metadata
            on_load_start
            on_lost_pointer_capture
            on_mounted
            on_mouse_down
            on_mouse_enter
            on_mouse_leave
            on_mouse_move
            on_mouse_out
            on_mouse_over
            on_mouse_up
            on_paste
            on_pause
            on_play
            on_playing
            on_pointer_cancel
            on_pointer_down
            on_pointer_enter
            on_pointer_leave
            on_pointer_move
            on_pointer_out
            on_pointer_over
            on_pointer_up
            on_progress
            on_rate_change
            on_reset
            on_resize
            on_scroll
            on_seeked
            on_seeking
            on_select
            on_selection_change
            on_select_start
            on_stalled
            on_submit
            on_suspend
            on_time_update
            on_toggle
            on_touch_cancel
            on_touch_end
            on_touch_move
            on_touch_start
            on_transition_end
            on_visible
            on_volume_change
            on_waiting
            on_wheel
        )
    }

    /// # Example
    /// ```rs
    /// use ::dioxus::prelude::*;
    /// use ::unsudo_dioxus::extendable;
    ///  
    /// #[derive(Props)]
    /// #[derive(Clone)]
    /// #[derive(PartialEq)]
    /// pub struct FooProps {
    ///     pub attrs: Option<extendable::AttrsProps>,
    ///     pub event: Option<extendable::EventProps>,
    ///     pub children: Option<Element>
    /// }
    /// 
    /// #[component]
    /// pub fn Foo(props: FooProps) -> Element {
    ///     rsx! {
    ///         extendable::Node {
    ///             attrs: props.attrs,
    ///             event: props.event.unwrap_or_default().add(extendable::EventProps {
    ///                 on_click: |_| {
    ///                     // Both `props.event.on_click` and this listener will be triggered on this event.
    ///                 }.into(),
    ///                 ..Default::default()
    ///             }),
    ///             { props.children }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn add_listener(self, edit: Self) -> Self {
        event_props_add_listener!(
            edit self
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
        ) 
    }
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct NodeProps {
    pub attrs: AttrsProps,
    pub event: EventProps,
    pub children: Option<Element>
}

#[component]
pub fn Node(props: NodeProps) -> Element {
    assign!(
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
        bleed bleed
        bookmark_label bookmark_label
        bookmark_level bookmark_level
        bookmark_state bookmark_state
        border border
        border_color border_color
        border_style border_style
        border_width border_width
        border_bottom border_bottom
        border_bottom_color border_bottom_color
        border_bottom_style border_bottom_style
        border_bottom_width border_bottom_width
        border_left border_left
        border_left_color border_left_color
        border_left_style border_left_style
        border_left_width border_left_width
        border_right border_right
        border_right_color border_right_color
        border_right_style border_right_style
        border_right_width border_right_width
        border_top border_top
        border_top_color border_top_color
        border_top_style border_top_style
        border_top_width border_top_width
        border_collapse border_collapse
        border_image border_image
        border_image_outset border_image_outset
        border_image_repeat border_image_repeat
        border_image_slice border_image_slice
        border_image_source border_image_source
        border_image_width border_image_width
        border_radius border_radius
        border_bottom_left_radius border_bottom_left_radius
        border_bottom_right_radius border_bottom_right_radius
        border_top_left_radius border_top_left_radius
        border_top_right_radius border_top_right_radius
        border_spacing border_spacing
        bottom bottom
        box_decoration_break box_decoration_break
        box_shadow box_shadow
        box_sizing box_sizing
        box_snap box_snap
        break_after break_after
        break_before break_before
        break_inside break_inside
        buffered_rendering buffered_rendering
        caption_side caption_side
        clear clear
        clear_side clear_side
        clip clip
        clip_path clip_path
        clip_rule clip_rule
        color color
        color_adjust color_adjust
        color_correction color_correction
        color_interpolation color_interpolation
        color_interpolation_filters color_interpolation_filters
        color_profile color_profile
        color_rendering color_rendering
        column_fill column_fill
        column_gap column_gap
        column_rule column_rule
        column_rule_color column_rule_color
        column_rule_style column_rule_style
        column_rule_width column_rule_width
        column_span column_span
        columns columns
        column_count column_count
        column_width column_width
        contain contain
        content content
        counter_increment counter_increment
        counter_reset counter_reset
        counter_set counter_set
        cue cue
        cue_after cue_after
        cue_before cue_before
        cursor cursor
        direction direction
        display display
        display_inside display_inside
        display_outside display_outside
        display_extras display_extras
        display_box display_box
        dominant_baseline dominant_baseline
        elevation elevation
        empty_cells empty_cells
        enable_background enable_background
        fill fill
        fill_opacity fill_opacity
        fill_rule fill_rule
        filter filter
        float float
        float_defer_column float_defer_column
        float_defer_page float_defer_page
        float_offset float_offset
        float_wrap float_wrap
        flow_into flow_into
        flow_from flow_from
        flex flex
        flex_basis flex_basis
        flex_grow flex_grow
        flex_shrink flex_shrink 
        flex_flow flex_flow
        flex_direction flex_direction
        flex_wrap flex_wrap
        flood_color flood_color
        flood_opacity flood_opacity
        font font
        font_family font_family
        font_size font_size
        font_stretch font_stretch
        font_style font_style
        font_weight font_weight
        font_feature_settings font_feature_settings
        font_kerning font_kerning
        font_language_override font_language_override
        font_size_adjust font_size_adjust
        font_synthesis font_synthesis
        font_variant font_variant
        font_variant_alternates font_variant_alternates
        font_variant_caps font_variant_caps
        font_variant_east_asian font_variant_east_asian
        font_variant_ligatures font_variant_ligatures
        font_variant_numeric font_variant_numeric
        font_variant_position font_variant_position
        footnote_policy footnote_policy
        glyph_orientation_horizontal glyph_orientation_horizontal
        glyph_orientation_vertical glyph_orientation_vertical
        grid grid
        grid_auto_flow grid_auto_flow
        grid_auto_columns grid_auto_columns
        grid_auto_rows grid_auto_rows
        grid_template grid_template
        grid_template_areas grid_template_areas
        grid_template_columns grid_template_columns
        grid_template_rows grid_template_rows
        grid_area grid_area
        grid_column grid_column
        grid_column_start grid_column_start
        grid_column_end grid_column_end
        grid_row grid_row
        grid_row_start grid_row_start
        grid_row_end grid_row_end
        hanging_punctuation hanging_punctuation
        height height
        hyphenate_character hyphenate_character
        hyphenate_limit_chars hyphenate_character
        hyphenate_limit_last hyphenate_limit_last
        hyphenate_limit_lines hyphenate_limit_lines
        hyphenate_limit_zone hyphenate_limit_zone
        hyphens hyphens
        icon icon
        image_orientation image_orientation
        image_resolution image_resolution
        image_rendering image_rendering
        ime ime
        ime_align ime_align
        ime_mode ime_mode
        ime_offset ime_offset
        ime_width ime_width
        initial_letters initial_letters
        inline_box_align inline_box_align
        isolation isolation
        justify_content justify_content
        justify_items justify_items
        justify_self justify_self
        kerning kerning
        left left
        letter_spacing letter_spacing
        lighting_color lighting_color
        line_box_contain line_box_contain
        line_break line_break
        line_grid line_grid
        line_height line_height
        line_slack line_slack
        line_snap line_snap
        list_style list_style
        list_style_image list_style_image
        list_style_position list_style_position
        list_style_type list_style_type
        margin margin 
        margin_bottom margin_bottom
        margin_left margin_left
        margin_right margin_right
        margin_top margin_top
        marker marker
        marker_end marker_end
        marker_mid marker_mid
        marker_pattern marker_pattern
        marker_segment marker_segment
        marker_start marker_start
        marker_knockout_left marker_knockout_left
        marker_knockout_right marker_knockout_right
        marker_side marker_side
        marks marks
        marquee_direction marquee_direction
        marquee_play_count marquee_play_count
        marquee_speed marquee_speed
        marquee_style marquee_style
        mask mask
        mask_image mask_image
        mask_repeat mask_repeat
        mask_position mask_position
        mask_clip mask_clip
        mask_origin mask_origin
        mask_size mask_size
        mask_box mask_box
        mask_box_outset mask_box_outset
        mask_box_repeat mask_box_repeat
        mask_box_slice mask_box_slice
        mask_box_source mask_box_source
        mask_box_width mask_box_width
        mask_type mask_type
        max_height max_height
        max_lines max_lines
        max_width max_width
        min_height min_height
        min_width min_width
        mix_blend_mode mix_blend_mode
        nav_down nav_down
        nav_index nav_index
        nav_left nav_left
        nav_right nav_right
        nav_up nav_up
        object_fit object_fit
        object_position object_position
        offset_after offset_after
        offset_before offset_before
        offset_end offset_end
        offset_start offset_start
        opacity opacity
        order order
        orphans orphans
        outline outline
        outline_color outline_color
        outline_style outline_style
        outline_width outline_width
        outline_offset outline_offset
        overflow overflow
        overflow_x overflow_x
        overflow_y overflow_y
        overflow_style overflow_style
        overflow_wrap overflow_wrap
        padding padding
        padding_bottom padding_bottom
        padding_left padding_left
        padding_right padding_right
        padding_top padding_top
        page page
        page_break_after page_break_after
        page_break_before page_break_before
        page_break_inside page_break_inside
        paint_order paint_order
        pause pause
        pause_after pause_after
        pause_before pause_before
        perspective perspective
        perspective_origin perspective_origin
        pitch pitch
        pitch_range pitch_range
        pointer_events pointer_events
        position position
        quotes quotes
        region_fragment region_fragment
        resize resize
        rest rest
        rest_after rest_after
        rest_before rest_before
        richness richness
        right right
        ruby_align ruby_align
        ruby_merge ruby_merge
        ruby_position ruby_position
        scroll_behavior scroll_behavior
        scroll_snap_coordinate scroll_snap_coordinate
        scroll_snap_destination scroll_snap_destination
        scroll_snap_points_x scroll_snap_points_x
        scroll_snap_points_y scroll_snap_points_y
        scroll_snap_type scroll_snap_type
        shape_image_threshold shape_image_threshold
        shape_inside shape_inside
        shape_margin shape_margin
        shape_outside shape_outside
        shape_rendering shape_rendering
        size size
        speak speak
        speak_as speak_as
        speak_header speak_header
        speak_numeral speak_numeral
        speak_punctuation speak_punctuation
        speech_rate speech_rate
        stop_color stop_color
        stop_opacity stop_opacity
        stress stress
        string_set string_set
        stroke stroke
        stroke_dasharray stroke_dash_array
        stroke_dashoffset stroke_dash_offset
        stroke_linecap stroke_line_cap
        stroke_linejoin stroke_line_join
        stroke_miterlimit stroke_miter_limit
        stroke_opacity stroke_opacity
        stroke_width stroke_width
        tab_size tab_size
        table_layout table_layout
        text_align text_align
        text_align_all text_align_all
        text_align_last text_align_last
        text_anchor text_anchor
        text_combine_upright text_combine_upright
        text_decoration text_decoration
        text_decoration_color text_decoration_color
        text_decoration_line text_decoration_line
        text_decoration_style text_decoration_style
        text_decoration_skip text_decoration_skip
        text_emphasis text_emphasis
        text_emphasis_color text_emphasis_color
        text_emphasis_style text_emphasis_style
        text_emphasis_position text_emphasis_position
        text_emphasis_skip text_emphasis_skip
        text_height text_height
        text_indent text_indent
        text_justify text_justify
        text_orientation text_orientation
        text_overflow text_overflow
        text_rendering text_rendering
        text_shadow text_shadow
        text_size_adjust text_size_adjust
        text_space_collapse text_space_collapse
        text_spacing text_spacing
        text_transform text_transform
        text_underline_position text_underline_position
        text_wrap text_wrap
        top top
        touch_action touch_action
        transform transform
        transform_box transform_box
        transform_origin transform_origin
        transform_style transform_style
        transition transition
        transition_delay transition_delay
        transition_duration transition_duration
        transition_property transition_property
        unicode_bidi unicode_bidi
        vector_effect vector_effect
        vertical_align vertical_align
        visibility visibility
        voice_balance voice_balance
        voice_duration voice_duration
        voice_family voice_family
        voice_pitch voice_pitch
        voice_range voice_range
        voice_stress voice_stress
        voice_volume voice_volume
        volume volume
        white_space white_space
        widows widows
        width width
        will_change will_change
        word_break word_break
        word_spacing word_spacing
        word_wrap word_wrap
        wrap_flow wrap_flow
        wrap_through wrap_through
        writing_mode writing_mode
        gap gap
        list_styler_type list_styler_type
        row_gap row_gap
        transition_timing_function transition_timing_function
        user_select user_select
        webkit_user_select webkit_user_select
        z_index z_index
        aria_current aria_current
        aria_details aria_details
        aria_disabled aria_disabled
        aria_hidden aria_hidden
        aria_invalid aria_invalid
        aria_keyshortcuts aria_key_shortcuts
        aria_label aria_label
        aria_roledescription aria_role_description
        aria_autocomplete aria_auto_complete
        aria_checked aria_checked
        aria_expanded aria_expanded
        aria_haspopup aria_has_popup
        aria_level aria_level
        aria_modal aria_modal
        aria_multiline aria_multi_line
        aria_multiselectable aria_multi_selectable
        aria_orientation aria_orientation
        aria_placeholder aria_placeholder
        aria_pressed aria_pressed
        aria_readonly aria_readonly
        aria_required aria_required
        aria_selected aria_selected
        aria_sort aria_sort
        aria_valuemax aria_value_max
        aria_valuemin aria_value_min
        aria_valuenow aria_value_now
        aria_valuetext aria_value_text
        aria_atomic aria_atomic
        aria_busy aria_busy
        aria_live aria_live
        aria_relevant aria_relevant
        aria_dropeffect aria_drop_effect
        aria_grabbed aria_grabbed
        aria_activedescendant aria_active_descendant
        aria_colcount aria_col_count
        aria_colindex aria_col_index
        aria_colspan aria_col_span
        aria_controls aria_controls
        aria_describedby aria_described_by
        aria_errormessage aria_error_message
        aria_flowto aria_flow_to
        aria_labelledby aria_labelled_by
        aria_owns aria_owns
        aria_posinset aria_pos_in_set
        aria_rowcount aria_row_count
        aria_rowindex aria_row_index
        aria_rowspan aria_row_span
        aria_setsize aria_set_size
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