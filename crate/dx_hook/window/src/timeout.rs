use ::gloo_timers::callback as cb;

pub fn use_timeout<T>(listener: T, ms: u32) -> cb::Timeout
where
    T: FnMut() + 'static {
    let mut listener = listener;
    cb::Timeout::new(ms, move || {
        listener();
    })
}


fn hello() {
    use_timeout(|| {
        
    }, 200);
}