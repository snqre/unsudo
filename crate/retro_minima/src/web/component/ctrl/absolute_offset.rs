use super::*;

pub type AbsoluteOffsetControlSystem<'a> = (Signal<AbsoluteOffsetConfiguration<'a>>, Stylesheet);

#[derive(Clone, PartialEq)]
pub struct AbsoluteOffsetConfiguration<'a> {
    pub l: Option<&'a str>,
    pub r: Option<&'a str>,
    pub t: Option<&'a str>,
    pub b: Option<&'a str>
}

pub fn use_absolute_offset(initial_configuration: AbsoluteOffsetConfiguration) -> AbsoluteOffsetControlSystem {
    let cfg: Signal<_> = use_signal(|| initial_configuration);

    let style: String = format!(
        r#"
            left: {},
            right: {},
            top: {},
            bottom: {}
        "#,
        cfg().l.unwrap_or(AUTO),
        cfg().r.unwrap_or(AUTO),
        cfg().t.unwrap_or(AUTO),
        cfg().b.unwrap_or(AUTO)
    );

    (cfg, style)
}