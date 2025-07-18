use super::*;

modwire::expose!(
    combination
    engine
    mode
    var
);



pub struct Q<
    const A: u8, 
          B = u128, 
          C = DefaultMode, 
          D = DefaultEngine> 
where
    B: num::Int,
    C: Mode,
    D: Engine,
    (): Ok<A, B> {
    data: B,
    mode: core::marker::PhantomData<C>,
    engine: core::marker::PhantomData<D>
}

impl<
    const A: u8, 
          B: num::Int, 
          C: Mode, 
          D: Engine> core::fmt::Display for Q<A, B, C, D>
where
    (): precision::Ok<A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v: &B = &self.data;
        let v: &str = v.into();
        let c: [char; 3];
        for c in v.chars() {
            
        }
        Ok(())
    }
}

impl<'a, const A: u8, B, C, D> Q<'a, A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
        
    pub fn to_negative(self) -> Self {
        Q::new(D::to_negative(&self.data))
    }

    pub fn to_positive(self) -> Self {
        Q::new(D::to_positive(&self.data))
    }
}

impl<'a, const A: u8, B, C, D> Eq for Q<'a, A, B, C, D> 
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {}

impl<'a, const A: u8, B, C, D> PartialEq for Q<'a, A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'a, const A: u8, B, C, D> core::ops::Add for Q<'a, A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: B = self.data;
        let y: B = rhs.data;
        Ok(Q::new(D::add(&x, &y)?))
    }
}




impl<const A: u8, B, C, D> float::Float for Q<A, B, C, D> 
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): Ok<A, B> {
    fn powf(self, exp: Self) -> Self {
        
    }

    fn powi(self, exp: i32) -> Self {
        
    }
}



