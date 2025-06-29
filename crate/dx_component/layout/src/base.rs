use super::*;

type MaybeListener<T> = Option<EventHandler<Event<T>>>;


// some custom events to add such as on visible or not etc.

#[derive(Props, Clone, PartialEq, Default)]
struct OnProps {
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
    
    #[props(default)] pub on_mouse_enter: MaybeListener<MouseData>,
    #[props(default)] pub on_mouse_leave: MaybeListener<MouseData>,
    #[props(default)] pub on_mouse_move: MaybeListener<MouseData>,
}

#[derive(Props, Clone, PartialEq)]
struct StyleProps {
    pub class: Option<String>,
    pub style: Option<String>
}

#[derive(Props, Clone, PartialEq)]
struct BaseProps {
    pub event: OnProps,
    pub style: StyleProps
}

#[component]
fn Base(props: BaseProps) -> Element {
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
            

        }
    }
}

#[allow(dead_code)]
fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(Event<T>) {
    move |e| {
        if let Some(listener) = maybe_listener {
            listener(e);
        }
    }
}