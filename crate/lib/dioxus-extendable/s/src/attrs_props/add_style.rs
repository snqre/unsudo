use super::*;

impl AttrsProps {
    pub fn add_style_before(self, #[allow(unused_variables)] style: &str) -> Self {
        Self {
            style: r#"
                {style}
                {self.style.to_owned().unwrap_or_default()}
            "#.into(),
            ..self
        }
    }

    pub fn add_style(self, #[allow(unused_variables)] style: &str) -> Self {
        Self {
            style: r#"
                {self.style.to_owned().unwrap_or_default()}
                {style}
            "#.into(),
            ..self
        }
    }
}