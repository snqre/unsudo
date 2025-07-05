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

macro_rules! try_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| $self.$key.to_owned()),
            )*
            ..Default::default()
        }
    };
}

macro_rules! merge {
    ($edit:ident $self:ident $($key:ident $payload_ty:ty)*) => {
        Self {
            $(
                $key: match ($edit.$key, $self.$key) {
                    (Some(a), Some(b)) => {
                        Some(Callback::new(move |data: dioxus::prelude::Event<$payload_ty>| {
                            let a: _ = a.to_owned();
                            let b: _ = b.to_owned();
                            let data_copy_a: _ = data.to_owned();
                            let data_copy_b: _ = data.to_owned();
                            a(data_copy_a);
                            b(data_copy_b);
                        }))
                    },
                    (Some(a), None) => Some(a.to_owned()),
                    (None, Some(b)) => Some(b.to_owned()),
                    (None, None) => None
                },
            )*
            ..Default::default()
        }
    };
}


pub type MaybeListener<T> = Option<EventHandler<dioxus::prelude::Event<T>>>;


#[allow(dead_code)]
pub(crate) fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(dioxus::prelude::Event<T>) {
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
        try_override!(
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

    // calls the handler as well as all handler beneath the call
    // stack. does not override the handler below.
    pub fn merge(self, edit: Self) -> Self {
        merge!(
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