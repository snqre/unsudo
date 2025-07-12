use super::*;

pub trait EditStyleExt<T> {
    fn add_style_before(self, style: &str) -> Self;
    fn add_style(self, style: &str) -> Self;
}

impl<T> EditStyleExt<T> for Option<T>
where
    T: Default,
    T: EditStyle {
    fn add_style_before(self, style: &str) -> T {
        self.unwrap_or_default().add_style_before(style)
    }

    fn add_style(self, style: &str) -> T {
        self.unwrap_or_default().add_style(style)
    }
}