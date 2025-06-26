use super::*;

pub type AbsoluteOffsetControlSystem = (Signal<UseAbsoluteOffsetConfig>, semantic::Stylesheet<String>);

#[derive(Clone, PartialEq)]
pub struct UseAbsoluteOffsetConfig {
    pub l: Option<String>,
    pub r: Option<String>,
    pub t: Option<String>,
    pub b: Option<String>
}

pub fn use_absolute_offset(init_cfg: UseAbsoluteOffsetConfig) -> AbsoluteOffsetControlSystem {
    let cfg: Signal<_> = use_signal(|| init_cfg);

    let style: String = format!(
        r#"
            left: {},
            right: {},
            top: {},
            bottom: {}
        "#,
        cfg().l.unwrap_or(auto()),
        cfg().r.unwrap_or(auto()),
        cfg().t.unwrap_or(auto()),
        cfg().b.unwrap_or(auto())
    );

    (cfg, style)
}