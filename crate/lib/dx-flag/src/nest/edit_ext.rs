use super::*;

pub trait EditExt<T> {
    fn keep(self, edit: T) -> T;
    fn edit(self, edit: T) -> T;
}

impl<T> EditExt<T> for Option<T>
where
    T: Edit,
    T: Default {
    fn keep(self, edit: T) -> T {
        self.unwrap_or_default().keep(edit)
    }

    fn edit(self, edit: T) -> T {
        self.unwrap_or_default().edit(edit)
    }
}