use super::*;

impl AttrsProps {
    pub fn add_class(self, #[allow(unused_variables)] class: &str) -> Self {
        Self {
            class: r#"
                {self.class.to_owned().unwrap_or_default()}
                {class}
            "#.into(),
            ..self
        }
    }
}