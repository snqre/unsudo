use super::*;

pub struct Ratio<T>(T);
impl<T> ops::Deref for Ratio<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> ops::DerefMut for Ratio<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}