use super::*;

pub type SizeControlSystem = (Signal<UseSizeConfig>, semantic::Stylesheet<String>);

#[derive(Clone, PartialEq)]
pub struct UseSizeConfig {
    pub w: String,
    pub h: String,
    pub min_w: Option<String>,
    pub max_w: Option<String>,
    pub min_h: Option<String>,
    pub max_h: Option<String>,
}

pub fn use_size(init_cfg: UseSizeConfig) -> SizeControlSystem {
    let cfg: Signal<_> = use_signal(|| init_cfg);
    
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
        cfg().min_w.unwrap_or(auto()),
        cfg().max_w.unwrap_or(auto()),
        cfg().h,
        cfg().min_h.unwrap_or(auto()),
        cfg().max_h.unwrap_or(auto())
    );

    (cfg, style)
}