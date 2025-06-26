use super::*;

pub type OffsetControlSystem<'a> = (Signal<OffsetConfiguration<'a>>, Stylesheet);

#[derive(Clone, PartialEq)]
pub struct OffsetConfiguration<'a> {
    pub x: &'a str,
    pub y: &'a str
}

pub fn use_offset(init_cfg: OffsetConfiguration) -> OffsetControlSystem {
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