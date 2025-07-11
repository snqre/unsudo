use super::*;

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
    ///             event: props.event.unwrap_or_default().append(extendable::EventProps {
    ///                 on_click: move |_| {
    ///                     // Both `props.event.on_click` and this listener will be triggered on this event.
    ///                 }.into(),
    ///                 ..Default::default()
    ///             })
    ///         }
    ///     }
    /// }
    /// 
    /// 
    /// #[component]
    /// pub fn Main() -> Element {
    ///     rsx! {
    ///         Foo {
    ///             event: extendable::EventProps {
    ///                 on_click: |_| {
    ///                     // Will also be called together with all inner listeners.
    ///                 },
    ///                 ..Default::default()
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn append(self, edit: Self) -> Self {
        append_event_props!(
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