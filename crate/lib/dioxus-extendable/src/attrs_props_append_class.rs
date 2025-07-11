use super::*;

impl AttrsProps {
    pub fn append_class(self, class: &str) -> Self {
        Self {
            class: r#"
                {self.class.to_owned().unwrap_or_default()}
                {class}
            "#.into(),
            ..self
        }
    }
}