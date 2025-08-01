

use core::hash;
use core::iter;
use core::slice;
use core::marker;
use core::str;
use core::ops;

pub struct VecIter<'a, const A: usize, B> {
    vec: &'a Vec<A, B>,
    key: usize
}

impl<'a, const A: usize, B> Iterator for VecIter<'a, A, B> {
    type Item = &'a B;

    fn next(&mut self) -> Option<Self::Item> {
        if self.key < self.vec.len() {
            let item = self.vec.get(self.key);
            self.key += 1;
            item
        } else {
            None
        }
    }
}

impl<'a, const A: usize, B> iter::IntoIterator for &'a Vec<A, B> {
    type Item = &'a B;
    type IntoIter = VecIter<'a, A, B>;

    fn into_iter(self) -> Self::IntoIter {
        VecIter {
            vec: self,
            key: 0
        }
    }
}


// ===

pub struct VecIterMut<'a, const A: usize, B> {
    ptr: *mut B,
    len: usize,
    key: usize,
    m_0: marker::PhantomData<&'a mut B>
}

impl<'a, const A: usize, B> Iterator for VecIterMut<'a, A, B> {
    type Item = &'a mut B;

    fn next(&mut self) -> Option<Self::Item> {
        if self.key < self.len {
            let item = unsafe {
                &mut *self.ptr.add(self.key)
            };
            self.key += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, const A: usize, B> iter::IntoIterator for &'a mut Vec<A, B> {
    type Item = &'a mut B;
    type IntoIter = VecIterMut<'a, A, B>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        let ptr = self.as_mut_slice().as_mut_ptr();
        VecIterMut {
            ptr,
            len,
            key: 0,
            m_0: marker::PhantomData
        }
    }
}


// ===

pub struct VecIntoIter<const A: usize, B> {
    vec: Vec<A, B>,
    key: usize
}

impl<const A: usize, B> Iterator for VecIntoIter<A, B> {
    type Item = B;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.key < self.vec.len() {
            self.key += 1;
            self.vec.pop()
        } else {
            None
        }
    }
}

impl<const A: usize, B> iter::IntoIterator for Vec<A, B> {
    type Item = B;
    type IntoIter = VecIntoIter<A, B>;

    fn into_iter(self) -> Self::IntoIter {
        VecIntoIter {
            vec: self,
            key: 0
        }
    }
}


// ===

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[repr(transparent)]
pub struct Vec<const A: usize, B>(heapless::Vec<B, A>);

impl<const A: usize, B> Vec<A, B> {

    /// Returns the maximum number of elements the vector can hold.
    pub const fn cap(&self) -> usize {
        A
    }

    /// Returns the number of elements in the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = [1, 2, 3];
    /// assert_eq!(a.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the vec is full
    #[inline]
    pub fn is_out_of_space(&self) -> bool {
        self.0.is_full()
    }

    /// Returns true if the vec is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const A: usize, B> Vec<A, B> {
    #[inline]
    pub fn push(&mut self, v: B) -> Result<()> {
        self.0.push(v).map_err(|_| Error::Overflow)?;
        Ok(())
    }

    /// Removes the last element from a vector and returns it, or `None` if it's empty.
    #[inline]
    pub fn pop(&mut self) -> Option<B> {
        self.0.pop()
    }

    /// Clears the vector, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Shortens the vector, keeping the first `len` elements and dropping the rest.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }
}

impl<const A: usize, B> Vec<A, B> {

    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert_eq!(Some(&40), v.get(1));
    /// assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    /// assert_eq!(None, v.get(3));
    /// assert_eq!(None, v.get(0..4));
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn get(&self, k: usize) -> Option<&B> {
        self.0.get(k)
    }

    /// Returns a mutable reference to an element or subslice depending on the
    /// type of index (see [`get`]) or `None` if the index is out of bounds.
    ///
    /// [`get`]: slice::get
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(elem) = x.get_mut(1) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(x, &[0, 42, 2]);
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn get_mut(&mut self, k: usize) -> Option<&mut B> {
        self.0.get_mut(k)
    }
}

impl<const A: usize, B> Vec<A, B> {

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    /// let buffer: Vec<u8, 5> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
    /// assert_eq!(buffer.as_slice(), &[1, 2, 3, 5, 8]);
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[B] {
        self.0.as_slice()
    }

    /// Extracts a mutable slice containing the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    /// let mut buffer: Vec<u8, 5> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
    /// buffer[0] = 9;
    /// assert_eq!(buffer.as_slice(), &[9, 2, 3, 5, 8]);
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [B] {
        self.0.as_mut_slice()
    }
}

impl<const A: usize, B> Vec<A, B> {

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[1, 2, 4];
    /// let mut iterator = x.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    #[track_caller]
    pub fn iter(&self) -> impl Iterator<Item = &B> {
        self.into_iter()
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [1, 2, 4];
    /// for elem in x.iter_mut() {
    ///     *elem += 2;
    /// }
    /// assert_eq!(x, &[3, 4, 6]);
    /// ```
    #[inline]
    #[track_caller]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut B> {
        self.into_iter()
    }

    /// Creates an iterator from a value.
    ///
    /// See the [module-level documentation] for more.
    ///
    /// [module-level documentation]: crate::iter
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [1, 2, 3];
    /// let mut iter = v.into_iter();
    ///
    /// assert_eq!(Some(1), iter.next());
    /// assert_eq!(Some(2), iter.next());
    /// assert_eq!(Some(3), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    #[inline]
    #[track_caller]
    pub fn into_iter(self) -> impl Iterator<Item = B> {
        <Self as iter::IntoIterator>::into_iter(self)
    }
}

impl<const A: usize, B> Vec<A, B> {

    /// An iterator over slice in (non-overlapping) chunks separated by a predicate.
    ///
    /// This struct is created by the [`chunk_by`] method on [slices].
    ///
    /// [`chunk_by`]: slice::chunk_by
    /// [slices]: slice
    #[inline]
    #[track_caller]
    pub fn chunk_by<C>(&self, predicate: C) -> slice::ChunkBy<'_, B, C> 
    where
        C: FnMut(&B, &B) -> bool {
        self.0.chunk_by(predicate)
    }

    /// Returns an iterator over the slice producing non-overlapping mutable
    /// runs of elements using the predicate to separate them.
    ///
    /// The predicate is called for every pair of consecutive elements,
    /// meaning that it is called on `slice[0]` and `slice[1]`,
    /// followed by `slice[1]` and `slice[2]`, and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = &mut [1, 1, 1, 3, 3, 2, 2, 2];
    ///
    /// let mut iter = slice.chunk_by_mut(|a, b| a == b);
    ///
    /// assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
    /// assert_eq!(iter.next(), Some(&mut [3, 3][..]));
    /// assert_eq!(iter.next(), Some(&mut [2, 2, 2][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// This method can be used to extract the sorted subslices:
    ///
    /// ```
    /// let slice = &mut [1, 1, 2, 3, 2, 3, 2, 3, 4];
    ///
    /// let mut iter = slice.chunk_by_mut(|a, b| a <= b);
    ///
    /// assert_eq!(iter.next(), Some(&mut [1, 1, 2, 3][..]));
    /// assert_eq!(iter.next(), Some(&mut [2, 3][..]));
    /// assert_eq!(iter.next(), Some(&mut [2, 3, 4][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    #[track_caller]
    pub fn chunk_by_mut<C>(&mut self, predicate: C) -> slice::ChunkByMut<'_, B, C> 
    where
        C: FnMut(&B, &B) -> bool {
        self.0.chunk_by_mut(predicate)
    }

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the
    /// slice, then the last chunk will not have length `chunk_size`.
    ///
    /// See [`chunks_exact`] for a variant of this iterator that returns chunks of always exactly
    /// `chunk_size` elements, and [`rchunks`] for the same iterator but starting at the end of the
    /// slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.chunks(2);
    /// assert_eq!(iter.next().unwrap(), &['l', 'o']);
    /// assert_eq!(iter.next().unwrap(), &['r', 'e']);
    /// assert_eq!(iter.next().unwrap(), &['m']);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// [`chunks_exact`]: slice::chunks_exact
    /// [`rchunks`]: slice::rchunks
    #[inline]
    #[track_caller]
    pub fn chunks(&self, chunk_size: usize) -> slice::Chunks<'_, B> {
        self.0.chunks(chunk_size)
    }

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the
    /// slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved
    /// from the `remainder` function of the iterator.
    ///
    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
    /// resulting code better than in the case of [`chunks`].
    ///
    /// See [`chunks`] for a variant of this iterator that also returns the remainder as a smaller
    /// chunk, and [`rchunks_exact`] for the same iterator but starting at the end of the slice.
    ///
    /// # Error
    /// 
    /// `ZeroChunkSize` : `chunk_size` is `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.chunks_exact(2);
    /// assert_eq!(iter.next().unwrap(), &['l', 'o']);
    /// assert_eq!(iter.next().unwrap(), &['r', 'e']);
    /// assert!(iter.next().is_none());
    /// assert_eq!(iter.remainder(), &['m']);
    /// ```
    ///
    /// [`chunks`]: slice::chunks
    /// [`rchunks_exact`]: slice::rchunks_exact
    #[inline]
    #[track_caller]
    pub fn chunks_exact(&self, chunk_size: usize) -> Result<slice::ChunksExact<'_, B>> {
        if chunk_size == 0 {
            return Err(Error::ZeroChunkSize)
        }
        let chunks_exact = self.0.chunks_exact(chunk_size);
        Ok(chunks_exact)
    }

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the
    /// length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be
    /// retrieved from the `into_remainder` function of the iterator.
    ///
    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
    /// resulting code better than in the case of [`chunks_mut`].
    ///
    /// See [`chunks_mut`] for a variant of this iterator that also returns the remainder as a
    /// smaller chunk, and [`rchunks_exact_mut`] for the same iterator but starting at the end of
    /// the slice.
    ///
    /// # Error
    /// 
    /// `ZeroChunkSize` : `chunk_size` is `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = &mut [0, 0, 0, 0, 0];
    /// let mut count = 1;
    ///
    /// for chunk in v.chunks_exact_mut(2) {
    ///     for elem in chunk.iter_mut() {
    ///         *elem += count;
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(v, &[1, 1, 2, 2, 0]);
    /// ```
    ///
    /// [`chunks_mut`]: slice::chunks_mut
    /// [`rchunks_exact_mut`]: slice::rchunks_exact_mut
    #[inline]
    #[track_caller]
    pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> Result<slice::ChunksExactMut<'_, B>> {
        if chunk_size == 0 {
            return Err(Error::ZeroChunkSize)
        }
        let out = self.0.chunks_exact_mut(chunk_size);
        Ok(out)
    }

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the
    /// length of the slice, then the last chunk will not have length `chunk_size`.
    ///
    /// See [`chunks_exact_mut`] for a variant of this iterator that returns chunks of always
    /// exactly `chunk_size` elements, and [`rchunks_mut`] for the same iterator but starting at
    /// the end of the slice.
    ///
    /// # Error
    /// 
    /// `ZeroChunkSize` : `chunk_size` is `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = &mut [0, 0, 0, 0, 0];
    /// let mut count = 1;
    ///
    /// for chunk in v.chunks_mut(2) {
    ///     for elem in chunk.iter_mut() {
    ///         *elem += count;
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(v, &[1, 1, 2, 2, 3]);
    /// ```
    ///
    /// [`chunks_exact_mut`]: slice::chunks_exact_mut
    /// [`rchunks_mut`]: slice::rchunks_mut
    #[inline]
    #[track_caller]
    pub fn chunks_mut(&mut self, chunk_size: usize) -> Result<slice::ChunksMut<'_, B>> {
        if chunk_size == 0 {
            return Err(Error::ZeroChunkSize)
        }
        let out = self.0.chunks_mut(chunk_size);
        Ok(out)
    }

    /// Returns an array reference to the last `N` items in the slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let u = [10, 40, 30];
    /// assert_eq!(Some(&[40, 30]), u.last_chunk::<2>());
    ///
    /// let v: &[i32] = &[10];
    /// assert_eq!(None, v.last_chunk::<2>());
    ///
    /// let w: &[i32] = &[];
    /// assert_eq!(Some(&[]), w.last_chunk::<0>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn last_chunk(&self) -> Option<&[B; A]> {
        self.0.last_chunk()
    }

    /// Returns a mutable array reference to the last `N` items in the slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(last) = x.last_chunk_mut::<2>() {
    ///     last[0] = 10;
    ///     last[1] = 20;
    /// }
    /// assert_eq!(x, &[0, 10, 20]);
    ///
    /// assert_eq!(None, x.last_chunk_mut::<4>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn last_chunk_mut(&mut self) -> Option<&mut [B; A]> {
        self.0.last_chunk_mut()
    }

    /// Returns an array reference to the first `N` items in the slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let u = [10, 40, 30];
    /// assert_eq!(Some(&[10, 40]), u.first_chunk::<2>());
    ///
    /// let v: &[i32] = &[10];
    /// assert_eq!(None, v.first_chunk::<2>());
    ///
    /// let w: &[i32] = &[];
    /// assert_eq!(Some(&[]), w.first_chunk::<0>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn first_chunk(&self) -> Option<&[B; A]> {
        self.0.first_chunk()
    }

    /// Returns a mutable array reference to the first `N` items in the slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(first) = x.first_chunk_mut::<2>() {
    ///     first[0] = 5;
    ///     first[1] = 4;
    /// }
    /// assert_eq!(x, &[5, 4, 2]);
    ///
    /// assert_eq!(None, x.first_chunk_mut::<4>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn first_chunk_mut(&mut self) -> Option<&mut [B; A]> {
        self.0.first_chunk_mut()
    }

    /// Returns an array reference to the last `N` items in the slice and the remaining slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[0, 1, 2];
    ///
    /// if let Some((elements, last)) = x.split_last_chunk::<2>() {
    ///     assert_eq!(elements, &[0]);
    ///     assert_eq!(last, &[1, 2]);
    /// }
    ///
    /// assert_eq!(None, x.split_last_chunk::<4>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn split_last_chunk(&self) -> Option<(&[B], &[B; A])> {
        self.0.split_last_chunk()
    }

    /// Returns a mutable array reference to the last `N` items in the slice and the remaining
    /// slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some((elements, last)) = x.split_last_chunk_mut::<2>() {
    ///     last[0] = 3;
    ///     last[1] = 4;
    ///     elements[0] = 5;
    /// }
    /// assert_eq!(x, &[5, 3, 4]);
    ///
    /// assert_eq!(None, x.split_last_chunk_mut::<4>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn split_last_chunk_mut(&mut self) -> Option<(&mut [B], &mut [B; A])>{
        self.0.split_last_chunk_mut()
    }

    /// Returns an array reference to the first `N` items in the slice and the remaining slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[0, 1, 2];
    ///
    /// if let Some((first, elements)) = x.split_first_chunk::<2>() {
    ///     assert_eq!(first, &[0, 1]);
    ///     assert_eq!(elements, &[2]);
    /// }
    ///
    /// assert_eq!(None, x.split_first_chunk::<4>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn split_first_chunk(&self) -> Option<(&[B; A], &[B])> {
        self.0.split_first_chunk()
    }

    /// Returns a mutable array reference to the first `N` items in the slice and the remaining
    /// slice.
    ///
    /// If the slice is not at least `N` in length, this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some((first, elements)) = x.split_first_chunk_mut::<2>() {
    ///     first[0] = 3;
    ///     first[1] = 4;
    ///     elements[0] = 5;
    /// }
    /// assert_eq!(x, &[3, 4, 5]);
    ///
    /// assert_eq!(None, x.split_first_chunk_mut::<4>());
    /// ```
    #[inline]
    #[track_caller]
    pub fn split_first_chunk_mut(&mut self) -> Option<(&mut [B; A], &mut [B])> {
        self.0.split_first_chunk_mut()
    }
}



impl<const A: usize, B> From<Vec<A, B>> for heapless::Vec<B, A> {
    fn from(value: Vec<A, B>) -> Self {
        value.0
    }
}

impl<const A: usize, B> From<heapless::Vec<B, A>> for Vec<A, B> {
    fn from(value: heapless::Vec<B, A>) -> Self {
        Self(value)
    }
}


impl<const A: usize, B> iter::FromIterator<B> for Vec<A, B> {
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let mut vec = heapless::Vec::<B, A>::new();
        for item in iter {
            if vec.push(item).is_err() {
                break
            }
        }
        Self(vec)
    }
}

#[cfg(feature = "std")]
impl<const A: usize, B> TryFrom<std::vec::Vec<B>> for Vec<A, B> {
    type Error = Error;

    fn try_from(value: std::vec::Vec<B>) -> Result<Self> {
        let mut vec = heapless::Vec::<B, A>::new();
        for item in value {
            vec.push(item).map_err(|_| Error::Overflow)?;
        }
        Ok(Self(vec))
    }
}

// VecString
// ===

impl<const A: usize, const B: usize> Vec<A, String<B>> {

}




// === === ===

pub struct MapIter {}

pub struct MapIterMut {}

#[derive(Clone)]
#[derive(Default)]
#[repr(transparent)]
pub struct Map<const A: usize, B, C>(heapless::FnvIndexMap<B, C, A>)
where
    B: hash::Hash,
    B: Eq;

impl<const A: usize, B, C> Map<A, B, C>
where
    B: hash::Hash,
    B: Eq, {
    pub fn get(&self, k: &B) {
        self.0.get(k);
    }

    pub fn get_mut(&mut self, k: &B) {
        self.0.get_mut(k);
    }
}

impl<const A: usize, B, C> Map<A, B, C>
where
    B: hash::Hash,
    B: Eq, {

    /// Inserts a key-value pair into the map.
    ///
    /// If an equivalent key already exists in the map: the key remains and retains in its place in
    /// the order, its corresponding value is updated with `value` and the older value is returned
    /// inside `Some(_)`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is inserted, last in order,
    /// and `None` is returned.
    ///
    /// Computes in **O(1)** time (average).
    ///
    /// See also entry if you you want to insert or modify or if you need to get the index of the
    /// corresponding key-value pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::FnvIndexMap;
    ///
    /// let mut map = FnvIndexMap::<_, _, 8>::new();
    /// assert_eq!(map.insert(37, "a"), Ok(None));
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Ok(Some("b")));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, k: B, v: C) -> Result<Option<()>> {
        self.0.insert(k, v).map_err(|_| Error::Overflow)?;
        Ok(Some(()))
    }

    /// Same as [`swap_remove`](Self::swap_remove)
    ///
    /// Computes in **O(1)** time (average).
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::FnvIndexMap;
    ///
    /// let mut map = FnvIndexMap::<_, _, 8>::new();
    /// map.insert(1, "a").unwrap();
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove(&mut self, key: &B) -> Option<C> {
        self.0.remove(key)
    }
}

impl<const A: usize, B, C> Map<A, B, C> 
where
    B: hash::Hash,
    B: Eq, {

    pub fn iter(&self) -> heapless::IndexMapIter<'_, B, C> {
        self.0.iter()
    }
}


