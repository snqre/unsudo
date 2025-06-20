use ::core::str;

use super::*;

pub struct Utf8<const T: usize> {
    buf: [u8; T],
    len: usize
}

impl<const T: usize> Utf8<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            buf: [0; T],
            len: 0
        }
    }

    pub const fn cap(&self) -> usize {
        T
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn chars(&self) -> Result<core::str::Chars> {
        let s = self.as_str().chars();
        Ok(s)
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<u8> {
        self.as_bytes().iter()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        let bytes = self.as_bytes();
        unsafe {
            core::str::from_utf8_unchecked(bytes)
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn push(&mut self, c: char) -> Result<()> {
        let mut buf = [0 as u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();
        let utf8 = core::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)?;
        self.push_str(utf8)?;
        Ok(())
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > T {
            return Err(Error::Overflow)
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    #[inline]
    pub fn pop(&mut self) -> Option<()> {
        if self.len == 0 {
            return None
        }
        let mut k = self.len - 1;
        while k > 0 && (self.buf[k] & 0b11000000) == 0b10000000 {
            k -= 1;
        }
        self.len = k;
        Some(())
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) -> Result<()> {
        if new_len > self.len {
            return Err(Error::Overflow)
        }
        let s = core::str::from_utf8(&self.buf[..new_len]).map_err(|_| Error::InvalidUtf8)?;
        self.len = s.len();
        Ok(())
    }
}

impl<const T: usize> Eq for Utf8<T> {}

impl<const T: usize> PartialEq for Utf8<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<const T: usize> Ord for Utf8<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let x = self.as_bytes();
        let y = other.as_bytes();
        x.cmp(y)
    }
}

impl <const T: usize> PartialOrd for Utf8<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let x = self.as_bytes();
        let y = other.as_bytes();
        let out = x.cmp(y);
        Some(out)
    }
}

impl<const T: usize> Clone for Utf8<T> {
    #[inline]
    fn clone(&self) -> Self {
        let mut new = Self::new();
        new.buf[..self.len].copy_from_slice(&self.buf[..self.len]);
        new.len = self.len;
        new
    }
}

impl<const T: usize> fmt::Debug for Utf8<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        write!(f, "{:?}", bytes)
    }
}

impl<const T: usize> TryFrom<&str> for Utf8<T> {
    type Error = Error;

    #[inline]
    fn try_from(value: &str) -> result::Result<Self, Self::Error> {
        let mut s = Self::new();
        s.push_str(value)?;
        Ok(s)
    }
}

#[cfg(feature = "std")]
impl<const T: usize> TryFrom<std::string::String> for Utf8<T> {
    type Error = Error;

    #[inline]
    fn try_from(value: std::string::String) -> result::Result<Self, Self::Error> {
        let mut s = Self::new();
        let value = value.as_str();
        s.push(value)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push() {
        let mut s: Utf8<3> = Utf8::new();
        s.push('a').unwrap();
        s.push('b').unwrap();
        s.push('c').unwrap();
        let s_match: Utf8<3> = "abc".try_into().unwrap();
        assert_eq!(s, s_match);
    }

    #[test]
    fn push_str() {
        let mut s: Utf8<11> = Utf8::new();
        s.push_str("Hello").unwrap();
        s.push_str(" ").unwrap();
        s.push_str("World").unwrap();
        let s_match: Utf8<11> = "Hello World".try_into().unwrap();
        assert_eq!(s, s_match);
    }

    #[test]
    fn truncate() {
        let mut s: Utf8<5> = "Hello".try_into().unwrap();
        s.truncate(4).unwrap();
        let s_match: Utf8<5> = "Hell".try_into().unwrap();
        assert_eq!(s, s_match);
    }

    #[test]
    fn pop() {
        let mut s: Utf8<5> = "Foo".try_into().unwrap();
        s.pop();
        s.pop();
        let s_ok: Utf8<5> = "F".try_into().unwrap();
        assert_eq!(s, s_ok);
    }

    #[test]
    fn clear() {
        let mut s: Utf8<5> = Utf8::new();
        s.push_str("Hello").unwrap();
        s.clear();
        let s_match: Utf8<5> = "".try_into().unwrap();
        assert_eq!(s, s_match);
    }

    #[test]
    fn clone() {
        let x: Utf8<5> = "Hello".try_into().unwrap();
        let y: Utf8<5> = x.clone();
        let s_ok: Utf8<5> = "Hello".try_into().unwrap();
        assert_eq!(x, y);
        assert_eq!(y, s_ok);
    }
}