use super::*;

pub type SizeControlSystem<'a> = (Signal<SizeConfiguration<'a>>, Stylesheet);

#[derive(Clone, PartialEq)]
pub struct SizeConfiguration<'a> {
    pub w: &'a str,
    pub h: &'a str,
    pub min_w: Option<&'a str>,
    pub max_w: Option<&'a str>,
    pub min_h: Option<&'a str>,
    pub max_h: Option<&'a str>,
}

pub fn use_size(initial_configuration: SizeConfiguration) -> SizeControlSystem {
    let cfg: Signal<_> = use_signal(|| initial_configuration);
    
    let style: String = format!(
        r#"
            width: {};
            min-width: {};
            max-width: {};
            height: {};
            min-height: {};
            max-height: {};
        "#,
        cfg().w,
        cfg().min_w.unwrap_or(AUTO),
        cfg().max_w.unwrap_or(AUTO),
        cfg().h,
        cfg().min_h.unwrap_or(AUTO),
        cfg().max_h.unwrap_or(AUTO)
    );

    (cfg, style)
}