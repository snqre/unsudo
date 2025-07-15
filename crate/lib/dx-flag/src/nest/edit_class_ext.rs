use super::*;

pub trait EditClassExt<T> {
    fn with_class(self, class: &str) -> T;
}

impl<T> EditClassExt<T> for Option<T>
where
    T: Default,
    T: EditClass {
    fn with_class(self, class: &str) -> T {
        self.unwrap_or_default().with_class(class)
    }
}