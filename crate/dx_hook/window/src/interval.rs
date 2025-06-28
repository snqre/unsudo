#[allow(unused_variables)]
pub fn use_interval<T>(hook: T, ms: u32)
where
    T: FnMut() + 'static {
    #[allow(unused_variables)]
    let hook: std::rc::Rc<_> = ::std::rc::Rc::new(::std::cell::RefCell::new(hook));
        
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use ::gloo_timers::callback as cb;

        let hook: ::std::rc::Rc<_> = hook.to_owned();
        let _ = cb::Interval::new(ms, move || {
            hook.borrow_mut()();
        });
    });
}