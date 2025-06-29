use dioxus::html::{g::arabic_form, munder::accentunder};

use super::*;

type MaybeListener<T> = Option<EventHandler<Event<T>>>;
type MaybeOpcode = Option<String>;

// some custom events to add such as on visible or not etc.

#[derive(Props, Clone, PartialEq, Default)]
struct EventProps {
    #[props(default=None)] pub on_abort: MaybeListener<MediaData>,
    #[props(default=None)] pub on_animation_end: MaybeListener<AnimationData>,
    #[props(default=None)] pub on_animation_iteration: MaybeListener<AnimationData>,
    #[props(default=None)] pub on_animation_start: MaybeListener<AnimationData>,
    #[props(default=None)] pub on_blur: MaybeListener<FocusData>,
    #[props(default=None)] pub on_can_play: MaybeListener<MediaData>,
    #[props(default=None)] pub on_can_play_through: MaybeListener<MediaData>,
    #[props(default=None)] pub on_change: MaybeListener<FormData>,
    #[props(default=None)] pub on_click: MaybeListener<MouseData>,
    #[props(default=None)] pub on_composition_end: MaybeListener<CompositionData>,
    #[props(default=None)] pub on_composition_start: MaybeListener<CompositionData>,
    #[props(default=None)] pub on_composition_update: MaybeListener<CompositionData>,
    #[props(default=None)] pub on_context_menu: MaybeListener<MouseData>,
    #[props(default=None)] pub on_copy: MaybeListener<ClipboardData>,
    #[props(default=None)] pub on_cut: MaybeListener<ClipboardData>,
    #[props(default=None)] pub on_double_click: MaybeListener<MouseData>,
    #[props(default=None)] pub on_drag: MaybeListener<DragData>,
    #[props(default=None)] pub on_drag_end: MaybeListener<DragData>,
    #[props(default=None)] pub on_drag_enter: MaybeListener<DragData>,
    #[props(default=None)] pub on_drag_exit: MaybeListener<DragData>,
    #[props(default=None)] pub on_drag_leave: MaybeListener<DragData>,
    #[props(default=None)] pub on_drag_over: MaybeListener<DragData>,
    #[props(default=None)] pub on_drag_start: MaybeListener<DragData>,
    #[props(default=None)] pub on_drop: MaybeListener<DragData>,
    #[props(default=None)] pub on_duration_change: MaybeListener<MediaData>,
    #[props(default=None)] pub on_emptied: MaybeListener<MediaData>,
    #[props(default=None)] pub on_encrypted: MaybeListener<MediaData>,
    #[props(default=None)] pub on_ended: MaybeListener<MediaData>,
    #[props(default=None)] pub on_error: MaybeListener<ImageData>,
    #[props(default=None)] pub on_focus: MaybeListener<FocusData>,
    #[props(default=None)] pub on_focus_in: MaybeListener<FocusData>,
    #[props(default=None)] pub on_focus_out: MaybeListener<FocusData>,
    #[props(default=None)] pub on_got_pointer_capture: MaybeListener<PointerData>,
    #[props(default=None)] pub on_input: MaybeListener<FormData>,
    #[props(default=None)] pub on_invalid: MaybeListener<FormData>,
    #[props(default=None)] pub on_key_down: MaybeListener<KeyboardData>,
    #[props(default=None)] pub on_key_press: MaybeListener<KeyboardData>,
    #[props(default=None)] pub on_key_up: MaybeListener<KeyboardData>,
    #[props(default=None)] pub on_load: MaybeListener<ImageData>,
    #[props(default=None)] pub on_loaded_data: MaybeListener<MediaData>,
    #[props(default=None)] pub on_loaded_metadata: MaybeListener<MediaData>,
    #[props(default=None)] pub on_load_start: MaybeListener<MediaData>,
    #[props(default=None)] pub on_lost_pointer_capture: MaybeListener<PointerData>,
    #[props(default=None)] pub on_mounted: MaybeListener<MountedData>,
    #[props(default=None)] pub on_mouse_down: MaybeListener<MouseData>,
    #[props(default=None)] pub on_mouse_enter: MaybeListener<MouseData>,
    #[props(default=None)] pub on_mouse_leave: MaybeListener<MouseData>,
    #[props(default=None)] pub on_mouse_move: MaybeListener<MouseData>,
    #[props(default=None)] pub on_mouse_out: MaybeListener<MouseData>,
    #[props(default=None)] pub on_mouse_over: MaybeListener<MouseData>,
    #[props(default=None)] pub on_mouse_up: MaybeListener<MouseData>,
    #[props(default=None)] pub on_paste: MaybeListener<ClipboardData>,
    #[props(default=None)] pub on_pause: MaybeListener<MediaData>,
    #[props(default=None)] pub on_play: MaybeListener<MediaData>,
    #[props(default=None)] pub on_playing: MaybeListener<MediaData>,
    #[props(default=None)] pub on_pointer_cancel: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_down: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_enter: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_leave: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_move: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_out: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_over: MaybeListener<PointerData>,
    #[props(default=None)] pub on_pointer_up: MaybeListener<PointerData>,
    #[props(default=None)] pub on_progress: MaybeListener<MediaData>,
    #[props(default=None)] pub on_rate_change: MaybeListener<MediaData>,
    #[props(default=None)] pub on_reset: MaybeListener<FormData>,
    #[props(default=None)] pub on_resize: MaybeListener<ResizeData>,
    #[props(default=None)] pub on_scroll: MaybeListener<ScrollData>,
    #[props(default=None)] pub on_seeked: MaybeListener<MediaData>,
    #[props(default=None)] pub on_seeking: MaybeListener<MediaData>,
    #[props(default=None)] pub on_select: MaybeListener<SelectionData>,
    #[props(default=None)] pub on_selection_change: MaybeListener<SelectionData>,
    #[props(default=None)] pub on_select_start: MaybeListener<SelectionData>,
    #[props(default=None)] pub on_stalled: MaybeListener<MediaData>,
    #[props(default=None)] pub on_submit: MaybeListener<FormData>,
    #[props(default=None)] pub on_suspend: MaybeListener<MediaData>,
    #[props(default=None)] pub on_time_update: MaybeListener<MediaData>,
    #[props(default=None)] pub on_toggle: MaybeListener<ToggleData>,
    #[props(default=None)] pub on_touch_cancel: MaybeListener<TouchData>,
    #[props(default=None)] pub on_touch_end: MaybeListener<TouchData>,
    #[props(default=None)] pub on_touch_move: MaybeListener<TouchData>,
    #[props(default=None)] pub on_touch_start: MaybeListener<TouchData>,
    #[props(default=None)] pub on_transition_end: MaybeListener<TransitionData>,
    #[props(default=None)] pub on_visible: MaybeListener<VisibleData>,
    #[props(default=None)] pub on_volume_change: MaybeListener<MediaData>,
    #[props(default=None)] pub on_waiting: MaybeListener<MediaData>,
    #[props(default=None)] pub on_wheel: MaybeListener<WheelData>,
}

macro_rules! has {
    ($($field_ident:ident)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct LowLevelAttributeProps {
            $(
                #[props(default=None)] pub $field_ident: MaybeOpcode,
            )*
        }
    };
}

has!(
    access_key
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
    animation_iteration_count
    animation_name
    animation_play_state
    animation_timing_function
    aria_active_descendant
    aria_atomic
    aria_auto_complete
    aria_busy
    aria_checked
    aria_col_count
    aria_col_index
    aria_col_span
    aria_controls
    aria_current
    aria_described_by
    aria_details
    aria_disabled
    aria_drop_effect
    aria_error_message
    aria_expanded
    aria_flow_to
    aria_grabbed
    aria_has_popup
    aria_hidden
    aria_invalid
    aria_key_shortcuts
    aria_label
    aria_labelled_by
    aria_level
    aria_live
    aria_modal
    aria_multi_line
    aria_multi_selectable
    aria_orientation
    aria_owns
    aria_placeholder
    aria_pos_in_set
    aria_pressed
    aria_readonly
    aria_relevant
    aria_required
    aria_role_description
    aria_row_count
    aria_row_index
    aria_row_span
    aria_selected
    aria_set_size
    aria_sort
    aria_value_max
    aria_value_min
    aria_value_now
    aria_value_text
    aspect_ratio
    auto_capitalize
    auto_focus
    azimuth
    backdrop_filter
    backface_visibility
    background
    background_attachment
    background_blend_mode
    background_clip
    background_color
    background_image
    background_origin
    background_position
    background_repeat
    background_size
    baseline_shift
    bleed
);

#[derive(Props, Clone, PartialEq)]
struct DivProps {
    pub event: EventProps,
    pub attrs: LowLevelAttributeProps,
}

#[component]
fn Div(props: DivProps) -> Element {
    rsx! {
        div {
            onabort: into_listener(props.event.on_abort),
            onanimationend: into_listener(props.event.on_animation_end),
            onanimationiteration: into_listener(props.event.on_animation_iteration),
            onanimationstart: into_listener(props.event.on_animation_start),
            onblur: into_listener(props.event.on_blur),
            oncanplay: into_listener(props.event.on_can_play),
            oncanplaythrough: into_listener(props.event.on_can_play_through),
            onchange: into_listener(props.event.on_change),
            onclick: into_listener(props.event.on_click),
            oncompositionend: into_listener(props.event.on_composition_end),
            oncompositionstart: into_listener(props.event.on_composition_start),
            oncompositionupdate: into_listener(props.event.on_composition_update),
            oncontextmenu: into_listener(props.event.on_context_menu),
            oncopy: into_listener(props.event.on_copy),
            oncut: into_listener(props.event.on_cut),
            ondoubleclick: into_listener(props.event.on_double_click),
            ondrag: into_listener(props.event.on_drag),
            ondragend: into_listener(props.event.on_drag_end),
            ondragenter: into_listener(props.event.on_drag_enter),
            ondragexit: into_listener(props.event.on_drag_exit),
            ondragleave: into_listener(props.event.on_drag_leave),
            ondragover: into_listener(props.event.on_drag_over),
            ondragstart: into_listener(props.event.on_drag_start),
            ondrop: into_listener(props.event.on_drop),
            ondurationchange: into_listener(props.event.on_duration_change),
            onemptied: into_listener(props.event.on_emptied),
            onencrypted: into_listener(props.event.on_encrypted),
            onended: into_listener(props.event.on_ended),
            onerror: into_listener(props.event.on_error),
            onfocus: into_listener(props.event.on_focus),
            onfocusin: into_listener(props.event.on_focus_in),
            onfocusout: into_listener(props.event.on_focus_out),
            ongotpointercapture: into_listener(props.event.on_got_pointer_capture),
            oninput: into_listener(props.event.on_input),
            oninvalid: into_listener(props.event.on_invalid),
            onkeydown: into_listener(props.event.on_key_down),
            onkeypress: into_listener(props.event.on_key_press),
            onkeyup: into_listener(props.event.on_key_up),
            onload: into_listener(props.event.on_load),
            onloadeddata: into_listener(props.event.on_loaded_data),
            onloadedmetadata: into_listener(props.event.on_loaded_metadata),
            onloadstart: into_listener(props.event.on_load_start),
            onlostpointercapture: into_listener(props.event.on_lost_pointer_capture),
            onmounted: into_listener(props.event.on_mounted),
            onmousedown: into_listener(props.event.on_mouse_down),
            onmouseenter: into_listener(props.event.on_mouse_enter),
            onmouseleave: into_listener(props.event.on_mouse_leave),
            onmousemove: into_listener(props.event.on_mouse_move),
            onmouseout: into_listener(props.event.on_mouse_out),
            onmouseover: into_listener(props.event.on_mouse_over),
            onmouseup: into_listener(props.event.on_mouse_up),
            onpaste: into_listener(props.event.on_paste),
            onpause: into_listener(props.event.on_pause),
            onplay: into_listener(props.event.on_play),
            onplaying: into_listener(props.event.on_playing),
            onpointercancel: into_listener(props.event.on_pointer_cancel),
            onpointerdown: into_listener(props.event.on_pointer_down),
            onpointerenter: into_listener(props.event.on_pointer_enter),
            onpointerleave: into_listener(props.event.on_pointer_leave),
            onpointermove: into_listener(props.event.on_pointer_move),
            onpointerout: into_listener(props.event.on_pointer_out),
            onpointerover: into_listener(props.event.on_pointer_over),
            onpointerup: into_listener(props.event.on_pointer_up),
            onprogress: into_listener(props.event.on_progress),
            onratechange: into_listener(props.event.on_rate_change),
            onreset: into_listener(props.event.on_reset),
            onresize: into_listener(props.event.on_resize),
            onscroll: into_listener(props.event.on_scroll),
            onseeked: into_listener(props.event.on_seeked),
            onseeking: into_listener(props.event.on_seeking),
            onselect: into_listener(props.event.on_select),
            onselectionchange: into_listener(props.event.on_selection_change),
            onselectstart: into_listener(props.event.on_select_start),
            onstalled: into_listener(props.event.on_stalled),
            onsubmit: into_listener(props.event.on_submit),
            onsuspend: into_listener(props.event.on_suspend),
            ontimeupdate: into_listener(props.event.on_time_update),
            ontoggle: into_listener(props.event.on_toggle),
            ontouchcancel: into_listener(props.event.on_touch_cancel),
            ontouchend: into_listener(props.event.on_touch_end),
            ontouchmove: into_listener(props.event.on_touch_move),
            ontouchstart: into_listener(props.event.on_touch_start),
            ontransitionend: into_listener(props.event.on_transition_end),
            onvisible: into_listener(props.event.on_visible),
            onvolumechange: into_listener(props.event.on_volume_change),
            onwaiting: into_listener(props.event.on_waiting),
            onwheel: into_listener(props.event.on_wheel),


            accesskey: props.attrs.access_key,
            align_content: props.attrs.align_content,
            align_items: props.attrs.align_items,
            align_self: props.attrs.align_self,
            alignment_adjust: props.attrs.alignment_adjust,
            alignment_baseline: props.attrs.alignment_baseline,
            all: props.attrs.all,
            alt: props.attrs.alt,
            animation: props.attrs.animation,
            animation_delay: props.attrs.animation_delay,
            animation_direction: props.attrs.animation_direction,
            animation_duration: props.attrs.animation_duration,
            animation_fill_mode: props.attrs.animation_fill_mode,
            animation_iteration_count: props.attrs.animation_iteration_count,
            animation_name: props.attrs.animation_name,
            animation_play_state: props.attrs.animation_play_state,
            animation_timing_function: props.attrs.animation_timing_function,
            aria_activedescendant: props.attrs.aria_active_descendant,
            aria_atomic: props.attrs.aria_atomic,
            aria_autocomplete: props.attrs.aria_auto_complete,
            aria_busy: props.attrs.aria_busy,
            aria_checked: props.attrs.aria_checked,
            aria_colcount: props.attrs.aria_col_count,
            aria_colindex: props.attrs.aria_col_index,
            aria_colspan: props.attrs.aria_col_span,
            aria_controls: props.attrs.aria_controls,
            aria_current: props.attrs.aria_current,
            aria_describedby: props.attrs.aria_described_by,
            aria_details: props.attrs.aria_details,
            aria_disabled: props.attrs.aria_disabled,
            aria_dropeffect: props.attrs.aria_drop_effect,
            aria_errormessage: props.attrs.aria_error_message,
            aria_expanded: props.attrs.aria_expanded,
            aria_flowto: props.attrs.aria_flow_to,
            aria_grabbed: props.attrs.aria_grabbed,
            aria_haspopup: props.attrs.aria_has_popup,
            aria_hidden: props.attrs.aria_hidden,
            aria_invalid: props.attrs.aria_invalid,
            aria_keyshortcuts: props.attrs.aria_key_shortcuts,
            aria_label: props.attrs.aria_label,
            aria_labelledby: props.attrs.aria_labelled_by,
            aria_level: props.attrs.aria_level,
            aria_live: props.attrs.aria_live,
            aria_modal: props.attrs.aria_modal,
            aria_multiline: props.attrs.aria_multi_line,
            aria_multiselectable: props.attrs.aria_multi_selectable,
            aria_orientation: props.attrs.aria_orientation,
            aria_owns: props.attrs.aria_owns,
            aria_placeholder: props.attrs.aria_placeholder,
            aria_posinset: props.attrs.aria_pos_in_set,
            aria_pressed: props.attrs.aria_pressed,
            aria_readonly: props.attrs.aria_readonly,
            aria_relevant: props.attrs.aria_relevant,
            aria_required: props.attrs.aria_required,
            aria_roledescription: props.attrs.aria_role_description,
            aria_rowcount: props.attrs.aria_row_count,
            aria_rowindex: props.attrs.aria_row_index,
            aria_rowspan: props.attrs.aria_row_span,
            aria_selected: props.attrs.aria_selected,
            aria_setsize: props.attrs.aria_set_size,
            aria_sort: props.attrs.aria_sort,
            aria_valuemax: props.attrs.aria_value_max,
            aria_valuemin: props.attrs.aria_value_min,
            aria_valuenow: props.attrs.aria_value_now,
            aria_valuetext: props.attrs.aria_value_text,
            aspect_ratio: props.attrs.aspect_ratio,
            autocapitalize: props.attrs.auto_capitalize,
            autofocus: props.attrs.auto_focus,
            azimuth: props.attrs.azimuth,
            backdrop_filter: props.attrs.backdrop_filter,
            backface_visibility: props.attrs.backface_visibility,
            background: props.attrs.background,
            background_attachment: props.attrs.background_attachment,
            background_blend_mode: props.attrs.background_blend_mode,
            background_clip: props.attrs.background_clip,
            background_color: props.attrs.background_color,
            background_image: props.attrs.background_image,
            background_origin: props.attrs.background_origin,
            background_position: props.attrs.background_position,
            background_repeat: props.attrs.background_repeat,
            background_size: props.attrs.background_size,
            baseline_shift: props.attrs.baseline_shift,
            bleed: props.attrs.bleed,
            b
        }
    }
}

#[allow(dead_code)]
fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(Event<T>) {
    move |data| {
        if let Some(listener) = maybe_listener {
            listener(data);
        }
    }
}