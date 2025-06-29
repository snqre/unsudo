use super::*;

macro_rules! has {
    ($($field_ident:ident $payload_ty:ty)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct EventProps {
            $(
                #[props(default=None)] pub $field_ident: MaybeListener<$payload_ty>,
            )*
        }
    };
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
    on_dreg DragData
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
    on_
);