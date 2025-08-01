use super::*;

pub trait EditClass {
    /// Appends a CSS class string to the existing `class` attribute.
    fn with_class(self, class: &str) -> Self;
}

impl EditClass for AttrsProps {
    fn with_class(self, class: &str) -> Self {
        let old_class: &str = &self.class.to_owned().unwrap_or_default();
        let new_class: &str = class;
        let class: String = format!("{} {}", old_class, new_class);
        let class: Option<String> = Some(class);
        Self { class, ..self }
    }
}