use super::*;

macro_rules! has {
    ($($field_ident:ident $payload_ty:ty)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct Event {
            $(
                #[props(default=None)] pub $field_ident: MaybeListener<$payload_ty>,
            )*
        }
    };
}

macro_rules! override_props {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| $self.$key.to_owned()),
            )*
            ..Default::default()
        }
    };
}


pub type MaybeListener<T> = Option<EventHandler<Event<T>>>;


#[allow(dead_code)]
pub(crate) fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(Event<T>) {
    move |data| {
        if let Some(listener) = maybe_listener {
            listener(data);
        }
    }
}


has!(
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

impl Event {
    pub fn try_override(self, edit: Self) -> Self {
        override_props!(
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
        )
    }
}