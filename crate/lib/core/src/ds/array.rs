use crate::require;

#[macro_export]
macro_rules! array {
    ($($data:expr),* $(,)?) => {{
        let mut arr = Array::<{count!($($data),*)}, _>::default();
        $(
            arr.push($data).expect("Exceeded fixed capacity.");
        )*
        arr
    }};
}

#[allow(unused_macros)]
macro_rules! count {
    () => { 0 };
    ($head:expr $(, $tail:expr)*) => {
        1 + count!($($tail),*)
    };
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Array<const A: usize, B> 
where 
    B: Copy {
    pub(super) buf: [core::mem::MaybeUninit<B>; A],
    pub(super) len: usize
}

impl<const A: usize, B> Default for Array<A, B> 
where 
    B: Copy {
    fn default() -> Self {
        Self {
            buf: unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            },
            len: 0
        }
    }
}

impl<const A: usize, B> Array<A, B> 
where
    B: Copy {
    #[inline]
    pub fn new(data: [B; A]) -> Self {
        let mut buf: [core::mem::MaybeUninit<B>; A] = unsafe {
            core::mem::MaybeUninit::uninit().assume_init()
        };
        for (k, data) in data.into_iter().enumerate() {
            buf[k].write(data);
        }
        Self {
            buf,
            len: A
        }
    }

    pub const fn get(&self, key: usize) -> Option<&B> {
        require!(key < self.len => None);
        Some(unsafe {
            &*self.buf[key].as_ptr()
        })
    }

    pub const fn get_mut(&mut self, k: usize) -> Option<&mut B> {
        require!(k < self.len => None);
        Some(unsafe {
            &mut *self.buf[k].as_mut_ptr()
        })
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

    pub const fn push(&mut self, data: B) -> Result<(), B> {
        require!(self.len < A => Err(data));
        self.buf[self.len].write(data);
        self.len += 1;
        Ok(())
    }

    pub const fn pop(&mut self) -> Option<B> {
        require!(!self.is_empty() => None);
        self.len -= 1;
        Some(unsafe {
            self.buf[self.len].assume_init_read()
        })
    }

    pub const fn swap_insert(&mut self, key: usize, data: B) -> Option<()> {
        require!(self.len < A => None);
        require!(key <= self.len => None);
        self.buf[self.len].write(data);
        self.len += 1;
        if key != self.len - 1 {
            unsafe {
                let tmp = self.buf[key].assume_init_read();
                self.buf[key] = self.buf[self.len - 1];
                self.buf[self.len - 1].write(tmp);
            }
        }
        Some(())
    }

    pub const fn swap_remove(&mut self, k: usize) -> Option<B> {
        require!(k < self.len => None);
        let ret = unsafe {
            self.buf[k].assume_init_read()
        };
        if k != self.len - 1 {
            self.buf[k] = self.buf[self.len - 1];
        }
        self.len -= 1;
        Some(ret)
    }

    #[inline]
    pub fn insert(&mut self, key: usize, data: B) -> Option<()> {
        require!(self.len < A => None);
        require!(key <= self.len => None);
        for o in (key..self.len).rev() {
            self.buf[o + 1] = self.buf[o];
        }
        self.buf[key].write(data);
        self.len += 1;
        Some(())
    }

    #[inline]
    pub fn remove(&mut self, key: usize) -> Option<B> {
        require!(key < self.len => None);
        let data = unsafe {
            self.buf[key].assume_init_read()
        };
        for o in key..self.len - 1 {
            self.buf[o] = self.buf[o + 1];
        }
        self.len -= 1;
        Some(data)
    }

    pub const fn as_slice(&self) -> &[B] {
        let data: *const B = self.buf.as_ptr() as *const B;
        unsafe {
            core::slice::from_raw_parts(data, self.len)
        }
    }

    pub const fn as_mut_slice(&mut self) -> &mut [B] {
        let data: *mut B = self.buf.as_mut_ptr() as *mut B;
        unsafe {
            core::slice::from_raw_parts_mut(data, self.len)
        }
    }
}

impl<const A: usize, B> Eq for Array<A, B> 
where
    B: Copy,
    B: PartialEq {}

impl<const A: usize, B> PartialEq for Array<A, B> 
where
    B: Copy,
    B: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.as_slice() == other.as_slice()
    }
}

impl<const A: usize, B> FromIterator<B> for Array<A, B>
where
    B: Copy {

    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut arr: Self = Self::default();
        for data in iter {
            if arr.push(data).is_err() {
                break
            }
        }
        arr
    }
}

impl<const A: usize, B> IntoIterator for Array<A, B>
where
    B: Copy {
    type Item = B;
    type IntoIter = Iter<A, B>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            buf: self.buf,
            len: self.len,
            k: 0
        }
    }
}

pub struct Iter<const A: usize, B> {
    buf: [core::mem::MaybeUninit<B>; A],
    len: usize,
    k: usize
}

impl<const A: usize, B> ExactSizeIterator for Iter<A, B> 
where
    B: Copy {}

impl<const A: usize, B> Iterator for Iter<A, B>
where
    B: Copy {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        require!(self.k >= self.len => None);
        let data = unsafe {
            self.buf[self.k].assume_init_read()
        };
        self.k += 1;
        Some(data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left: usize = self.len - self.k;
        (left, Some(left))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn push_pop() {
        let mut arr: Array<4, u8> = Array::default();
        assert_eq!(arr.len(), 0);
        arr.push(1).unwrap();
        arr.push(2).unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), None);
    }

    #[test]
    fn insert_remove_ordered() {
        let mut arr: Array<4, u8> = Array::default();
        arr.push(1).unwrap();
        arr.push(3).unwrap();
        arr.insert(1, 2).unwrap(); // insert in middle
        assert_eq!(arr.as_slice(), &[1, 2, 3]);

        let val = arr.remove(1).unwrap(); // remove middle
        assert_eq!(val, 2);
        assert_eq!(arr.as_slice(), &[1, 3]);
    }
    
    #[test]
    fn swap_insert_remove_unordered() {
        let mut arr: Array<4, u8> = Array::default();
        arr.push(10).unwrap();
        arr.push(20).unwrap();
        arr.push(30).unwrap();

        // Insert 15 at index 1, expect ordering not preserved
        arr.swap_insert(1, 15).unwrap();
        assert_eq!(arr.len(), 4);
        assert!(arr.as_slice().contains(&15));

        let val = arr.swap_remove(1).unwrap();
        assert!(val == 15 || val == 20); // due to swap, either could be returned
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn iter_works() {
        let mut arr: Array<3, u8> = Array::default();
        arr.push(1).unwrap();
        arr.push(2).unwrap();
        arr.push(3).unwrap();

        let collected: Array<3, u8> = arr.into_iter().collect();
        assert_eq!(collected, array![1, 2, 3]);
    }
}