use super::*;

// as the map fills performance degrades,
// always allow for extra capacity.

macro_rules! map {
    ($($key:expr => $val:expr)*) => {{
        let mut m = Map::new();
        $(
            m.insert($key, $val).unwrap();
        )*
        m
    }};
}



pub trait Key 
where
    Self: core::fmt::Debug,
    Self: Default,
    Self: Copy,
    Self: Eq,
    Self: core::hash::Hash {}

impl<T> Key for T
where
    T: core::fmt::Debug,
    T: Default,
    T: Copy,
    T: Eq,
    T: core::hash::Hash {}



pub trait Val
where
    Self: core::fmt::Debug,
    Self: Default,
    Self: Copy {}

impl<T> Val for T
where
    T: core::fmt::Debug,
    T: Default,
    T: Copy {}



pub trait Hasher
where
    Self: Default,
    Self: core::hash::Hasher {}

impl<T> Hasher for T
where
    T: Default,
    T: core::hash::Hasher {}





pub struct Map<
    const A: usize, 
          B, 
          C, 
          D = core::hash::SipHasher> 
where
    B: Key,
    C: Val,
    D: Hasher {
    keys: [Option<B>; A],
    vals: [Option<C>; A],
    len: usize,
    hasher: core::marker::PhantomData<D>
}







impl<
    const A: usize, 
          B, 
          C, 
          D> Default for Map<A, B, C, D> 
where
    B: Key,
    C: Val,
    D: Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl<
    const A: usize, 
          B: Key, 
          C: Val, 
          D: Hasher> Map<A, B, C, D> {
    pub const fn new() -> Self {
        Self {
            keys: [None; A],
            vals: [None; A],
            len: 0,
            hasher: core::marker::PhantomData
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn cap(&self) -> usize {
        A
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, key: &B) -> Option<&C> {
        let mut position: usize = self.hash_index(key);
        for _ in 0..A {
            match &self.keys[position] {
                Some(k) if k == key => return self.vals[position].as_ref(),
                None => return None,
                _ => position = (position + 1) % A
            }
        }
        None
    }

    #[inline]
    pub fn insert(&mut self, key: B, data: C) -> Result<(), (B, C)> {
        if self.len >= A {
            return Err((key, data))
        }
        let mut position: usize = self.hash_index(&key);
        for _ in 0..A {
            match &self.keys[position] {
                Some(existing_key) if *existing_key == key => {
                    self.vals[position] = Some(data);
                    return Ok(())
                },
                None => {
                    self.keys[position] = Some(key);
                    self.vals[position] = Some(data);
                    self.len += 1;
                    return Ok(())
                },
                _ => position = (position + 1) % A
            }
        }
        Err((key, data))
    }

    #[inline]
    pub fn remove(&mut self, key: &B) -> Option<C> {
        let mut pos = self.hash_index(key);
        for _ in 0..A {
            match self.keys[pos] {
                Some(k) if k == *key => {
                    let old_val = self.vals[pos].take();
                    self.keys[pos] = None;
                    self.len -= 1;

                    // Reinsert subsequent cluster keys to avoid lookup holes
                    let mut next = (pos + 1) % A;
                    while let Some(k) = self.keys[next] {
                        let val = self.vals[next].take().unwrap();
                        self.keys[next] = None;
                        self.len -= 1;
                        self.insert(k, val).unwrap();
                        next = (next + 1) % A;
                    }
                    return old_val;
                }
                None => return None,
                _ => pos = (pos + 1) % A,
            }
        }
        None
    }

    fn hash_index(&self, key: &B) -> usize {
        let mut hasher: D = D::default();
        key.hash(&mut hasher);
        (
            hasher.finish() as usize
        ) % A
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_it() {
        let mut user_counts: Map<1000, utf8::Utf8<100>, u32> = Map::new();
        user_counts.insert(
            "strong_gamer".try_into().unwrap(), 
            300
        ).unwrap();
        user_counts.remove(
            &"strong_gamer".try_into().unwrap()
        ).unwrap();
        user_counts.insert(
            "new_user".try_into().unwrap(), 
            200
        ).unwrap();
        user_counts.len();
    }
}