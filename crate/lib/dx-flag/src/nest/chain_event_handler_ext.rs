use super::*;

pub trait ChainEventHandlerExt<T> {
    fn on(self, edit: T) -> T;
}

impl<T> ChainEventHandlerExt<T> for Option<T>
where
    T: Default,
    T: ChainEventHandler {
    fn on(self, edit: T) -> T {
        self.unwrap_or_default().on(edit)
    }
}