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
}