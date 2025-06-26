use super::*;

pub fn use_interval<T>(hook: T, duration: Duration) 
where
    T: FnMut() + 'static {
    let hook: std::rc::Rc<_> = ::std::rc::Rc::new(::std::cell::RefCell::new(hook));
        
    use_effect(move || {
        use ::gloo_timers::callback as cb;
        let hook: ::std::rc::Rc<_> = hook.to_owned();
        let ms: u32 = duration.as_millis() as u32;
        cb::Interval::new(ms, move || {
            hook.borrow_mut()();
        });
    });
}