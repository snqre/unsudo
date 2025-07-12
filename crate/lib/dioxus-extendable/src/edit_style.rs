use super::*;

pub trait EditStyle {
    fn with_style_before(self, style: &str) -> Self;
    fn with_style(self, style: &str) -> Self;
}

impl EditStyle for AttrsProps {
    fn with_style_before(self, style: &str) -> Self {
        let old_style: &str = &self.style.to_owned().unwrap_or_default();
        let new_style: &str = style;
        let style: OptionalAttributeOpcode = format!("{} {}", new_style, old_style).into();
        Self { style, ..self }
    }

    fn with_style(self, style: &str) -> Self {
        let old_style: &str = &self.style.to_owned().unwrap_or_default();
        let new_style: &str = style;
        let style: OptionalAttributeOpcode = format!("{} {}", old_style, new_style).into();
        Self { style, ..self }
    }
}