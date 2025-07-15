pub struct Coordinate<T> 
where
    T: Copy,
    T: num::Num {
    pub x: T,
    pub y: T
}

impl<T> Coordinate<T> 
where
    T: Copy,
    T: num::Num {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T> Default for Coordinate<T> 
where
    T: Copy,
    T: num::Num {
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
}