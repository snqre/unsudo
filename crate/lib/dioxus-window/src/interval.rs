use super::*;
use std::sync;

pub fn use_interval<T>(hook: T, ms: u32)
where
    T: FnMut() + 'static {
    let hook: sync::Rc<_> = sync::Rc::new(sync::RefCell::new(hook));
        
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use ::gloo_timers::callback as cb;

        let hook: ::std::rc::Rc<_> = hook.to_owned();
        let _ = cb::Interval::new(ms, move || {
            hook.borrow_mut()();
        });
    });
}



fn t() {
    use_interval(|| {

    }, 2000);
}