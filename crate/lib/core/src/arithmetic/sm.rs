pub struct Degree<T>(pub T);
impl<T> core::ops::Deref for Degree<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for Degree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Radian<T>(pub T);
impl<T> core::ops::Deref for Radian<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for Radian<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Fixed<T>(pub T);
impl<T> core::ops::Deref for Fixed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for Fixed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Ratio<T>(pub T);
impl<T> core::ops::Deref for Ratio<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for Ratio<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}