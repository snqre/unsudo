use super::*;

pub trait EditStyleExt<T> {
    fn with_style_before(self, style: &str) -> T;
    fn with_style(self, style: &str) -> T;
}

impl<T> EditStyleExt<T> for Option<T>
where
    T: Default,
    T: EditStyle {
    fn with_style_before(self, style: &str) -> T {
        self.unwrap_or_default().with_style_before(style)
    }

    fn with_style(self, style: &str) -> T {
        self.unwrap_or_default().with_style(style)
    }
}