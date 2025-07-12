use super::*;

pub trait EditStyle {
    fn add_style_before(self, style: &str) -> Self;
    fn add_style(self, style: &str) -> Self;
}