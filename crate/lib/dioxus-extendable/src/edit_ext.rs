use super::*;

pub trait EditExt<T> {
    fn or_keep(self, edit: T) -> T;
    fn or_edit(self, edit: T) -> T;
}

impl<T> EditExt<T> for Option<T>
where
    T: Edit,
    T: Default {
    fn or_keep(self, edit: T) -> T {
        self.unwrap_or_default().take_or_keep(edit)
    }

    fn or_edit(self, edit: T) -> T {
        self.unwrap_or_default().take_or_else(edit)
    }
}