use super::*;

pub type OffsetControlSystem = (Signal<UseOffsetConfig>, semantic::Stylesheet<String>);

#[derive(Clone, PartialEq)]
pub struct UseOffsetConfig {
    pub x: String,
    pub y: String
}

pub fn use_offset(init_cfg: UseOffsetConfig) -> OffsetControlSystem {
    let cfg: Signal<_> = use_signal(|| init_cfg);

    let style: String = format!(
        r#"
            transform: translate({}, {});
        "#,
        cfg().x,
        cfg().y
    );

    (cfg, style)
}